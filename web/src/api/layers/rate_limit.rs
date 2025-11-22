use axum::body::Body;
use governor::middleware::StateInformationMiddleware;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::PeerIpKeyExtractor;

pub fn build_rate_limit_layer()
-> tower_governor::GovernorLayer<PeerIpKeyExtractor, StateInformationMiddleware, Body> {
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_millisecond(50)
            .burst_size(50)
            .use_headers()
            .finish()
            .unwrap(),
    );

    tower_governor::GovernorLayer::<PeerIpKeyExtractor, StateInformationMiddleware, Body>::new(
        governor_conf,
    )
}
