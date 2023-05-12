use actix_web::{get, App, HttpRequest, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/greeter/{name}")]
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("httpserver factory is firing");
        App::new().service(index).service(greet) //Create application builder. and Register HTTP service
    })
    .bind(("127.0.0.1", 8080))? //Resolves socket address(es) and binds server to created listener(s).
    .workers(3) //Sets number of workers to start (per bind address), default is no of physical CPUs 
    .run() //Start listening for incoming connections.
    .await
}
