use crate::graphql::AnimalSchema;

use actix_cors::Cors;
use actix_web::{
    body::MessageBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
    guard, web, Error, HttpRequest, HttpResponse, Result, Scope,
};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};

pub fn graphqlscope() -> Scope<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl MessageBody>,
        Config = (),
        InitError = (),
        Error = Error,
    >,
> {
    let cors = Cors::default()
        // .send_wildcard()
        // .allowed_origin("http://localhost:5000")
        .allow_any_origin()
        .allow_any_method()
        .allow_any_header()
        .supports_credentials()
        .max_age(3600);

    web::scope("/graphql").wrap(cors).configure(playgroundroute)
}

fn playgroundroute(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(
                web::route()
                    .guard(guard::Header("upgrade", "websocket"))
                    .guard(guard::Get())
                    .to(index_ws),
            )
            .route(web::get().to(index_playground))
            .route(web::post().to(graphql_index)),
    );
}

async fn index_ws(
    schema: web::Data<AnimalSchema>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    GraphQLSubscription::new(Schema::clone(&*schema)).start(&req, payload)
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}

async fn graphql_index(schema: web::Data<AnimalSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
