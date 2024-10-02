use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct AppState {
    messages: Mutex<Vec<String>>,
}

async fn post_message(data: web::Data<AppState>, body: String) -> impl Responder {
    let mut messages = data.messages.lock().unwrap();
    messages.push(body);
    HttpResponse::Ok().body("Message received")
}

async fn get_messages(data: web::Data<AppState>) -> impl Responder {
    let messages = data.messages.lock().unwrap();
    let response = messages.join("\n");
    HttpResponse::Ok().body(response)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Server is running on port {port}");

    // Initialize shared state
    let shared_data = web::Data::new(AppState {
        messages: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .route("/message", web::post().to(post_message))
            .route("/messages", web::get().to(get_messages))
    })
    .bind(("0.0.0.0", port))?
    .workers(2)
    .run()
    .await
}
