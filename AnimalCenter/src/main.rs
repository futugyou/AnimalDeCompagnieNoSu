use actix_web::{get, guard, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

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
