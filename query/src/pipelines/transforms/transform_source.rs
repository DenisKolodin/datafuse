// Copyright 2020 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::any::Any;
use std::sync::Arc;

use common_exception::ErrorCode;
use common_exception::Result;
use common_planners::ReadDataSourcePlan;
use common_streams::CorrectWithSchemaStream;
use common_streams::ProgressStream;
use common_streams::SendableDataBlockStream;
use common_tracing::tracing;

use crate::catalogs::Catalog;
use crate::pipelines::processors::EmptyProcessor;
use crate::pipelines::processors::Processor;
use crate::sessions::DatabendQueryContextRef;

pub struct SourceTransform {
    ctx: DatabendQueryContextRef,
    source_plan: ReadDataSourcePlan,
}

impl SourceTransform {
    pub fn try_create(
        ctx: DatabendQueryContextRef,
        source_plan: ReadDataSourcePlan,
    ) -> Result<Self> {
        Ok(SourceTransform { ctx, source_plan })
    }

    async fn read_table(&self) -> Result<SendableDataBlockStream> {
        let table = if self.source_plan.tbl_args.is_none() {
            let catalog = self.ctx.get_catalog();
            catalog.build_table(&self.source_plan.table_info)?
        } else {
            let func_meta = self.ctx.get_table_function(
                &self.source_plan.table_info.name,
                self.source_plan.tbl_args.clone(),
            )?;
            func_meta.as_table()
        };

        // TODO(xp): get_single_node_table_io_context() or
        //           get_cluster_table_io_context()?
        let io_ctx = Arc::new(self.ctx.get_cluster_table_io_context()?);
        let table_stream = table.read(io_ctx, &self.source_plan);
        let progress_stream =
            ProgressStream::try_create(table_stream.await?, self.ctx.progress_callback()?)?;

        Ok(Box::pin(
            self.ctx.try_create_abortable(Box::pin(progress_stream))?,
        ))
    }
}

#[async_trait::async_trait]
impl Processor for SourceTransform {
    fn name(&self) -> &str {
        "SourceTransform"
    }

    fn connect_to(&mut self, _: Arc<dyn Processor>) -> Result<()> {
        Result::Err(ErrorCode::LogicalError(
            "Cannot call SourceTransform connect_to",
        ))
    }

    fn inputs(&self) -> Vec<Arc<dyn Processor>> {
        vec![Arc::new(EmptyProcessor::create())]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    async fn execute(&self) -> Result<SendableDataBlockStream> {
        let db = self.source_plan.table_info.db.clone();
        let table = self.source_plan.table_info.name.clone();

        tracing::debug!("execute, table:{:#}.{:#} ...", db, table);

        // We need to keep the block struct with the schema
        // Because the table may not support require columns

        // TODO(xp): should use this but it fails the stateless test:
        //           Passing in only required fields needs to modify all components that use schema.
        // let schema = DataSchema::new_from(
        //     self.source_plan
        //         .scan_fields()
        //         .values()
        //         .cloned()
        //         .collect::<Vec<_>>(),
        //     self.source_plan.schema().meta().clone(),
        // );

        let schema = self.source_plan.table_info.schema.clone();
        Ok(Box::pin(CorrectWithSchemaStream::new(
            self.read_table().await?,
            schema,
        )))
    }
}
