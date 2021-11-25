use actix_web::{web, HttpResponse, Responder};
use bson::{doc, Bson};
use futures::stream::StreamExt;
use mongodb::{options::FindOptions, Client};
use std::sync::Mutex;
use chrono::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewLog {
  pub id: String,
  pub message: String,
}

const MONGO_DB: &'static str = "iotPlantDB";
const MONGO_COLL_LOGS: &'static str = "logs";

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
  cfg.service(
    web::resource("/logs")
      .route(web::get().to(get_logs))
      .route(web::post().to(add_log)),
  );
}

async fn get_logs(data: web::Data<Mutex<Client>>) -> impl Responder {
  let logs_collection = data
    .lock()
    .unwrap()
    .database(MONGO_DB)
    .collection(MONGO_COLL_LOGS);

  let filter = doc! {};
  let find_options = FindOptions::builder().sort(doc! { "_id": -1}).build();
  let mut cursor = logs_collection.find(filter, find_options).await.unwrap();

  let mut results = Vec::new();
  while let Some(result) = cursor.next().await {
    match result {
      Ok(document) => {
        results.push(document);
      }
      _ => {
        return HttpResponse::InternalServerError().finish();
      }
    }
  }
  HttpResponse::Ok().json(results)
}

async fn add_log(data: web::Data<Mutex<Client>>, new_log: web::Json<NewLog>) -> impl Responder {
  let logs_collection = data
    .lock()
    .unwrap()
    .database(MONGO_DB)
    .collection(MONGO_COLL_LOGS);

  match logs_collection.insert_one(doc! {"deviceId": &new_log.id, "message": &new_log.message, "createdOn": Bson::DateTime(Utc::now())}, None).await {
    Ok(db_result) => {
      println!("{:?}", db_result);
      if let Some(new_id) = db_result.inserted_id.as_object_id() {
        println!("New document inserted with id {}", new_id);   
      }
      return HttpResponse::Created().json(db_result.inserted_id)
    }
    Err(err) =>
    {
      println!("Failed! {}", err);
      return HttpResponse::InternalServerError().finish()
    }
  }
}
