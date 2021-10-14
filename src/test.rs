use actix_web::{get, HttpResponse,Responder};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Test {
    name : String,
}

#[get("/hi")]
pub(crate) async fn test() -> impl Responder {
    HttpResponse::Ok().json("API GET 방식으로 호출하기")
}

#[get("/hi/{id}")]
pub(crate) async fn get() -> impl Responder {
    // let test = Test { name: String::from(path) };
    HttpResponse::Ok().json("hiiiii")
}
