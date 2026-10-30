slint::include_modules!();

use std::rc::Rc;
use std::cell::RefCell;
use slint::{ComponentHandle, Model};

#[derive(Default)]
pub struct AppState {
    pub active_zone: &'static str, // "apps" | "inputs" | "util"
    pub selected_app: usize,
    pub selected_input: usize,
    pub selected_util: usize,
    pub selected_setting: usize,
    pub settings_open: bool,
    pub wallpaper_idx: usize,
    pub layout_mode: &'static str,
}

const WALLPAPER_NAMES: [&str; 5] = [
    "Starry night",
    "Mountain valley",
    "Misty coast",
    "Alpine peaks",
    "Amber glow",
];

const APP_PACKAGES: [&str; 8] = [
    "org.smarttube.stable",
    "com.spotify.tv.android",
    "com.apple.atve.androidtv.appletv",
    "com.amazon.amazonvideo.livingroom",
    "com.netflix.ninja",
    "com.aurora.store",
    "com.spocky.projengmenu",
    "android.settings.SETTINGS",
];

#[cfg(target_os = "android")]
fn launch_package(app: &slint::android::AndroidApp, package: &str) {
    if package == "android.settings.SETTINGS" {
        launch_settings(app);
        return;
    }
    unsafe {
        let vm_ptr = app.vm_as_ptr() as *mut jni::sys::JavaVM;
        if vm_ptr.is_null() { return; }
        let vm = match jni::JavaVM::from_raw(vm_ptr) {
            Ok(v) => v,
            Err(_) => return,
        };
        let mut env = match vm.attach_current_thread() {
            Ok(e) => e,
            Err(_) => return,
        };
        let activity = jni::objects::JObject::from_raw(app.activity_as_ptr() as jni::sys::jobject);

        let pkg_jstring = match env.new_string(package) {
            Ok(s) => s,
            Err(_) => return,
        };

        let pm = match env.call_method(&activity, "getPackageManager", "()Landroid/content/pm/PackageManager;", &[]) {
            Ok(val) => match val.l() {
                Ok(obj) => obj,
                Err(_) => return,
            },
            Err(_) => return,
        };

        let intent = match env.call_method(&pm, "getLaunchIntentForPackage", "(Ljava/lang/String;)Landroid/content/Intent;", &[(&pkg_jstring).into()]) {
            Ok(val) => match val.l() {
                Ok(obj) => obj,
                Err(_) => return,
            },
            Err(_) => return,
        };

        if !intent.as_raw().is_null() {
            log::info!("Launching TV package via Intent: {}", package);
            let _ = env.call_method(&activity, "startActivity", "(Landroid/content/Intent;)V", &[(&intent).into()]);
        } else {
            log::warn!("Package not found or no launch intent: {}", package);
        }
    }
}

#[cfg(target_os = "android")]
fn launch_settings(app: &slint::android::AndroidApp) {
    unsafe {
        let vm_ptr = app.vm_as_ptr() as *mut jni::sys::JavaVM;
        if vm_ptr.is_null() { return; }
        let vm = match jni::JavaVM::from_raw(vm_ptr) {
            Ok(v) => v,
            Err(_) => return,
        };
        let mut env = match vm.attach_current_thread() {
            Ok(e) => e,
            Err(_) => return,
        };
        let activity = jni::objects::JObject::from_raw(app.activity_as_ptr() as jni::sys::jobject);

        let action = match env.new_string("android.settings.SETTINGS") {
            Ok(s) => s,
            Err(_) => return,
        };
        let intent_class = match env.find_class("android/content/Intent") {
            Ok(c) => c,
            Err(_) => return,
        };
        let intent = match env.new_object(&intent_class, "(Ljava/lang/String;)V", &[(&action).into()]) {
            Ok(obj) => obj,
            Err(_) => return,
        };
        log::info!("Launching Android TV System Settings");
        let _ = env.call_method(&activity, "startActivity", "(Landroid/content/Intent;)V", &[(&intent).into()]);
    }
}

#[cfg(not(target_os = "android"))]
fn launch_package(_app: &slint::android::AndroidApp, package: &str) {
    println!("Mock launch package: {}", package);
}

