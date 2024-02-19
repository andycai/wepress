use crate::global::flag::ARGS;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ADDR: String = {
        if ARGS.addr.is_empty() {
            return std::env::var("ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
        }
        ARGS.addr.clone()
    };
}
