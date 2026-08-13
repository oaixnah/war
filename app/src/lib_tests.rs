use warpui::View as _;

use super::*;
use crate::settings::DefaultSessionMode;
#[cfg(feature = "local_tty")]
use crate::terminal::local_tty::spawner::PtySpawner;

fn assert_local_app_excludes_hosted_singletons(ctx: &AppContext) {
    assert!(!ctx.has_singleton_model::<ServerApiProvider>());
    assert!(!ctx.has_singleton_model::<AuthStateProvider>());
    assert!(!ctx.has_singleton_model::<AuthManager>());
    assert!(!ctx.has_singleton_model::<NetworkLogModel>());
    assert!(!ctx.has_singleton_model::<TelemetryCollector>());
    assert!(!ctx.has_singleton_model::<CloudModel>());
    assert!(!ctx.has_singleton_model::<Listener>());
    assert!(!ctx.has_singleton_model::<SyncQueue>());
    assert!(!ctx.has_singleton_model::<UpdateManager>());
    assert!(!ctx.has_singleton_model::<UserWorkspaces>());
    assert!(!ctx.has_singleton_model::<AIRequestUsageModel>());
    assert!(!ctx.has_singleton_model::<IapManager>());
    assert!(!ctx.has_singleton_model::<AutoupdateState>());
    assert!(!ctx.has_singleton_model::<ChangelogModel>());
    assert!(!ctx.has_singleton_model::<remote_server::manager::RemoteServerManager>());
    assert!(!ctx.has_singleton_model::<local_control::LocalControlServer>());
    assert!(ctx.has_singleton_model::<UndoCloseStack>());
}

