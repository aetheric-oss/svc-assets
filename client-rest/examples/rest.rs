//! Example communication with this service

use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::{body::Bytes, body::Incoming, Error, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use lib_common::grpc::get_endpoint_from_env;
use lib_common::time::Utc;
use std::convert::Infallible;
use svc_assets_client_rest::types::*;
use tokio::net::TcpStream;

fn check_body(bytes: &Bytes) -> String {
    match String::from_utf8(bytes.to_vec()) {
        Ok(s) => s,
        Err(e) => {
            println!("Could not read string from bytes: {}", e);
            String::from("")
        }
    }
}
fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, Infallible> {
    Full::new(chunk.into()).boxed()
}
fn empty() -> BoxBody<Bytes, Infallible> {
    Empty::<Bytes>::new().boxed()
}

async fn evaluate(
    resp: Result<Response<Incoming>, Error>,
    expected_code: StatusCode,
) -> (bool, String) {
    let Ok(response) = resp else {
        println!(
            "Response was an Err() type: {:?}",
            resp.as_ref().unwrap_err()
        );
        println!("{:?}", resp);
        return (false, String::from(""));
    };

    let status = response.status();

    if status != expected_code {
        println!("expected code: {}, actual: {}", expected_code, status);
        println!("{:?}", response);
        return (false, String::from(""));
    }

    let bytes = match response.collect().await {
        Ok(bytes) => bytes,
        Err(e) => {
            println!("{}", e);
            return (false, format!("{}", e));
        }
    }
    .to_bytes();

    let str = check_body(&bytes);
    println!("{} (body: {})", status, str);

    (true, str)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("NOTE: Ensure the server is running, or this example will fail.");

    let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_REST");
    let addr = format!("{host}:{port}");

    println!("Rest endpoint set to [{addr}].");

    let stream = TcpStream::connect(addr.clone()).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let mut ok = true;

    let aircraft_id: String;

    // POST /assets/aircraft
    {
        let data = vehicle::Data {
            vehicle_model_id: "Jet".to_string(),
            registration_number: "N2133423".to_string(),
            serial_number: "1234".to_string(),
            description: None,
            last_maintenance: Some(Utc::now().into()),
            next_maintenance: None,
            hangar_id: None,
            hangar_bay_id: None,
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
        let uri = format!("http://{}/assets/aircraft", addr.clone());
        let req = match Request::builder()
            .method(Method::POST)
            .uri(uri.clone())
            .header("content-type", "application/json")
            .body(full(data_str))
        {
            Ok(r) => r,
            Err(e) => {
                println!("Error: {}", e);
                return Ok(());
            }
        };
        let res = sender.send_request(req).await;
        let (success, result_str) = evaluate(res, StatusCode::OK).await;
        ok &= success;

        aircraft_id = result_str;
        println!("Aircraft created: {}", aircraft_id);
    }

    // DELETE /assets/aircraft/{aircraft_id}
    {
        let uri = format!("http://{}/assets/aircraft/{}", addr, aircraft_id);
        let req = match Request::builder()
            .method(Method::DELETE)
            .uri(uri.clone())
            .header("content-type", "application/json")
            .body(empty())
        {
            Ok(r) => r,
            Err(e) => {
                println!("Error: {}", e);
                return Ok(());
            }
        };

        let res = sender.send_request(req).await;
        let (success, result_str) = evaluate(res, StatusCode::OK).await;
        ok &= success;

        println!("Result {}", result_str);
    }

    if ok {
        println!("\u{1F9c1} All endpoints responded!");
    } else {
        eprintln!("\u{2620} Errors");
    }

    Ok(())
}
