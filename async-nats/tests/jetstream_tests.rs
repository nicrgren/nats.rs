use serde::{Deserialize, Serialize};
use serde_json::json;

/// contains info about the `JetStream` usage from the current account.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct AccountInfo {
    pub memory: i64,
    pub storage: i64,
    pub streams: i64,
    pub consumers: i64,
}

mod jetstream {
    use super::*;
    use async_nats::jetstream::response::Response;

    #[tokio::test]
    async fn request_ok() {
        let server = nats_server::run_server("tests/configs/jetstream.conf");
        let client = async_nats::connect(server.client_url()).await.unwrap();
        let mut context = async_nats::jetstream::new(client);

        let response: Response<AccountInfo> = context
            .request("$JS.API.INFO".to_string(), &())
            .await
            .unwrap();

        assert!(matches!(response, Response::Ok { .. }));
    }

    #[tokio::test]
    async fn request_err() {
        let server = nats_server::run_server("tests/configs/jetstream.conf");
        let client = async_nats::connect(server.client_url()).await.unwrap();
        let mut context = async_nats::jetstream::new(client);

        let response: Response<AccountInfo> = context
            .request("$JS.API.STREAM.INFO.nonexisting".to_string(), &())
            .await
            .unwrap();

        assert!(matches!(response, Response::Err { .. }));
    }

    // Interesting edge case with Jetstream
    #[tokio::test]
    #[ignore]
    async fn request_no_responders() {
        // let server = nats_server::run_server("tests/configs/jetstream.conf");
        // let client = async_nats::connect(server.client_url()).await.unwrap();
        let client = async_nats::connect("nats://localhost:4222").await.unwrap();
        let mut context = async_nats::jetstream::new(client);

        let response: Response<AccountInfo> = context
            .request("$JS.API.FONI".to_string(), &())
            .await
            .unwrap();

        assert!(matches!(response, Response::Err { .. }));
    }
}