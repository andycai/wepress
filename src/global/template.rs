use crate::template::AppState;

use axum_template::engine::Engine;
use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref APP_STATE: AppState = {
        let tera = Tera::new("templates/**/*").unwrap();
        AppState {
            engine: Engine::from(tera),
        }
    };
}
