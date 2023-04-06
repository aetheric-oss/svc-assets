//! Example communication with this service

use hyper::{Body, Client, Method, Request, Response};
use hyper::{Error, StatusCode};
use svc_assets_client_rest::types::*;

async fn evaluate(
    resp: Result<Response<Body>, Error>,
    expected_code: StatusCode,
) -> (bool, String) {
    let mut ok = true;
    let result_str: String = match resp {
        Ok(r) => {
            let tmp = r.status() == expected_code;
            ok &= tmp;

            match hyper::body::to_bytes(r.into_body()).await {
                Ok(b) => match String::from_utf8(b.to_vec()) {
                    Ok(s) => s,
                    Err(e) => {
                        ok = false;
                        return (ok, e.to_string());
                    }
                },
                Err(e) => {
                    ok = false;
                    return (ok, e.to_string());
                }
            }
        }
        Err(e) => {
            ok = false;
            e.to_string()
        }
    };

    (ok, result_str)
}

#[allow(unused_assignments)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("NOTE: Ensure the server is running, or this example will fail.");

    let rest_port = std::env::var("HOST_PORT_REST").unwrap_or_else(|_| "8004".to_string());

    // let host_port = env!("HOST_PORT");
    let url = format!("http://0.0.0.0:{rest_port}");
    let mut ok = true;
    let client = Client::builder()
        .pool_idle_timeout(std::time::Duration::from_secs(10))
        .build_http();

    let mut aircraft_id = String::new();

    // POST /assets/aircraft
    {
        let data = RegisterAircraftPayload {
            manufacturer: "Elroy".to_string(),
            model: "Jet".to_string(),
            max_payload_kg: 1000.0,
            max_range_km: 1000.0,
            owner: "123".to_string(),
            registration_number: "N2133423".to_string(),
            serial_number: "1234".to_string(),
            status: AssetStatus::Available,
            whitelist: vec![],
            description: None,
            group_id: None,
            last_maintenance: None,
            name: None,
            next_maintenance: None,
            last_vertiport_id: None,
        };
        let data_str = match serde_json::to_string(&data) {
            Ok(s) => s,
            Err(e) => {
                println!("Error: {}", e);
                return Ok(());
            }
        };
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
        let (success, result_str) = evaluate(resp, StatusCode::OK).await;
        ok &= success;
        aircraft_id = result_str.clone();
        println!("{}: {}", uri, result_str);
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
        let (success, result_str) = evaluate(resp, StatusCode::OK).await;
        ok &= success;

        println!("{}: {}", uri, result_str);
    }

    if ok {
        println!("\u{1F9c1} All endpoints responded!");
    } else {
        eprintln!("\u{2620} Errors");
    }

    Ok(())
}
