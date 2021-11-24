use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mongodb::{options::ClientOptions, Client};
use std::sync::Mutex;

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
	HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
	HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	let client_options = ClientOptions::parse((project::MONGO_URL)).await.unwrap();
	let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));

	HttpServer::new(|| {
		App::new()
			.app_data(client.clone())
			.service(hello)
			.service(echo)
			.route("/test", web::get().to(manual_hello))
	})
	.bind("localhost:8080")?
	.run()
	.await
}