use super::{ChannelState, derive_http_origin_from_ws_url};

#[test]
fn oss_defaults_use_war_identity() {
    let state = ChannelState::init();

    assert_eq!(state.config.app_id.to_string(), "tech.oaix.War");
    assert_ne!(state.config.app_id.to_string(), "dev.warp.WarpOss");
    assert_eq!(state.config.logfile_name, "war.log");
    assert_ne!(state.config.logfile_name, "warp-oss.log");
    assert_eq!(ChannelState::url_scheme(), "war");
}

#[test]
fn wss_becomes_https_and_strips_path() {
    let got = derive_http_origin_from_ws_url("wss://rtc.app.warp.dev/graphql/v2");
    assert_eq!(got.as_deref(), Some("https://rtc.app.warp.dev"));
}

#[test]
fn ws_becomes_http_and_preserves_port() {
    let got = derive_http_origin_from_ws_url("ws://localhost:8080/graphql/v2");
    assert_eq!(got.as_deref(), Some("http://localhost:8080"));
}

#[test]
fn unparseable_input_returns_none() {
    assert!(derive_http_origin_from_ws_url("not a url").is_none());
    assert!(derive_http_origin_from_ws_url("https://app.warp.dev").is_none());
}
