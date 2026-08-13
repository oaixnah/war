use super::{AuthState, PersistAction};

#[test]
fn local_state_has_no_account_identity_or_persistence() {
    let state = AuthState::initialize_local();

    assert!(!state.is_logged_in());
    assert!(state.user_id().is_none());
    assert_eq!(state.anonymous_id(), uuid::Uuid::nil().to_string());
    assert!(matches!(state.persist_action(), PersistAction::DoNothing));
}
