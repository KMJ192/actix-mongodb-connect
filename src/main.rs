use std::env;
use actix_web::{web, App, HttpServer, Responder};
use mongodb::{options::ClientOptions, Client};
use std::sync::Mutex;

pub mod logs_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	env::set_var("RUST_LOG", "actix_web=debug");
	let mongo_url = env::var("MONGO_DB_CONNECT").expect("Expected a url in the environment");

	let mut client_options = ClientOptions::parse(&mongo_url).await.expect("connection Error");
	client_options.app_name = Some("testDB".to_string());

	let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));

	HttpServer::new(move || 
		App::new().route("/", web::get().to(hello)).app_data(client.clone()).service(
			web::scope("/api").configure(logs_handler::scoped_config)
		))
	.bind("localhost:8081")?
	.run()
	.await
}

async fn hello() -> impl Responder {
	format!("Hello fellow!")
}
