mod create;
mod edit;
mod get;
mod delete;
use actix_web::web::{get, post, scope, ServiceConfig};


pub fn todo_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("app/items")
            .route("/", get().to(get::get))
            .route("create/{name}", post().to(create::create))
            .route("edit", post().to(edit::edit))
            .route("delete", post().to(delete::delete))

    );
}
