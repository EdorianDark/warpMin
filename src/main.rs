use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_warp::{BadRequest, Response};
use http::StatusCode;
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter, Rejection};

mod starwars;
use starwars::{QueryRoot, StarWars};

async fn graphql_playground() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(HttpResponse::builder()
        .header("content-type", "text/html")
        .body(playground_source(GraphQLPlaygroundConfig::new("/"))))
}

async fn graphql_post(
    (schema, request): (
        Schema<QueryRoot, EmptyMutation, EmptySubscription>,
        async_graphql::Request,
    ),
) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(async_graphql_warp::Response::from(schema.execute(request).await))
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();

    println!("Playground: http://localhost:8000");

    let graphql_post_route = async_graphql_warp::graphql(schema).and_then(graphql_post);

    let graphql_playground_filter = warp::path::end()
        .and(warp::get())
        .and_then(graphql_playground);

    let routes = graphql_playground_filter.or(graphql_post);

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