#[test]
fn local_app_production_closure_constructs_and_renders_a_fresh_terminal() {
    use ::settings::Setting as _;

    let _undo_closed_panes = FeatureFlag::UndoClosedPanes.override_enabled(true);
    let tempdir = tempfile::tempdir().expect("temporary persistence directory should be created");
    persistence::with_test_app_database_file_path(tempdir.path().join("war.sqlite"), || {
        App::test(ASSETS, |mut app| async move {
            app.add_singleton_model(|ctx| AppExecutionMode::new(ExecutionMode::App, false, ctx));
            let (public_preferences, startup_toml_parse_error) =
                settings::init_public_user_preferences();
            let private_preferences = settings::init_private_user_preferences();
            app.add_singleton_model(move |_| {
                ::settings::PublicPreferences::new(public_preferences)
            });
            app.add_singleton_model(move |_| private_preferences);
            #[cfg(feature = "local_tty")]
            app.add_singleton_model(|_| PtySpawner::new_for_test());

            let app_state = app.update(|ctx| {
                initialize_local_app(IntervalTimer::new(), startup_toml_parse_error, ctx)
            });
            app.update(|ctx| {
                send_telemetry_from_app_ctx!(TelemetryEvent::QuitModalDisabled, ctx);
                send_telemetry_sync_from_app_ctx!(TelemetryEvent::QuitModalDisabled, ctx);
            });
            let _agent_view = FeatureFlag::AgentView.override_enabled(true);
            app.read(assert_local_app_excludes_hosted_singletons);

            let global_resource_handles =
                app.read(|ctx| GlobalResourceHandlesProvider::as_ref(ctx).get().clone());
            let (window_id, root) =
                app.add_window(warpui::platform::WindowStyle::NotStealFocus, |ctx| {
                    root_view::RootView::new(
                        global_resource_handles,
                        root_view::NewWorkspaceSource::Empty {
                            previous_active_window: None,
                            shell: None,
                        },
                        ctx,
                    )
                });

            root.read(&app, |view, ctx| drop(view.render(ctx)));
            let views = app.read(|ctx| ctx.view_ids_for_window(window_id));
            app.read(|ctx| {
                for view_id in views {
                    drop(
                        ctx.render_view(window_id, view_id)
                            .expect("every local root view dependency should render"),
                    );
                }
            });
            assert_eq!(app.views_of_type::<Workspace>(window_id).unwrap().len(), 1);
            assert_eq!(
                app.views_of_type::<crate::terminal::view::TerminalView>(window_id)
                    .unwrap()
                    .len(),
                1
            );
            let pane_group = app
                .views_of_type::<crate::pane_group::PaneGroup>(window_id)
                .expect("local pane group lookup should succeed")
                .into_iter()
                .next()
                .expect("local workspace should contain a pane group");
            AISettings::handle(&app).update(&mut app, |settings, ctx| {
                let _ = ctx;
                settings.default_session_mode_internal = DefaultSessionMode::Agent;
            });
            let closed_pane_id = pane_group.update(&mut app, |pane_group, ctx| {
                let pane_id: crate::pane_group::PaneId = pane_group
                    .terminal_pane_ids()
                    .next()
                    .expect("local pane group should contain a terminal");
                pane_group.add_terminal_pane(crate::pane_group::Direction::Right, None, ctx);
                pane_group.close_pane(pane_id, ctx);
                assert!(pane_group.is_pane_hidden_for_close(pane_id));
                pane_id
            });
            UndoCloseStack::handle(&app).update(&mut app, |stack, ctx| {
                stack.undo_close(ctx);
            });
            pane_group.read(&app, |pane_group, _| {
                assert!(!pane_group.is_pane_hidden_for_close(closed_pane_id));
                assert_eq!(pane_group.pane_count(), 2);
            });
            pane_group.update(&mut app, |pane_group, ctx| {
                pane_group.close_pane(closed_pane_id, ctx);
            });
            crate::undo_close::UndoCloseSettings::handle(&app).update(&mut app, |settings, ctx| {
                settings
                    .enabled
                    .set_value(false, ctx)
                    .expect("disable undo-close to discard the local pane");
            });
            pane_group.read(&app, |pane_group, _| {
                assert!(!pane_group.has_pane_id(closed_pane_id));
            });

            let workspace = app
                .views_of_type::<Workspace>(window_id)
                .expect("local workspace lookup should succeed")
                .into_iter()
                .next()
                .expect("local window should contain a workspace");
            let active_terminal = workspace
                .read(&app, |workspace, ctx| {
                    workspace
                        .active_tab_pane_group()
                        .read(ctx, |pane_group, ctx| pane_group.active_session_view(ctx))
                })
                .expect("local workspace should contain an active terminal");
            active_terminal.update(&mut app, |terminal, ctx| {
                terminal.set_active_block_session_id_for_test(
                    crate::terminal::model::session::SessionId::from(1),
                    ctx,
                );
            });
            workspace.update(&mut app, |workspace, ctx| {
                warpui::TypedActionView::handle_action(
                    workspace,
                    &WorkspaceAction::ShowCommandSearch(Default::default()),
                    ctx,
                );
                assert!(workspace.is_command_search_open());
            });
            let command_search = app
                .views_of_type::<crate::search::command_search::view::CommandSearchView>(window_id)
                .expect("local command search lookup should succeed")
                .into_iter()
                .next()
                .expect("local workspace should contain command search");
            command_search.read(&app, |view, ctx| drop(view.render(ctx)));
            command_search.update(&mut app, |view, ctx| {
                warpui::TypedActionView::handle_action(
                    view,
                    &crate::search::command_search::view::CommandSearchAction::Close,
                    ctx,
                );
            });
            workspace.read(&app, |workspace, _| {
                assert!(!workspace.is_command_search_open());
            });
            crate::undo_close::UndoCloseSettings::handle(&app).update(&mut app, |settings, ctx| {
                settings
                    .enabled
                    .set_value(true, ctx)
                    .expect("re-enable undo-close for local tab coverage");
            });
            workspace.update(&mut app, |workspace, ctx| {
                warpui::TypedActionView::handle_action(
                    workspace,
                    &WorkspaceAction::AddDefaultTab,
                    ctx,
                );
                assert_eq!(workspace.tab_count(), 2);
                warpui::TypedActionView::handle_action(
                    workspace,
                    &WorkspaceAction::CloseActiveTab,
                    ctx,
                );
                assert_eq!(workspace.tab_count(), 1);
            });
            assert!(!app.read(|ctx| UndoCloseStack::as_ref(ctx).is_empty()));
            UndoCloseStack::handle(&app).update(&mut app, |stack, ctx| {
                stack.undo_close(ctx);
            });
            workspace.read(&app, |workspace, _| {
                assert_eq!(workspace.tab_count(), 2);
            });
            let active_pane_group = workspace.read(&app, |workspace, _| {
                workspace.active_tab_pane_group().clone()
            });
            active_pane_group.update(&mut app, |pane_group, ctx| {
                let pane_id = pane_group
                    .pane_ids()
                    .next()
                    .expect("restored local tab should contain a pane");
                pane_group.close_pane(pane_id, ctx);
            });
            workspace.read(&app, |workspace, _| {
                assert_eq!(workspace.tab_count(), 1);
            });
            app.update(|ctx| {
                let menu_bar = app_menus::menu_bar(ctx);
                for menu in menu_bar.menus {
                    update_menu_items(menu.menu_items, ctx);
                }
                update_menu_items(app_menus::dock_menu().menu_items, ctx);
            });
            app.read(assert_local_app_excludes_hosted_singletons);

            app.update(|ctx| {
                ctx.dispatch_global_action("workspace:save_app", &());
                PersistenceWriter::handle(ctx).update(ctx, |writer, _| writer.terminate());
            });

            drop(app_state);
        });
    });
}

