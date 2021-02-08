use crate::AnimalSchema;

use actix_web::Scope;
use actix_web::{web, HttpResponse, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};

pub fn graphqlscope() -> Scope {
    web::scope("/graphql").configure(playgroundroute)
}

fn playgroundroute(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(index_playground))
            .route(web::post().to(graphql_index)),
    );
}

async fn index_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
        )))
}

async fn graphql_index(schema: web::Data<AnimalSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}
