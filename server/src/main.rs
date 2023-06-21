//! Main function starting the server and initializing dependencies.

use log::info;
use svc_assets::*;

#[tokio::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("(svc-assets) server startup.");

    // Will use default config settings if no environment vars are found.
    let config = Config::try_from_env().unwrap_or_default();

    // Start Logger
    let log_cfg: &str = config.log_config.as_str();
    if let Err(e) = log4rs::init_file(log_cfg, Default::default()) {
        panic!("(logger) could not parse {}: {}.", log_cfg, e);
    }

    // Allow option to only generate the spec file to a given location
    // use `make rust-openapi` to generate the OpenAPI specification
    let args = Cli::parse();
    if let Some(target) = args.openapi {
        return rest::generate_openapi_spec(&target);
    }

    // REST Server
    tokio::spawn(rest::server::rest_server(config.clone(), None));

    // GRPC Server
    tokio::spawn(grpc::server::grpc_server(config, None)).await?;

    info!("(svc-assets) server shutdown.");
    Ok(())
}
