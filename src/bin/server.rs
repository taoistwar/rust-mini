use hyper::Body;
use rust_mini::grpc::controller_grpc_server::{ControllerGrpc, ControllerGrpcServer};
use rust_mini::grpc_port;
use std::net::SocketAddr;
use std::task::{Context, Poll};
use tonic::body::BoxBody;
use tonic::transport::Server;
use tower::layer::util::Stack;
use tower::{Layer, Service};
use tower_http::classify::{GrpcCode, GrpcErrorsAsFailures, SharedClassifier};
use tower_http::trace::{DefaultOnFailure, TraceLayer};
use tracing::{info, span, warn, Level};

pub const CONTROLLER_GRPC: u16 = 9190;

#[derive(Clone)]
pub struct ControllerServer {}

#[tonic::async_trait]
impl ControllerGrpc for ControllerServer {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    OK(())
}
