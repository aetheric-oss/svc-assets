//! Integration Tests
use lib_common::log_macros;

log_macros!("it", "test");

fn get_log_string(function: &str, name: &str) -> String {
    #[cfg(feature = "stub_server")]
    return format!("({} MOCK) {} server.", function, name);

    #[cfg(not(feature = "stub_server"))]
    return format!("({}) {} server.", function, name);
}

#[tokio::test]
async fn test_server_requests_and_logs() {
    use logtest::Logger;
    use svc_assets::grpc::server::*;

    let name = "assets";

    // Start the logger.
    let mut logger = Logger::start();

    //test_is_ready_request_logs
    {
        let imp = ServerImpl::default();
        let result = imp.is_ready(tonic::Request::new(ReadyRequest {})).await;
        assert!(result.is_ok());
        let result: ReadyResponse = result.unwrap().into_inner();
        assert_eq!(result.ready, true);

        // Search for the expected log message
        let expected = get_log_string("is_ready", name);
        println!("expected message: {}", expected);
        assert!(logger.any(|log| {
            if log.target().contains("app::") {
                println!("{}", log.target());
                let message = log.args();
                println!("{:?}", message);
                log.args() == expected
            } else {
                false
            }
        }));
    }
}
