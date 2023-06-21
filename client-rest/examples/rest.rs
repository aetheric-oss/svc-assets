//! Example communication with this service

use hyper::{Body, Client, Method, Request, Response};
use hyper::{Error, StatusCode};
use lib_common::grpc::get_endpoint_from_env;
use svc_assets_client_rest::types::*;

fn check_body(bytes: &hyper::body::Bytes) -> String {
    match String::from_utf8(bytes.to_vec()) {
        Ok(s) => s,
        Err(e) => {
            println!("Could not read string from bytes: {}", e);
            String::from("")
        }
    }
}

async fn evaluate(response: Result<Response<Body>, Error>, expected_code: StatusCode) -> String {
    let Ok(response) = response else {
        println!("Response was an Err() type: {:?}", response.as_ref().unwrap_err());
        println!("{:?}", response);
        return String::from("");
    };

    let status = response.status();

    if status != expected_code {
        println!("expected code: {}, actual: {}", expected_code, status);
        println!("{:?}", response);
        return String::from("");
    }

    let str = match hyper::body::to_bytes(response.into_body()).await {
        Ok(b) => check_body(&b),
        Err(e) => {
            println!("Could not get bytes from response body: {}", e);
            return String::from("");
        }
    };
    println!("{} (body: {})", status.to_string(), str);

    str
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("NOTE: Ensure the server is running, or this example will fail.");

    let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_REST");
    let url = format!("http://{host}:{port}");

    println!("Rest endpoint set to [{url}].");

    let client = Client::builder()
        .pool_idle_timeout(std::time::Duration::from_secs(10))
        .build_http();

    let aircraft_id: String;

    // POST /assets/aircraft
    {
        let data = vehicle::Data {
            vehicle_model_id: "Jet".to_string(),
            registration_number: "N2133423".to_string(),
            serial_number: "1234".to_string(),
            description: None,
            last_maintenance: Some(chrono::Utc::now().into()),
            next_maintenance: None,
            last_vertiport_id: None,
            created_at: None,
            updated_at: None,
            asset_group_id: None,
            schedule: None,
        };
        let data_str = match serde_json::to_string(&data) {
            Ok(s) => s,
            Err(e) => {
                println!("Error: {}", e);
                return Ok(());
            }
        };
        println!("Post data: {}", data_str);
        let uri = format!("{}/assets/aircraft", url);
        let req = match Request::builder()
            .method(Method::POST)
            .uri(uri.clone())
            .header("content-type", "application/json")
            .body(Body::from(data_str))
        {
            Ok(r) => r,
            Err(e) => {
                println!("Error: {}", e);
                return Ok(());
            }
        };

        let resp = client.request(req).await;
        let result_str = evaluate(resp, StatusCode::OK).await;
        aircraft_id = result_str;
        println!("Aircraft created: {}", aircraft_id);
    }

    // DELETE /assets/aircraft/{aircraft_id}
    {
        let uri = format!("{}/assets/aircraft/{}", url, aircraft_id);
        let req = match Request::builder()
            .method(Method::DELETE)
            .uri(uri.clone())
            .header("content-type", "application/json")
            .body(Body::empty())
        {
            Ok(r) => r,
            Err(e) => {
                println!("Error: {}", e);
                return Ok(());
            }
        };

        let resp = client.request(req).await;
        evaluate(resp, StatusCode::OK).await;
    }

    Ok(())
}
