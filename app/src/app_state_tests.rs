use super::*;

fn local_terminal_leaf() -> PaneNodeSnapshot {
    PaneNodeSnapshot::Leaf(LeafSnapshot {
        is_focused: true,
        custom_vertical_tabs_title: None,
        contents: LeafContents::Terminal(TerminalPaneSnapshot {
            uuid: vec![1],
            cwd: Some("/tmp".to_owned()),
            shell_launch_data: None,
            is_active: true,
            is_read_only: false,
            input_config: None,
            llm_model_override: None,
            active_profile_id: None,
            conversation_ids_to_restore: vec![],
            active_conversation_id: None,
        }),
    })
}

fn window_with_root(root: PaneNodeSnapshot) -> WindowSnapshot {
    WindowSnapshot {
        tabs: vec![TabSnapshot {
            custom_title: None,
            root,
            default_directory_color: None,
            selected_color: SelectedTabColor::default(),
            left_panel: None,
            right_panel: None,
            group_id: None,
            pinned: false,
        }],
        active_tab_index: 0,
        team_uid: None,
        bounds: None,
        fullscreen_state: Default::default(),
        quake_mode: false,
        universal_search_width: None,
        warp_ai_width: None,
        voltron_width: None,
        warp_drive_index_width: None,
        left_panel_open: false,
        vertical_tabs_panel_open: false,
        left_panel_width: None,
        right_panel_width: None,
        agent_management_filters: None,
        tab_groups: vec![],
    }
}

#[test]
fn local_terminal_window_is_safe_to_restore() {
    let window = window_with_root(PaneNodeSnapshot::Branch(BranchSnapshot {
        direction: SplitDirection::Vertical,
        children: vec![
            (PaneFlex(1.0), local_terminal_leaf()),
            (PaneFlex(1.0), local_terminal_leaf()),
        ],
    }));

    assert!(window.is_local_terminal_only());
}

#[test]
fn hosted_or_agent_state_is_not_safe_to_restore() {
    let mut window = window_with_root(local_terminal_leaf());
    let PaneNodeSnapshot::Leaf(leaf) = &mut window.tabs[0].root else {
        unreachable!();
    };
    let LeafContents::Terminal(terminal) = &mut leaf.contents else {
        unreachable!();
    };
    terminal.llm_model_override = Some("hosted-model".to_owned());
    assert!(!window.is_local_terminal_only());

    let code_window = window_with_root(PaneNodeSnapshot::Leaf(LeafSnapshot {
        is_focused: true,
        custom_vertical_tabs_title: None,
        contents: LeafContents::Code(CodePaneSnapShot::Local {
            tabs: vec![],
            active_tab_index: 0,
            source: None,
        }),
    }));
    assert!(!code_window.is_local_terminal_only());
}

#[test]
fn empty_pane_branch_is_not_safe_to_restore() {
    let window = window_with_root(PaneNodeSnapshot::Branch(BranchSnapshot {
        direction: SplitDirection::Vertical,
        children: vec![],
    }));

    assert!(!window.is_local_terminal_only());
}

#[test]
fn test_has_horizontal_split() {
    let single_leaf = PaneNodeSnapshot::Leaf(LeafSnapshot {
        is_focused: false,
        custom_vertical_tabs_title: None,
        contents: LeafContents::Code(CodePaneSnapShot::Local {
            tabs: vec![CodePaneTabSnapshot {
                path: Some(PathBuf::new()),
            }],
            active_tab_index: 0,
            source: None,
        }),
    });
    assert!(!single_leaf.has_horizontal_split());

    let horizontal_split = PaneNodeSnapshot::Branch(BranchSnapshot {
        direction: SplitDirection::Horizontal,
        children: vec![
            (
                PaneFlex(1.),
                PaneNodeSnapshot::Leaf(LeafSnapshot {
                    is_focused: false,
                    custom_vertical_tabs_title: None,
                    contents: LeafContents::Code(CodePaneSnapShot::Local {
                        tabs: vec![CodePaneTabSnapshot {
                            path: Some(PathBuf::new()),
                        }],
                        active_tab_index: 0,
                        source: None,
                    }),
                }),
            ),
            (
                PaneFlex(1.),
                PaneNodeSnapshot::Leaf(LeafSnapshot {
                    is_focused: false,
                    custom_vertical_tabs_title: None,
                    contents: LeafContents::Code(CodePaneSnapShot::Local {
                        tabs: vec![CodePaneTabSnapshot {
                            path: Some(PathBuf::new()),
                        }],
                        active_tab_index: 0,
                        source: None,
                    }),
                }),
            ),
        ],
    });
    assert!(horizontal_split.has_horizontal_split());
}

#[test]
fn test_code_pane_snapshot_single_tab() {
    let snapshot = CodePaneSnapShot::Local {
        tabs: vec![CodePaneTabSnapshot {
            path: Some(PathBuf::from("/tmp/test.rs")),
        }],
        active_tab_index: 0,
        source: Some(CodeSource::FileTree {
            location: crate::code::buffer_location::LocalOrRemotePath::Local(PathBuf::from(
                "/tmp/test.rs",
            )),
        }),
    };
    let CodePaneSnapShot::Local {
        tabs,
        active_tab_index,
        source,
    } = &snapshot;
    assert_eq!(tabs.len(), 1);
    assert_eq!(*active_tab_index, 0);
    assert_eq!(tabs[0].path, Some(PathBuf::from("/tmp/test.rs")));
    assert!(matches!(source, Some(CodeSource::FileTree { .. })));
}

#[test]
fn test_code_pane_snapshot_with_multiple_tabs() {
    let snapshot = CodePaneSnapShot::Local {
        tabs: vec![
            CodePaneTabSnapshot {
                path: Some(PathBuf::from("/tmp/main.rs")),
            },
            CodePaneTabSnapshot {
                path: Some(PathBuf::from("/tmp/lib.rs")),
            },
            CodePaneTabSnapshot { path: None },
        ],
        active_tab_index: 1,
        source: Some(CodeSource::Link {
            path: PathBuf::from("/tmp/main.rs"),
            range_start: None,
            range_end: None,
        }),
    };
    let CodePaneSnapShot::Local {
        tabs,
        active_tab_index,
        source,
    } = &snapshot;
    assert_eq!(tabs.len(), 3);
    assert_eq!(*active_tab_index, 1);
    assert_eq!(tabs[0].path, Some(PathBuf::from("/tmp/main.rs")));
    assert_eq!(tabs[1].path, Some(PathBuf::from("/tmp/lib.rs")));
    assert_eq!(tabs[2].path, None);
    assert!(matches!(source, Some(CodeSource::Link { .. })));
}
