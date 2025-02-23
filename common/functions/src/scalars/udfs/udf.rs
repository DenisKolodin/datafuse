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

use crate::scalars::function_factory::FunctionFactory;
use crate::scalars::udfs::exists::ExistsFunction;
use crate::scalars::CrashMeFunction;
use crate::scalars::DatabaseFunction;
use crate::scalars::SleepFunction;
use crate::scalars::ToTypeNameFunction;
use crate::scalars::UdfExampleFunction;
use crate::scalars::VersionFunction;

#[derive(Clone)]
pub struct UdfFunction;

impl UdfFunction {
    pub fn register(factory: &mut FunctionFactory) {
        factory.register("example", UdfExampleFunction::desc());
        factory.register("totypename", ToTypeNameFunction::desc());
        factory.register("database", DatabaseFunction::desc());
        factory.register("version", VersionFunction::desc());
        factory.register("sleep", SleepFunction::desc());
        factory.register("crashme", CrashMeFunction::desc());
        factory.register("exists", ExistsFunction::desc());
    }
}
