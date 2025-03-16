use std::path::PathBuf;

use poem::{Result, Route, Server, get, handler, listener::TcpListener, web::Path};

mod parse_portfolio;

fn fetch_data_file(fname: &str) -> PathBuf {
    [env!("CARGO_MANIFEST_DIR"), "src", "resources", fname]
        .iter()
        .collect()
}

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[handler]
fn portfolio() -> String {
    serde_json::to_string_pretty(&parse_portfolio::parse(fetch_data_file("portfolio.toml")))
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/hello/:name", get(hello))
        .at("/portfolio", get(portfolio));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
