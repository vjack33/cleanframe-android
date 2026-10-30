slint::include_modules!();

use std::rc::Rc;
use slint::ComponentHandle;

#[derive(Default)]
pub struct AppState {
    pub active_zone: &'static str,
    pub selected_app: usize,
    pub selected_input: usize,
    pub selected_util: usize,
    pub selected_setting: usize,
    pub settings_open: bool,
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Debug)
            .with_tag("CLEANFRAME_RUST"),
    );
    log::info!("CleanFrame Native Rust starting on Android TV...");

    slint::android::init(app).expect("Failed to initialize Slint on Android");

    let main_window = MainWindow::new().expect("Failed to create MainWindow");
    let state = Rc::new(std::cell::RefCell::new(AppState {
        active_zone: "apps",
        selected_app: 3,
        selected_input: 0,
        selected_util: 0,
        selected_setting: 0,
        settings_open: false,
    }));

    main_window.set_clock_text("1:54".into());
    main_window.set_selected_app(3);

    // Connect remote navigation callbacks
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_key_up(move || {
            let mut s = state_clone.borrow_mut();
            if s.settings_open {
                if s.selected_setting > 0 {
                    s.selected_setting -= 1;
                }
            } else if s.active_zone == "apps" {
                if s.selected_app > 0 {
                    s.selected_app -= 1;
                }
            } else if s.active_zone == "util" {
                if s.selected_util > 0 {
                    s.selected_util -= 1;
                } else {
                    s.active_zone = "inputs";
                    s.selected_input = 0;
                }
            }
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_key_down(move || {
            let mut s = state_clone.borrow_mut();
            if s.settings_open {
                if s.selected_setting < 7 {
                    s.selected_setting += 1;
                }
            } else if s.active_zone == "apps" {
                if s.selected_app < 7 {
                    s.selected_app += 1;
                }
            } else if s.active_zone == "inputs" {
                s.active_zone = "util";
                s.selected_util = 0;
            } else if s.active_zone == "util" {
                if s.selected_util < 3 {
                    s.selected_util += 1;
                }
            }
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_key_left(move || {
            let mut s = state_clone.borrow_mut();
            if s.settings_open {
                s.settings_open = false;
            } else if s.active_zone == "inputs" {
                if s.selected_input > 0 {
                    s.selected_input -= 1;
                } else {
                    s.active_zone = "apps";
                }
            } else if s.active_zone == "util" {
                s.active_zone = "apps";
            }
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_key_right(move || {
            let mut s = state_clone.borrow_mut();
            if s.active_zone == "apps" {
                if s.selected_app >= 4 {
                    s.active_zone = "util";
                    if s.selected_app >= 6 {
                        s.selected_util = 3; // Direct jump to Settings!
                    } else {
                        s.selected_util = 0;
                    }
                } else {
                    s.active_zone = "inputs";
                    s.selected_input = 0;
                }
            } else if s.active_zone == "inputs" {
                if s.selected_input < 3 {
                    s.selected_input += 1;
                }
            }
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_key_ok(move || {
            let mut s = state_clone.borrow_mut();
            if s.active_zone == "util" && s.selected_util == 3 {
                s.settings_open = true;
                s.selected_setting = 0;
            }
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_key_back(move || {
            let mut s = state_clone.borrow_mut();
            if s.settings_open {
                s.settings_open = false;
            } else if s.active_zone != "apps" {
                s.active_zone = "apps";
            }
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    main_window.run().expect("Slint window run failed");
}

fn sync_ui(win: &MainWindow, s: &AppState) {
    win.set_active_zone(s.active_zone.into());
    win.set_selected_app(s.selected_app as i32);
    win.set_selected_input(s.selected_input as i32);
    win.set_selected_util(s.selected_util as i32);
    win.set_selected_setting(s.selected_setting as i32);
    win.set_settings_open(s.settings_open);
}

