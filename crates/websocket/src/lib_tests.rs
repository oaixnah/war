use super::{WebSocket, disable_outbound_connections};

#[tokio::test]
async fn disabled_connections_fail_before_network_access() {
    disable_outbound_connections();

    let error = match WebSocket::connect("ws://example.com", None::<&str>).await {
        Ok(_) => panic!("disabled websocket should fail"),
        Err(error) => error,
    };

    assert_eq!(
        error.to_string(),
        "outbound websocket connections are disabled"
    );
}
