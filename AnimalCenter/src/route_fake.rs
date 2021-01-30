use actix_web::{get, post, web, HttpResponse, Responder};
use std::sync::Mutex;

pub struct AppState {
    pub app_name: String,
}

pub struct AppStatwWithCounter {
    pub app_counter: Mutex<i32>,
}

#[get("/")]
pub async fn hello(
    data: web::Data<AppState>,
    data2: web::Data<AppStatwWithCounter>,
) -> impl Responder {
    let app_name = &data.app_name;
    let mut app_counter = data2.app_counter.lock().unwrap();
    *app_counter += 1;
    let reslut = format!("{} {}", app_name, app_counter);
    HttpResponse::Ok().body(reslut)
}

#[get("/data")]
pub async fn index(data: web::Data<AppState>, data2: web::Data<AppStatwWithCounter>) -> String {
    let app_name = &data.app_name;
    let mut app_counter = data2.app_counter.lock().unwrap();
    *app_counter += 1;
    format!("{} {}", app_name, app_counter)
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub fn configtest(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test1")
            .route(web::get().to(|| HttpResponse::Ok().body("get test1")))
            .route(web::post().to(|| HttpResponse::Ok().body("post test1")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}
