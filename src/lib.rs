use grpc::{api::api_grpc_client::ApiGrpcClient, StopMode, TaskCheckpointEventType};

pub const GRPC_PORT_ENV: &str = "GRPC_PORT";

pub mod grpc {
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("arroyo_rpc");

    pub mod api {
        #![allow(clippy::derive_partial_eq_without_eq)]
        tonic::include_proto!("arroyo_api");
    }

    pub const API_FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("api_descriptor");
}
