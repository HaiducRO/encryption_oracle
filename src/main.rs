use encryption_oracle::routes::routes;


#[tokio::main(flavor = "current_thread")]
async fn main() {
	let app = routes();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await
    .expect("Failed to bind address");
    axum::serve(listener, app)
    .await
    .expect("Server failed!");
}