use actix_web::{get, guard, post, rt::System, web, App, HttpResponse, HttpServer, Responder};
mod animal;
use std::sync::mpsc;
use std::sync::Mutex;
use std::thread;

struct AppState {
    app_name: String,
}

struct AppStatwWithCounter {
    app_counter: Mutex<i32>,
}

#[get("/")]
async fn hello(data: web::Data<AppState>, data2: web::Data<AppStatwWithCounter>) -> impl Responder {
    let app_name = &data.app_name;
    let mut app_counter = data2.app_counter.lock().unwrap();
    *app_counter += 1;
    let reslut = format!("{} {}", app_name, app_counter);
    HttpResponse::Ok().body(reslut)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/data")]
async fn index(data: web::Data<AppState>, data2: web::Data<AppStatwWithCounter>) -> String {
    let app_name = &data.app_name;
    let mut app_counter = data2.app_counter.lock().unwrap();
    *app_counter += 1;
    format!("{} {}", app_name, app_counter)
}

fn config1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test1")
            .route(web::get().to(|| HttpResponse::Ok().body("get test1")))
            .route(web::post().to(|| HttpResponse::Ok().body("post test1")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_counter = web::Data::new(AppStatwWithCounter {
        app_counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                app_name: String::from("hello world"),
            })
            .app_data(app_counter.clone())
            .configure(animal::animalconfig)
            .configure(config1)
            .service(
                web::scope("/guard")
                    .guard(guard::Header("Host", "www.rust-lang.org")) //that means Header must had Host, and value is www.rust-lang.org
                    .route("", web::get().to(manual_hello)),
            )
            .service(index)
            .service(hello)
            .service(echo)
            .service(web::scope("/app").route("/hello", web::get().to(manual_hello)))
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
