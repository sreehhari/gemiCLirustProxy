use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
#[derive(Deserialize, Serialize)]
struct GeminiResponse {
    response: String,
}

#[get("/gemini")]
async fn gemini_endpoint(
    query: web::Query<std::collections::HashMap<String, String>>) -> impl Responder {
    let prompt = match query.get("prompt") {
        Some(p) => p,
        None => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({ "error": "Missing prompt" }))
        }
    };

    let api_key = match env::var("API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({ "error": "API_KEY not set" }))
        }
    };

    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", api_key);

    let client = reqwest::Client::new();
    let reqwest_body = serde_json::json!({
        "contents":[{"parts":[{"text":prompt}]}]
    });

    match client.post(&url).json(&reqwest_body).send().await {
        Ok(response) => match response.json::<serde_json::Value>().await {
            Ok(json) => {
                let text = json["candidates"][0]["content"]["parts"][0]["text"]
                    .as_str()
                    .unwrap_or("no response")
                    .to_string();
                HttpResponse::Ok().body(text)
            }
            Err(_) => HttpResponse::InternalServerError()
                .json(serde_json::json!({"error":"invalid response from the gemini api"})),
        },
        Err(_) => HttpResponse::InternalServerError()
            .json(serde_json::json!({"error":"request to gemini failed"})),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port: u16 = port.parse().unwrap_or(8080);
    println!("the server is running on port {}", port);

    HttpServer::new(|| App::new().service(gemini_endpoint))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
