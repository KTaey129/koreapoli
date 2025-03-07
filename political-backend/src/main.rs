mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use dotenvy::dotenv;

// #[tokio::main]
// async fn main() {
//     dotenv().ok();

//     let app = Router::new()
//         .route("/", get(|| async {"Hello, World!"}));

//     let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
//     println!("Server running at http://{}", addr);

//     let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
//     axum::serve(listener, app).await.unwrap();
// }

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let app = Router::new()
        .merge(routes::polls::poll_routes());
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running at http://{}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
