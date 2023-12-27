#![allow(unreachable_code)]
#[macro_use]
use reqwest::Client;
use rouille::{input, router};
use serde::{Deserialize, Serialize};
use std::error::Error;
extern crate dotenv;
extern crate rouille;

use dotenv::dotenv;
use std::env;

// Define a struct to hold the access token response
#[derive(Deserialize)]
struct AccessTokenResponse {
    access_token: String,
    // Include other fields as needed
}

// Define a struct for the request body to exchange the code for a token
#[derive(Serialize)]
struct AccessTokenRequest {
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
}

async fn exchange_code_for_token(code: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let req_body = AccessTokenRequest {
        client_id: "YOUR_CLIENT_ID".to_string(),
        client_secret: "YOUR_CLIENT_SECRET".to_string(),
        code: code.to_string(),
        redirect_uri: "YOUR_REDIRECT_URI".to_string(),
    };

    let res = client
        .post("https://slack.com/api/oauth.v2.access")
        .json(&req_body)
        .send()
        .await?
        .json::<AccessTokenResponse>()
        .await?;

    Ok(res.access_token)
}

fn main() {
    dotenv().ok();

    let client_id = std::env::var("CLIENT_ID").expect("CLIENT_ID must be set.");
    let scope = "channels:read,groups:read,channels:write,chat:write";
    let redirect_uri = "http://localhost:8080/callback"; // Or use OOB if localhost isn't viable

    println!("Please go to the following URL and authorize the app:");
    println!(
        "https://slack.com/oauth/v2/authorize?client_id={}&scope={}&redirect_uri={}",
        client_id, scope, redirect_uri
    );

    println!("Now listening on localhost:8000");

    // The `start_server` starts listening forever on the given address.
    rouille::start_server("localhost:8000", move |request| {
        // The closure passed to `start_server` will be called once for each client request. It
        // will be called multiple times concurrently when there are multiple clients.
        // Here starts the real handler for the request.
        //
        // The `router!` macro is very similar to a `match` expression in core Rust. The macro
        // takes the request as parameter and will jump to the first block that matches the
        // request.
        //
        // Each of the possible blocks builds a `Response` object. Just like most things in Rust,
        // the `router!` macro is an expression whose value is the `Response` built by the block
        // that was called. Since `router!` is the last piece of code of this closure, the
        // `Response` is then passed back to the `start_server` function and sent to the client.
        router!(request,

            (GET) (/callback) => {
                // If the request's URL is `/hello/world`, we jump here.
                println!("{:#?}", request.raw_query_string());



                // Builds a `Response` object that contains the "hello world" text.
                rouille::Response::text("hello world")
            },

            // The code block is called if none of the other blocks matches the request.
            // We return an empty response with a 404 status code.
            _ => rouille::Response::empty_404()
        )
    });
}
