mod animal;

use actix_web::{get, guard, post, web, App, HttpResponse, HttpServer, Responder, Result};
use animal::{AnimalSchema, QueryRoot};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
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

fn config1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test1")
            .route(web::get().to(|| HttpResponse::Ok().body("get test1")))
            .route(web::post().to(|| HttpResponse::Ok().body("post test1")))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed())),
    );
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_counter = web::Data::new(AppStatwWithCounter {
        app_counter: Mutex::new(0),
    });
    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                app_name: String::from("hello world"),
            })
            .data(schema.clone())
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
            .service(
                web::resource("/graphql")
                    .guard(guard::Get())
                    .to(index_playground),
            )
            .service(
                web::resource("/graphql")
                    .guard(guard::Post())
                    .to(graphql_index),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
async fn graphql_index(schema: web::Data<AnimalSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}
