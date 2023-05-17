mod create;
use actix_web::web::{ServiceConfig, post, scope};

pub fn todo_views_factory(app: &mut ServiceConfig){
    app.service(
        scope("app/item")
        .route("/create/{name}", post().to(create::create))
    );
}