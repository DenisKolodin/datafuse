// Copyright 2021 Datafuse Labs.
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

use std::net::SocketAddr;

use axum::handler::get;
use axum::routing::BoxRoute;
use axum::AddExtensionLayer;
use axum::Router;
use common_base::tokio;
use common_exception::Result;

use crate::common::service::HttpShutdownHandler;
use crate::servers::http::v1::statement::statement_router;
use crate::servers::Server;
use crate::sessions::SessionManagerRef;

pub struct HttpHandler {
    session_manager: SessionManagerRef,
    shutdown_handler: HttpShutdownHandler,
}

impl HttpHandler {
    pub fn create(session_manager: SessionManagerRef) -> Box<dyn Server> {
        Box::new(HttpHandler {
            session_manager,
            shutdown_handler: HttpShutdownHandler::create("HttpHandler".to_string()),
        })
    }
    fn build_router(&self) -> Router<BoxRoute> {
        Router::new()
            .route("/", get(|| async { "This is http handler." }))
            .nest("/v1/statement", statement_router())
            .layer(AddExtensionLayer::new(self.session_manager.clone()))
            .boxed()
    }
    async fn start_without_tls(&mut self, listening: SocketAddr) -> Result<SocketAddr> {
        let server = axum_server::bind(listening.to_string())
            .handle(self.shutdown_handler.abort_handle.clone())
            .serve(self.build_router());

        self.shutdown_handler.try_listen(tokio::spawn(server)).await
    }
}

#[async_trait::async_trait]
impl Server for HttpHandler {
    async fn shutdown(&mut self) {
        self.shutdown_handler.shutdown().await;
    }

    async fn start(&mut self, listening: SocketAddr) -> common_exception::Result<SocketAddr> {
        self.start_without_tls(listening).await
    }
}
