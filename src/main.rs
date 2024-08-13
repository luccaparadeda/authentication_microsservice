use utoipa::{
 OpenApi
};

use axum:: {
    routing::get,
    Router,
};
use tracing_subscriber;
use utoipa_scalar::{Scalar, Servable};

mod controllers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    #[derive(OpenApi)]
    #[openapi(
        info(
            description = "A simple authentication microsservice, using keycloak as authentication server",
            version = "1.0.0",
            title = "Keycloak authentication microsservice"
        ),
        paths(
            controllers::health
        ),
        tags(
            (name = "Healthcheck", description = "Healthcheck para saber se a API está no ar."),
        )
    )]
    struct ApiDoc;

     tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .merge(Scalar::with_url("/docs", ApiDoc::openapi())).route("/healthcheck", get(controllers::health));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
