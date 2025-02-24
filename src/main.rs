use actix_web::{get, web::{self, ServiceConfig, Data}, App, HttpResponse, Responder, HttpServer};
use dotenvy::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
#[derive(Deserialize, Serialize)]
struct GeminiResponse {
    response: String,
}

#[get("/gemini")]
async fn gemini_endpoint(
    query: web::Query<std::collections::HashMap<String, String>>,
api_key:web::Data<String>,) -> impl Responder {
    let prompt = match query.get("prompt") {
        Some(p) => p,
        None => {
            return HttpResponse::BadRequest()
                .json(serde_json::json!({ "error": "Missing prompt" }))
        }
    };



    // let api_key = match env::var("API_KEY") {
    //     Ok(key) => key,
    //     Err(_) => {
    //         return HttpResponse::InternalServerError()
    //             .json(serde_json::json!({ "error": "API_KEY not set" }))
    //     }
    // };

    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", api_key.get_ref());

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
#[shuttle_runtime::main]
async fn actix_web(#[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_actix_web::ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // dotenv().ok();
    let api_key= secrets.get("API_KEY").expect("api key not found");
    let factory = move |cfg: &mut ServiceConfig|{
        cfg.app_data(Data::new(api_key.clone()))
        .service(gemini_endpoint);
    };
    Ok(factory.into())
}
