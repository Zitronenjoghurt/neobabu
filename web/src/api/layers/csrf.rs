use crate::state::ServerState;
use axum_csrf::{CsrfConfig, CsrfLayer};

pub fn build_csrf_layer(state: &ServerState) -> CsrfLayer {
    let config = if state.config.is_dev_mode() {
        CsrfConfig::default().with_lifetime(tower_sessions::cookie::time::Duration::hours(1))
    } else {
        CsrfConfig::default()
            .with_lifetime(tower_sessions::cookie::time::Duration::hours(1))
            .with_secure(true)
            .with_cookie_domain(Some("neobabu.lemon.indstries"))
    };
    CsrfLayer::new(config)
}
