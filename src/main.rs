use actix_web::{web::{self, Query},App,HttpResponse,HttpServer,Responder};
use reqwest;
use serde_json::Value;
use dotenvy::dotenv;
use std::env;

async fn gemini_handler(query:web:Query<std::collections::HashMap<String,String>>)-impl Responder{
    let prompt = match query.get("prompt"){
        Some(p)=>p.clone(),
        None => return HttpResponse::BadRequest().json(serde_json::json!({"error":"missing prompt"})),
    },

    let api_key = match env::var("API_KEY"){
        Ok(key)=>key,
        
    }
}
