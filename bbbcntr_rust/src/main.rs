use axum::{
    Router, 
    routing::{get, post},
    response::Html,
    Extension
};
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use bbbcntr_rust::controllers::*;
use sqlx::postgres::PgPoolOptions;
use anyhow::Context;

async fn hello_world() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html lang="en">
          <head>
            <meta charset="utf-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />
            <link rel="stylesheet" href="/static/css/index.css" />
            <link href="https://unpkg.com/pattern.css" rel="stylesheet" />
            
          </head>
          <body class="bg-black h-screen w-screen ">
            <div class="w-screen">
              <div class="h-screen w-full overflow-y-scroll no-scrollbar">
                <main class="text-gray-800 font-bold text-9xl text-center pattern-cross-dots-xl h-[60vh]  bg-gray-950 ">
                  <h1 class="
                    bg-clip-text text-transparent
                    bg-gradient-to-r from-red-600
                    to-purple-500 pt-20 py-10 animate-glotext no-scrollbar">
                    BBB <b class="font-black">Cntr</b>
                  </h1>
                  <h3 class="text-white text-xl font-medium pb-10">
                    All your data.
                    In one place.
                    Freedom.
                  </h3>
                  <h1 class="text-3xl bottom-0 text-white">
                    Get Started
                  </h1>
                </main>
                <div class="w-screen h-screen bg-gradient-to-r from-black via-purple-400 to-black pt-2 text-center">
                  <div class="p-4 font-mono text-2xl font-black text-white bg-gradient-to-b from-gray-950 to-transparent h-full">
                    Sigma sigma on the wall
                  </div>
                </div>
              </div>
            </div>
          </body>
        </html>
        "#
    )
}

fn init_router() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/tokens", get(all_tokens))
        .route("/tokens/:id", get(get_token_by_id))
        .route("/clients", get(all_clients))
        .route("/users", get(all_users))
        .route("/tokens", post(new_token))
        .route("/clients", post(new_client))
        .nest_service("/static", ServeDir::new("static"))
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    let env = std::fs::read_to_string(".env").unwrap();
    let (key, database_url) = env.split_once('=').unwrap();

    let pool = PgPoolOptions::new()
    .max_connections(50)
    .connect(&database_url)
    .await
    .context("could not connect to database_url")
    .unwrap();

    // build our application with a route
    let app = init_router()
          .layer(Extension(pool))
          .layer(TraceLayer::new_for_http());
        // `POST /users` goes to `create_user`
        // .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}