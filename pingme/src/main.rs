use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{routing::post, Router};
use http::StatusCode;
use notify_rust::{
    get_bundle_identifier_or_default, set_application, Notification,
};

const PORT: u16 = 30000;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let safari_id = get_bundle_identifier_or_default("Safari");
    if let Err(e) = set_application(&safari_id) {
        eprintln!("Error setting application: {}", e);
    }

    // build our application with a single route
    let app = Router::new().route(
        "/",
        post(|body: String| async move {
            if body.is_empty() {
                return (StatusCode::BAD_REQUEST, ());
            }
            if let Err(e) = Notification::new()
                .summary(&format!("Pinged by {body}"))
                .appname("PingMe")
                .show()
            {
                eprintln!("Error sending notification: {}", e);
            }
            (StatusCode::NO_CONTENT, ())
        }),
    );
    
    tokio::spawn(async move {
        let client = reqwest::Client::new();
        let mut last_ip = IpAddr::V4(Ipv4Addr::LOCALHOST);
        loop {
            let new_ip = loop {
                if let Some(ip) = public_ip::addr().await {
                    break ip;
                } else {
                    eprintln!("couldn't get an IP address");
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                }
            };
            if new_ip == last_ip {
                continue;
            }
            last_ip = new_ip;
            let socket_addr = SocketAddr::new(last_ip, PORT);
            loop {
                let resp = match client.post("https://www.manglemix.com/pingme/update").body(socket_addr.to_string()).send().await {
                    Ok(x) => x,
                    Err(e) => {
                        eprintln!("couldn't get response: {}", e);
                        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                        continue;
                    }
                };
                if resp.status().is_success() {
                    break;
                } else {
                    eprintln!("couldn't get success response: {}", resp.status().canonical_reason().unwrap_or(resp.status().as_str()));
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    });

    // run our app with hyper, listening globally on port 30000
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}")).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