#[test]
fn local_app_preserves_corrupt_persistence_and_starts_fresh() {
    let tempdir = tempfile::tempdir().expect("temporary persistence directory should be created");
    let database_path = tempdir.path().join("war.sqlite");
    let corrupt_data = b"not a sqlite database";
    std::fs::write(&database_path, corrupt_data).expect("corrupt test database should be written");

    persistence::with_test_app_database_file_path(database_path.clone(), || {
        App::test(ASSETS, |mut app| async move {
            app.add_singleton_model(|ctx| AppExecutionMode::new(ExecutionMode::App, false, ctx));
            let (public_preferences, startup_toml_parse_error) =
                settings::init_public_user_preferences();
            let private_preferences = settings::init_private_user_preferences();
            app.add_singleton_model(move |_| {
                ::settings::PublicPreferences::new(public_preferences)
            });
            app.add_singleton_model(move |_| private_preferences);
            #[cfg(feature = "local_tty")]
            app.add_singleton_model(|_| PtySpawner::new_for_test());

            let app_state = app.update(|ctx| {
                initialize_local_app(IntervalTimer::new(), startup_toml_parse_error, ctx)
            });
            assert!(app_state.is_none());

            let global_resource_handles =
                app.read(|ctx| GlobalResourceHandlesProvider::as_ref(ctx).get().clone());
            let (window_id, root) =
                app.add_window(warpui::platform::WindowStyle::NotStealFocus, |ctx| {
                    root_view::RootView::new(
                        global_resource_handles,
                        root_view::NewWorkspaceSource::Empty {
                            previous_active_window: None,
                            shell: None,
                        },
                        ctx,
                    )
                });
            root.read(&app, |view, ctx| drop(view.render(ctx)));
            assert_eq!(app.views_of_type::<Workspace>(window_id).unwrap().len(), 1);
            assert_eq!(
                app.views_of_type::<crate::terminal::view::TerminalView>(window_id)
                    .unwrap()
                    .len(),
                1
            );
            app.update(|ctx| {
                PersistenceWriter::handle(ctx).update(ctx, |writer, _| writer.terminate());
            });
        });
    });

    assert_eq!(
        std::fs::read(database_path).expect("corrupt database should remain readable"),
        corrupt_data
    );
}

fn update_menu_items(items: Vec<warpui::platform::menu::MenuItem>, ctx: &mut AppContext) {
    use warpui::platform::menu::{MenuItem, MenuItemProperties};

    for item in items {
        if let MenuItem::Custom(item) = item {
            (item.updater)(&MenuItemProperties::default(), ctx);
            if let Some(submenu) = item.submenu {
                update_menu_items(submenu, ctx);
            }
        }
    }
}

#[test]
fn app_api_key_requires_validation() {
    let app = LaunchMode::App {
        args: Default::default(),
        api_key: Some("app-api-key".to_owned()),
    };

    assert!(matches!(
        app.auth_initialization(),
        AuthInitialization::PendingApiKey(api_key) if api_key == "app-api-key"
    ));
}

#[test]
fn tui_api_key_requires_validation() {
    let tui = LaunchMode::Tui {
        entrypoint: TuiEntryPoint::Interactive {
            mount: Box::new(|_| {}),
            api_key: Some("tui-api-key".to_owned()),
        },
    };

    assert!(matches!(
        tui.auth_initialization(),
        AuthInitialization::PendingApiKey(api_key) if api_key == "tui-api-key"
    ));
}

#[test]
fn command_line_api_key_requires_validation() {
    let command_line = LaunchMode::CommandLine {
        command: CliCommand::Whoami,
        global_options: GlobalOptions {
            api_key: Some("cli-api-key".to_owned()),
            ..Default::default()
        },
        debug: false,
        is_sandboxed: false,
        computer_use_override: None,
    };

    assert!(matches!(
        command_line.auth_initialization(),
        AuthInitialization::PendingApiKey(api_key) if api_key == "cli-api-key"
    ));
}

