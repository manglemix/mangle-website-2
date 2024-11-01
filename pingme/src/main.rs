use std::net::SocketAddr;

use axum::{routing::post, Router};
use http::StatusCode;
use ngrok::{config::TunnelBuilder, tunnel::UrlTunnel};
use notify_rust::{
    get_bundle_identifier_or_default, set_application, Notification,
};

const AUTHTOKEN: &str = env!("NGROK_AUTHTOKEN");

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

    let tun = ngrok::Session::builder()
        // Read the token from the NGROK_AUTHTOKEN environment variable
        .authtoken(AUTHTOKEN)
        // Connect the ngrok session
        .connect()
        .await.unwrap()
        // Start a tunnel with an HTTP edge
        .http_endpoint()
        .listen()
        .await.unwrap();

    println!("Tunnel started on URL: {:?}", tun.url());

    {
        let client = reqwest::Client::new();
        client.post("https://www.manglemix.com/pingme/update").body(tun.url().to_string()).send().await.unwrap();
    }
    
    axum::Server::builder(tun)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
