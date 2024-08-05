use http_server::HttpServer;

mod http_server;
mod stats;
mod terminal;

pub type AppResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> AppResult<()> {
    HttpServer::run()
}