#[test]
fn startup_without_api_key_loads_persisted_auth() {
    let app = LaunchMode::App {
        args: Default::default(),
        api_key: None,
    };

    assert!(matches!(
        app.auth_initialization(),
        AuthInitialization::Persisted
    ));
}

#[test]
fn local_war_disables_networked_startup_services() {
    let app = LaunchMode::App {
        args: Default::default(),
        api_key: None,
    };

    assert!(app.is_local_war());
    assert!(!app.should_start_local_http_server());
    assert!(!app.needs_crash_reporting());
    assert!(!app.needs_profiling());
}

#[test]
fn local_war_does_not_skip_restoration_for_inherited_cloud_uri() {
    let url = Url::parse("war://action/new_cloud_agent_conversation?source=web_home")
        .expect("test URI should parse");
    let app = LaunchMode::App {
        args: warp_cli::AppArgs {
            urls: vec![url.clone()],
            ..Default::default()
        },
        api_key: None,
    };

    assert!(app.is_local_war());
    assert!(is_cloud_agent_web_home_launch_url(&url));
    assert!(!should_skip_restore_for_launch(&app));
}

#[test]
fn tui_uses_distinct_secure_storage_service_name() {
    let launch_mode = LaunchMode::Tui {
        entrypoint: TuiEntryPoint::Interactive {
            mount: Box::new(|_| {}),
            api_key: None,
        },
    };
    assert!(matches!(
        &launch_mode,
        LaunchMode::Tui {
            entrypoint: TuiEntryPoint::Interactive { .. }
        }
    ));

    assert_eq!(
        launch_mode.secure_storage_service_name("dev.warp.Warp-Dev"),
        "dev.warp.Warp-Dev.tui"
    );
}

#[test]
fn app_keeps_default_secure_storage_service_name() {
    let launch_mode = LaunchMode::App {
        args: Default::default(),
        api_key: None,
    };

    assert_eq!(
        launch_mode.secure_storage_service_name("dev.warp.Warp-Dev"),
        "dev.warp.Warp-Dev"
    );
}

#[test]
fn startup_auth_is_non_blocking_only_for_tui() {
    // Only the TUI front-end skips the startup IAP wait; every other launch mode
    // keeps the blocking behavior so this scope can't widen beyond the TUI.
    let tui = LaunchMode::Tui {
        entrypoint: TuiEntryPoint::Interactive {
            mount: Box::new(|_| {}),
            api_key: None,
        },
    };
    assert!(startup_auth_is_non_blocking(&tui));

    let blocking_modes = [
        LaunchMode::App {
            args: Default::default(),
            api_key: None,
        },
        LaunchMode::CommandLine {
            command: CliCommand::Whoami,
            global_options: GlobalOptions::default(),
            debug: false,
            is_sandboxed: false,
            computer_use_override: None,
        },
        LaunchMode::Test {
            driver: Box::new(None),
            is_integration_test: false,
            use_local_app: false,
        },
        LaunchMode::RemoteServerProxy,
        LaunchMode::RemoteServerDaemon {
            identity_key: "test".to_owned(),
        },
    ];
    for mode in blocking_modes {
        assert!(
            !startup_auth_is_non_blocking(&mode),
            "{} must block startup auth on IAP",
            mode.as_str_for_tracing()
        );
    }
}

#[test]
fn launch_modes_select_expected_logging_frontend() {
    let tui = LaunchMode::Tui {
        entrypoint: TuiEntryPoint::Interactive {
            mount: Box::new(|_| {}),
            api_key: None,
        },
    };
    let app = LaunchMode::App {
        args: Default::default(),
        api_key: None,
    };
    let test = LaunchMode::Test {
        driver: Box::new(None),
        is_integration_test: false,
        use_local_app: false,
    };

    assert_eq!(tui.log_frontend(), LogFrontend::Tui);
    assert_eq!(app.log_frontend(), LogFrontend::Gui);
    assert_eq!(test.log_frontend(), LogFrontend::Gui);
    assert_eq!(
        LaunchMode::RemoteServerProxy.log_frontend(),
        LogFrontend::Cli
    );
    assert_eq!(
        LaunchMode::RemoteServerDaemon {
            identity_key: "test".to_owned(),
        }
        .log_frontend(),
        LogFrontend::Cli
    );
}
