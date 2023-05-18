// The entry point for the server where the server is defined
use actix_web::{App, HttpServer};
use actix_service::Service;
mod json_serialization;
mod processes;
mod jwt;
mod state;
mod todo;
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
        .wrap_fn(|req, srv|{
            print!("{:?}", req);
            let future = srv.call(req);
            async {
                let res = future.await?;
                Ok(res)
            }
        }).configure(views::views_factory);
        return app;
    })
    .bind(("127.0.0.1", 8080))? //Resolves socket address(es) and binds server to created listener(s).
    .workers(3) //Sets number of workers to start (per bind address), default is no of physical CPUs
    .run() //Start listening for incoming connections.
    .await
}
