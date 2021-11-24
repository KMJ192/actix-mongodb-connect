#![allow(proc_macro_derive_resolution_fallback)]

use crate::db::model::Sample;
use crate::Mongo;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};
use std::collections::HashMap;

#[get("/notice")]
pub async fn hello(db: web::Data<Mongo>, req: HttpRequest) -> impl Responder {
    let params = web::Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();

    // 아래와 같이 query를 보내면 Query struct엔 아래와 같이 Map으로 저장됨
    // http://localhost:8010/notice?date=21.01.06&id=4
    // Query({"id": "4", "date": "21.01.06"})
    // println!("{:?}", params);

    let typed_collection = db
        .lock()
        .unwrap()
        .database("ajou")
        .collection::<Sample>("notice");

    let date = params.get("date");
    let cate = params.get("category");

    
    // query 중 date, category 둘다 있는지 한 개만 있는지 체크해서 build
    let mut notices = if date.is_some() && cate.is_some() {
        typed_collection
            .find(
                doc! {"$and" : [{ "date": { "$eq": date } }, { "category": { "$eq": cate }}]},
                None,
            )
            // .find(doc! {"date": {"$eq": params.get("date")}}, None)
            .await
            .unwrap()
    } else if date.is_some() {
        typed_collection
            .find(doc! {"date": {"$eq": date}}, None)
            .await
            .unwrap()
    } else if cate.is_some() {
        typed_collection
            .find(doc! {"category": {"$eq": cate}}, None)
            .await
            .unwrap()
    } else {
        let find_options = FindOptions::builder().limit(1).build();
        typed_collection.find(doc! {}, find_options).await.unwrap()
    };

    let mut result: Vec<Sample> = Vec::new();
    while let Some(notice) = notices.try_next().await.unwrap() {
        result.push(notice);
    }

    HttpResponse::Ok()
        .content_type("application/json")
        .json(json!(result))
}

/* 결과
GET: http://localhost:8010/notice?date=21.08.26&category=학사

[
    {
        "id": 13987,
        "category": "학사",
        "title": "[봉사활동] 2021 하반기 영통구청 저소득층 언택트 멘토링 자원봉사자 모집(~8/31)",
        "date": "21.08.26",
        "link": "https://www.ajou.ac.kr/kr/ajou/notice.do?mode=view&articleNo=112670&article.offset=0&articleLimit=1648",
        "writer": "사회봉사센터"
    },
    {
        "id": 13988,
        "category": "학사",
        "title": "[다산학부대학] 2021학년도 2학기 신설 교과목 안내(기초수학A, 기초수학B)",
        "date": "21.08.26",
        "link": "https://www.ajou.ac.kr/kr/ajou/notice.do?mode=view&articleNo=112674&article.offset=0&articleLimit=1648",
        "writer": "다산학부대학교학팀"
    },
    {
        "id": 13992,
        "category": "학사",
        "title": "(추가공지)[다산학부대학] 2021-2학기 Co-BSM 본수강신청 안내",
        "date": "21.08.26",
        "link": "https://www.ajou.ac.kr/kr/ajou/notice.do?mode=view&articleNo=112687&article.offset=0&articleLimit=1648",
        "writer": "다산학부대학교학팀"
    }
]

 */