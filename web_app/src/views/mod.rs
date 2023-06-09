// The factory that defines all the views for the whole app
mod app;
mod auth;
mod todo;
use auth::auth_views_factory;
use todo::todo_views_factory;

use actix_web::web::ServiceConfig;

pub fn views_factory(app: &mut ServiceConfig) {
    auth_views_factory(app);
    todo_views_factory(app);
    app::app_views_factory(app)
}
