mod web;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use web::app::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("reached main");
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv::dotenv().ok();

    let app = App::new().await.expect("could not create app");
    app.serve(8080).await
}
