use warp_core::channel::Channel;

use super::{LocalPtyPolicy, SshRemoteServerSupport};

#[test]
fn oss_gui_policy_disables_hosted_local_pty_integrations() {
    let policy = LocalPtyPolicy::for_gui(Channel::Oss, false);

    assert!(!policy.enable_hosted_integrations);
    assert_eq!(
        policy.ssh_remote_server_support,
        SshRemoteServerSupport::Disabled
    );
}

#[test]
fn non_oss_gui_policy_preserves_hosted_local_pty_integrations() {
    for channel in [
        Channel::Stable,
        Channel::Preview,
        Channel::Dev,
        Channel::Local,
        Channel::Integration,
    ] {
        let policy = LocalPtyPolicy::for_gui(channel, false);

        assert!(policy.enable_hosted_integrations, "channel: {channel}");
        assert_eq!(
            policy.ssh_remote_server_support,
            SshRemoteServerSupport::Enabled,
            "channel: {channel}"
        );
    }
}

#[test]
fn oss_gui_policy_preserves_explicit_hosted_integrations() {
    let policy = LocalPtyPolicy::for_gui(Channel::Oss, true);

    assert!(policy.enable_hosted_integrations);
    assert_eq!(
        policy.ssh_remote_server_support,
        SshRemoteServerSupport::Enabled
    );
}
