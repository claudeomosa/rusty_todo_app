// The entry point for the server where the server is defined
use actix_web::{App, HttpServer };
mod views;
mod todo;
mod processes;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);
        return app
    })
    .bind(("127.0.0.1", 8080))? //Resolves socket address(es) and binds server to created listener(s).
    .workers(3) //Sets number of workers to start (per bind address), default is no of physical CPUs 
    .run() //Start listening for incoming connections.
    .await
}