#[cfg(not(target_os = "android"))]
fn launch_settings(_app: &slint::android::AndroidApp) {
    println!("Mock launch settings");
}

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("CleanFrameNative"),
    );
    log::info!("CleanFrame Native Rust starting on Android TV...");

    slint::android::init(app.clone()).expect("Failed to initialize Slint on Android");

    let main_window = MainWindow::new().expect("Failed to create MainWindow");
    let state = Rc::new(RefCell::new(AppState {
        active_zone: "apps",
        selected_app: 3, // Default to Prime Video (matching reference screenshot)
        selected_input: 0,
        selected_util: 0,
        selected_setting: 0,
        settings_open: false,
        wallpaper_idx: 0,
        layout_mode: "List",
    }));

    main_window.set_clock_text("1:54".into());
    main_window.set_selected_app(3);
    main_window.set_wallpaper_idx(0);
    main_window.set_current_layout("List".into());

    let app_handle = Rc::new(app);

    // KEY UP
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_key_up(move || {
            let mut s = state_clone.borrow_mut();
            if s.settings_open {
                if s.selected_setting > 0 {
                    s.selected_setting -= 1;
                }
            } else if s.layout_mode == "Dock" {
                if s.active_zone == "apps" {
                    s.active_zone = "inputs";
                    s.selected_input = 0;
                } else if s.active_zone == "inputs" {
                    s.active_zone = "util";
                    s.selected_util = 0;
                }
            } else {
                // List Mode
                if s.active_zone == "apps" {
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
            }
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    // KEY DOWN
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_key_down(move || {
            let mut s = state_clone.borrow_mut();
            if s.settings_open {
                if s.selected_setting < 7 {
                    s.selected_setting += 1;
                }
            } else if s.layout_mode == "Dock" {
                if s.active_zone == "util" {
                    s.active_zone = "inputs";
                    s.selected_input = 0;
                } else if s.active_zone == "inputs" {
                    s.active_zone = "apps";
                }
            } else {
                // List Mode
                if s.active_zone == "apps" {
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
            }
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    // KEY LEFT
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_key_left(move || {
            let mut s = state_clone.borrow_mut();
            if s.settings_open {
                s.settings_open = false;
            } else if s.layout_mode == "Dock" {
                if s.active_zone == "apps" {
                    if s.selected_app > 0 {
                        s.selected_app -= 1;
                    }
                } else if s.active_zone == "inputs" {
                    if s.selected_input > 0 {
                        s.selected_input -= 1;
                    }
                } else if s.active_zone == "util" {
                    if s.selected_util > 0 {
                        s.selected_util -= 1;
                    }
                }
            } else {
                // List Mode
                if s.active_zone == "inputs" {
                    if s.selected_input > 0 {
                        s.selected_input -= 1;
                    } else {
                        s.active_zone = "apps";
                    }
                } else if s.active_zone == "util" {
                    s.active_zone = "apps";
                }
            }
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    // KEY RIGHT
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_key_right(move || {
            let mut s = state_clone.borrow_mut();
            if s.settings_open {
                // Stay in settings
            } else if s.layout_mode == "Dock" {
                if s.active_zone == "apps" {
                    if s.selected_app < 7 {
                        s.selected_app += 1;
                    }
                } else if s.active_zone == "inputs" {
                    if s.selected_input < 3 {
                        s.selected_input += 1;
                    }
                } else if s.active_zone == "util" {
                    if s.selected_util < 3 {
                        s.selected_util += 1;
                    }
                }
            } else {
                // List Mode
                if s.active_zone == "apps" {
                    if s.selected_app < 4 {
                        s.active_zone = "inputs";
                        s.selected_input = 0;
                    } else {
                        s.active_zone = "util";
                        if s.selected_app >= 6 {
                            s.selected_util = 3; // Direct jump to Settings
                        } else {
                            s.selected_util = 0;
                        }
                    }
                } else if s.active_zone == "inputs" {
                    if s.selected_input < 3 {
                        s.selected_input += 1;
                    }
                }
            }
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    // KEY OK (Action Trigger)
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        let app_ref = app_handle.clone();
        main_window.on_key_ok(move || {
            let mut s = state_clone.borrow_mut();
            if s.settings_open {
                // Interactive Settings options:
                match s.selected_setting {
                    0 => {
                        // Background: cycle through wallpapers
                        s.wallpaper_idx = (s.wallpaper_idx + 1) % WALLPAPER_NAMES.len();
                        log::info!("Switched wallpaper to: {}", WALLPAPER_NAMES[s.wallpaper_idx]);
                    }
                    1 => {
                        // Layout: toggle List <-> Dock
                        s.layout_mode = if s.layout_mode == "List" { "Dock" } else { "List" };
                        log::info!("Switched layout to: {}", s.layout_mode);
                    }
                    5 => {
                        // TV Inputs: close settings and go to inputs shelf
                        s.settings_open = false;
                        s.active_zone = "inputs";
                        s.selected_input = 0;
                    }
                    6 => {
                        // Apps on Home: launch Aurora Store
                        launch_package(&app_ref, "com.aurora.store");
                    }
                    7 => {
                        // Samsung / Home and TV settings: launch system settings!
                        launch_settings(&app_ref);
                    }
                    _ => {
                        log::info!("Settings action on item {}", s.selected_setting);
                    }
                }
            } else if s.active_zone == "apps" {
                if s.selected_app == 7 {
                    // "Settings" in app list: opens CleanFrame Settings sheet!
                    s.settings_open = true;
                    s.selected_setting = 0;
                } else if s.selected_app < APP_PACKAGES.len() {
                    let pkg = APP_PACKAGES[s.selected_app];
                    launch_package(&app_ref, pkg);
                }
            } else if s.active_zone == "inputs" {
                log::info!("HDMI input activated: {}", s.selected_input);
            } else if s.active_zone == "util" {
                match s.selected_util {
                    0 => {
                        // Art mode: cycle wallpaper
                        s.wallpaper_idx = (s.wallpaper_idx + 1) % WALLPAPER_NAMES.len();
                    }
                    1 => {
                        // All apps: launch Aurora Store
                        launch_package(&app_ref, "com.aurora.store");
                    }
                    2 => {
                        // Inputs: switch to inputs shelf
                        s.active_zone = "inputs";
                        s.selected_input = 0;
                    }
                    3 => {
                        // Settings: open settings sheet
                        s.settings_open = true;
                        s.selected_setting = 0;
                    }
                    _ => {}
                }
            }
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    // KEY BACK
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
    win.set_wallpaper_idx(s.wallpaper_idx as i32);
    win.set_current_layout(s.layout_mode.into());

    // Update settings readout values
    let settings = win.get_settings_list();
    if let Some(mut first) = settings.row_data(0) {
        first.value = WALLPAPER_NAMES[s.wallpaper_idx].into();
        settings.set_row_data(0, first);
    }
    if let Some(mut second) = settings.row_data(1) {
        second.value = s.layout_mode.into();
        settings.set_row_data(1, second);
    }
}
