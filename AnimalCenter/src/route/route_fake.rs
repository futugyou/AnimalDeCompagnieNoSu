use actix_web::{get, post, web, HttpResponse, Responder};
use std::sync::Mutex;

pub struct AppState {
    pub app_name: String,
}

pub struct AppStatwWithCounter {
    pub app_counter: Mutex<i32>,
}

impl AppStatwWithCounter {
    pub fn new() -> Self {
        Self {
            app_counter: Mutex::new(0),
        }
    }
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

pub fn makefakeroute<T, B>(app: actix_web::App<T, B>) -> actix_web::App<T, B>
where
    B: actix_web::dev::MessageBody,
    T: actix_service::ServiceFactory<
        Config = (),
        Request = actix_web::dev::ServiceRequest,
        Response = actix_web::dev::ServiceResponse<B>,
        Error = actix_web::Error,
        InitError = (),
    >,
{
    let app_counter = web::Data::new(AppStatwWithCounter::new());
    app // #region -> base usage
        .service(echo)
        // #endregion
        // #region -> data and app_data
        .data(AppState {
            app_name: String::from("hello world"),
        })
        .app_data(app_counter.clone())
        .service(index)
        .service(hello)
        // #endregion
        // #region -> use config
        .configure(configtest)
        // #endregion
        // #region -> web::scope
        .service(
            web::scope("/guard")
                .guard(actix_web::guard::Header("Host", "www.rust-lang.org")) //that means Header must had Host, and value is www.rust-lang.org
                .route("", web::get().to(manual_hello)),
        )
        .service(web::scope("/app").route("/hello", web::get().to(manual_hello)))
        // #endregion
        // #region -> direct route
        .route("/hey", web::get().to(manual_hello))
    // #endregion
}
