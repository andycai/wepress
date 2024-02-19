use axum::extract::FromRef;
use axum_template::engine::Engine;
use tera::Tera;

pub type AppEngine = Engine<Tera>;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub engine: AppEngine,
}
