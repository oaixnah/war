use chrono::Utc;

use super::{disable, flush_events, record_event};

#[test]
fn disabling_telemetry_clears_and_rejects_events() {
    record_event(
        None,
        "anonymous".to_owned(),
        "Before disable".into(),
        None,
        false,
        Utc::now(),
    );

    disable();
    record_event(
        None,
        "anonymous".to_owned(),
        "After disable".into(),
        None,
        false,
        Utc::now(),
    );

    assert!(flush_events().is_empty());
}
