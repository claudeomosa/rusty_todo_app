mod create;
mod get;
use actix_web::web::{ServiceConfig, get, post, scope};

pub fn todo_views_factory(app: &mut ServiceConfig){
    app.service(
        scope("app/items")
        .route("/", get().to(get::get))
        .route("create/{name}", post().to(create::create))
    );
}