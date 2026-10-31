slint::include_modules!();

mod blur;

use std::rc::Rc;
use std::cell::RefCell;
use slint::{ComponentHandle, Model, VecModel};

pub struct WallpaperMeta {
    pub id: &'static str,
    pub name: &'static str,
    pub credit: &'static str,
    pub desc: &'static str,
}

pub const WALLPAPERS: [WallpaperMeta; 15] = [
    WallpaperMeta { id: "misty", name: "Misty coast", credit: "Ganapathy Kumar · Unsplash", desc: "Pacific coastal mountain ocean shore" },
    WallpaperMeta { id: "cove", name: "Quiet cove", credit: "Joshua Wordel · Unsplash", desc: "Serene ocean cove with gentle surf" },
    WallpaperMeta { id: "forest", name: "Forest mist", credit: "Dejan Zakic · Unsplash", desc: "Deep redwood forest canopy under morning fog" },
    WallpaperMeta { id: "valley", name: "Mountain valley", credit: "Aniket Deole · Unsplash", desc: "Yosemite valley landscape photography" },
    WallpaperMeta { id: "night", name: "Desert stars", credit: "Mohammad Alizade · Unsplash", desc: "Curated astronomy mountain ridge photography" },
    WallpaperMeta { id: "peaks", name: "Blue peaks", credit: "John Rodenn Castillo · Unsplash", desc: "Snow-capped mountain range under clear sky" },
    WallpaperMeta { id: "bridge", name: "Bridge at dusk", credit: "Justin Wolff · Unsplash", desc: "Golden Gate bridge in evening twilight" },
    WallpaperMeta { id: "ridge", name: "Sunlit ridges", credit: "Cristina Gottardi · Unsplash", desc: "Rolling sunlit mountain ridge lines" },
    WallpaperMeta { id: "golden", name: "Golden hour", credit: "Karthik Sreenivas · Unsplash", desc: "Golden hour sunset lighting over peaks" },
    WallpaperMeta { id: "summit", name: "Sunlit summits", credit: "Rudra Gupta · Unsplash", desc: "Alpine summits glowing in dawn light" },
    WallpaperMeta { id: "goldcoast", name: "Gold Coast dusk", credit: "City of Gold Coast · Unsplash", desc: "Coastal sunset reflections on the water" },
    WallpaperMeta { id: "sydney", name: "Sydney Harbour", credit: "Caleb · Unsplash", desc: "Sydney harbour waterfront at twilight" },
    WallpaperMeta { id: "amber", name: "Amber hills", credit: "Chris Barbalis · Unsplash", desc: "Warm amber sunset lighting across hills" },
    WallpaperMeta { id: "mint", name: "Mint wave", credit: "MagicPattern · Unsplash", desc: "Crashing turquoise ocean wave crest" },
    WallpaperMeta { id: "alpine", name: "Alpine glow", credit: "Cristina Gottardi · Unsplash", desc: "Alpenglow mountain peaks at sunrise" },
];

const WALLPAPER_THUMBS: [&[u8]; 15] = [
    include_bytes!("../ui/assets/wallpapers/misty-thumb.jpg"),
    include_bytes!("../ui/assets/wallpapers/cove-thumb.jpg"),
    include_bytes!("../ui/assets/wallpapers/forest-thumb.jpg"),
    include_bytes!("../ui/assets/wallpapers/valley-thumb.jpg"),
    include_bytes!("../ui/assets/wallpapers/night-thumb.jpg"),
    include_bytes!("../ui/assets/wallpapers/peaks-thumb.jpg"),
    include_bytes!("../ui/assets/wallpapers/bridge-thumb.jpg"),
    include_bytes!("../ui/assets/wallpapers/ridge-thumb.jpg"),
    include_bytes!("../ui/assets/wallpapers/golden-thumb.jpg"),
    include_bytes!("../ui/assets/wallpapers/summit-thumb.jpg"),
    include_bytes!("../ui/assets/wallpapers/goldcoast-thumb.jpg"),
    include_bytes!("../ui/assets/wallpapers/sydney-thumb.jpg"),
    include_bytes!("../ui/assets/wallpapers/amber-thumb.jpg"),
    include_bytes!("../ui/assets/wallpapers/mint-thumb.jpg"),
    include_bytes!("../ui/assets/wallpapers/alpine-thumb.jpg"),
];

/// Dynamic on-the-fly blur for ANY custom wallpaper file from disk, gallery, or USB.
#[allow(dead_code)]
pub fn get_frost_for_custom_file(path: &str) -> Option<slint::Image> {
    let bytes = std::fs::read(path).ok()?;
    blur::blur_wallpaper_bytes(&bytes)
}

pub const APP_NAMES: [&str; 8] = [
    "SmartTube",
    "Spotify",
    "Apple TV",
    "Prime Video",
    "Netflix",
    "Aurora Store",
    "Projectivy Launcher",
    "Settings",
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

const HDMI_IDS: [&str; 4] = ["hdmi1", "hdmi2", "hdmi3", "av"];
const HDMI_LABELS: [&str; 4] = ["HDMI 1 · Mac", "HDMI 2", "HDMI 3", "AV"];
const UTIL_LABELS: [&str; 4] = ["Art Mode", "All apps", "Inputs", "Settings"];

#[derive(Default)]
pub struct AppState {
    pub active_zone: &'static str, // "apps" | "inputs" | "util" | "media"
    pub selected_app: usize,
    pub selected_input: usize,
    pub selected_util: usize,
    pub selected_setting: usize,
    pub selected_detail_option: i32, // -1: Back button, 0..N: options
    pub settings_open: bool,
    pub settings_detail: bool,
    pub current_group: &'static str,
    pub wallpaper_idx: usize,
    pub layout_mode: &'static str,
    pub typography: &'static str,
    pub material: &'static str,
    pub focus_style: &'static str,
    pub icon_pack: &'static str,
    pub art_mode: bool,
    pub art_saved_notice: bool,
    pub now_playing_visible: bool,
    pub now_playing_title: String,
    pub now_playing_artist: String,
    pub now_playing_playing: bool,
    pub frost_cache: std::cell::RefCell<std::collections::HashMap<usize, (slint::Image, slint::Image)>>,
}

impl AppState {
    pub fn get_frost_images(&self, idx: usize) -> (slint::Image, slint::Image) {
        let mut cache = self.frost_cache.borrow_mut();
        if let Some(pair) = cache.get(&idx) {
            return pair.clone();
        }
        let bytes = if idx < WALLPAPER_THUMBS.len() {
            WALLPAPER_THUMBS[idx]
        } else {
            WALLPAPER_THUMBS[0]
        };
        if let Some(pair) = blur::blur_wallpaper_bytes_pair(bytes) {
            cache.insert(idx, pair.clone());
            pair
        } else {
            (slint::Image::default(), slint::Image::default())
        }
    }

    pub fn get_frost_image(&self, idx: usize) -> slint::Image {
        self.get_frost_images(idx).0
    }
}

fn get_candidates(package: &str) -> Vec<&str> {
    match package {
        "org.smarttube.stable" => vec![
            "org.smarttube.stable",
            "org.smarttube.beta",
            "com.google.android.youtube.tv",
            "com.google.android.youtube",
            "com.liskovsoft.videomanager",
            "com.teamsmart.videomanager.tv",
        ],
        "com.spotify.tv.android" => vec![
            "com.spotify.tv.android",
            "com.spotify.music",
            "com.spotify.mobile.android.ui",
        ],
        "com.amazon.amazonvideo.livingroom" => vec![
            "com.amazon.amazonvideo.livingroom",
            "com.amazon.avod.thirdpartyclient",
        ],
        "com.netflix.ninja" => vec![
            "com.netflix.ninja",
            "com.netflix.mediaclient",
        ],
        "com.apple.atve.androidtv.appletv" => vec![
            "com.apple.atve.androidtv.appletv",
            "com.apple.android.tv",
            "com.apple.android.music",
        ],
        "com.aurora.store" => vec![
            "com.aurora.store",
            "cm.aptoidetv.pt",
            "com.android.vending",
        ],
        "com.spocky.projengmenu" => vec![
            "com.spocky.projengmenu",
            "com.premium.tv.launcher.ui",
            "com.google.android.tvlauncher",
        ],
        "android.settings.SETTINGS" | "com.android.tv.settings" => vec![
            "com.android.tv.settings",
            "android.settings.SETTINGS",
        ],
        other => vec![other],
    }
}

#[cfg(target_os = "android")]
fn launch_package(app: &slint::android::AndroidApp, package: &str) {
    if package == "android.settings.SETTINGS" || package == "com.android.tv.settings" {
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

        let pm = match env.call_method(&activity, "getPackageManager", "()Landroid/content/pm/PackageManager;", &[]) {
            Ok(val) => match val.l() {
                Ok(obj) => obj,
                Err(_) => return,
            },
            Err(_) => return,
        };

        let candidates = get_candidates(package);
        let mut final_intent = None;
        let mut launched_pkg = "";

        for cand in &candidates {
            let pkg_jstring = match env.new_string(cand) {
                Ok(s) => s,
                Err(_) => continue,
            };

            // 1. Try Leanback Launcher Intent first (required for TV apps like Netflix, Apple TV, SmartTube)
            let mut intent = match env.call_method(&pm, "getLeanbackLaunchIntentForPackage", "(Ljava/lang/String;)Landroid/content/Intent;", &[(&pkg_jstring).into()]) {
                Ok(val) => match val.l() {
                    Ok(obj) => if !obj.as_raw().is_null() { Some(obj) } else { None },
                    Err(_) => None,
                },
                Err(_) => None,
            };

            // 2. Fallback to standard launch intent
            if intent.is_none() {
                intent = match env.call_method(&pm, "getLaunchIntentForPackage", "(Ljava/lang/String;)Landroid/content/Intent;", &[(&pkg_jstring).into()]) {
                    Ok(val) => match val.l() {
                        Ok(obj) => if !obj.as_raw().is_null() { Some(obj) } else { None },
                        Err(_) => None,
                    },
                    Err(_) => None,
                };
            }

            if let Some(intent_obj) = intent {
                final_intent = Some(intent_obj);
                launched_pkg = cand;
                break;
            }
        }

        if let Some(intent_obj) = final_intent {
            let _ = env.call_method(&intent_obj, "addFlags", "(I)Landroid/content/Intent;", &[0x10000000i32.into()]);
            log::info!("Launching TV package: {} (target: {})", launched_pkg, package);
            let _ = env.call_method(&activity, "startActivity", "(Landroid/content/Intent;)V", &[(&intent_obj).into()]);
        } else {
            log::warn!("Package not found on TV: {}, trying fallback launch", package);
            if package.contains("store") || package.contains("aurora") {
                let pkg_jstring = match env.new_string("com.android.vending") {
                    Ok(s) => s,
                    Err(_) => return,
                };
                if let Ok(val) = env.call_method(&pm, "getLaunchIntentForPackage", "(Ljava/lang/String;)Landroid/content/Intent;", &[(&pkg_jstring).into()]) {
                    if let Ok(obj) = val.l() {
                        if !obj.as_raw().is_null() {
                            let _ = env.call_method(&obj, "addFlags", "(I)Landroid/content/Intent;", &[0x10000000i32.into()]);
                            let _ = env.call_method(&activity, "startActivity", "(Landroid/content/Intent;)V", &[(&obj).into()]);
                        }
                    }
                }
            }
        }
    }
}

#[cfg(target_os = "android")]
fn launch_settings_env(env: &mut jni::JNIEnv, activity: &jni::objects::JObject) {
    let action = match env.new_string("android.settings.SETTINGS") {
        Ok(s) => s,
        Err(_) => return,
    };
    let intent_class = match env.find_class("android/content/Intent") {
        Ok(c) => c,
        Err(_) => return,
    };
    if let Ok(intent) = env.new_object(&intent_class, "(Ljava/lang/String;)V", &[(&action).into()]) {
        let _ = env.call_method(&intent, "addFlags", "(I)Landroid/content/Intent;", &[0x10000000i32.into()]);
        log::info!("Launching Android TV System Settings via fallback");
        let _ = env.call_method(activity, "startActivity", "(Landroid/content/Intent;)V", &[(&intent).into()]);
    }
}

#[cfg(target_os = "android")]
fn launch_tv_input(app: &slint::android::AndroidApp, input_id: &str) {
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

        let mut target_id = String::new();
        let target_lower = input_id.to_lowercase();
        let tv_input_service = match env.new_string("tv_input") {
            Ok(s) => s,
            Err(_) => return,
        };
        if let Ok(manager_val) = env.call_method(&activity, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[(&tv_input_service).into()]) {
            if let Ok(manager_obj) = manager_val.l() {
                if !manager_obj.as_raw().is_null() {
                    if let Ok(list_val) = env.call_method(&manager_obj, "getTvInputList", "()Ljava/util/List;", &[]) {
                        if let Ok(list_obj) = list_val.l() {
                            if !list_obj.as_raw().is_null() {
                                if let Ok(size_val) = env.call_method(&list_obj, "size", "()I", &[]) {
                                    let size = size_val.i().unwrap_or(0);

                                    struct InputInfoItem {
                                        id: String,
                                        parent_id: String,
                                        label: String,
                                    }
                                    let mut inputs: Vec<InputInfoItem> = Vec::new();

                                    for i in 0..size {
                                        if let Ok(item_val) = env.call_method(&list_obj, "get", "(I)Ljava/lang/Object;", &[i.into()]) {
                                            if let Ok(item_obj) = item_val.l() {
                                                if let Ok(is_pt) = env.call_method(&item_obj, "isPassthroughInput", "()Z", &[]) {
                                                    if is_pt.z().unwrap_or(false) {
                                                        let id_rust = if let Ok(id_val) = env.call_method(&item_obj, "getId", "()Ljava/lang/String;", &[]) {
                                                            if let Ok(id_obj) = id_val.l() {
                                                                if !id_obj.as_raw().is_null() {
                                                                    env.get_string(&jni::objects::JString::from(id_obj)).map(|s| s.to_string_lossy().to_string()).unwrap_or_default()
                                                                } else { String::new() }
                                                            } else { String::new() }
                                                        } else { String::new() };

                                                        let parent_rust = if let Ok(p_val) = env.call_method(&item_obj, "getParentId", "()Ljava/lang/String;", &[]) {
                                                            if let Ok(p_obj) = p_val.l() {
                                                                if !p_obj.as_raw().is_null() {
                                                                    env.get_string(&jni::objects::JString::from(p_obj)).map(|s| s.to_string_lossy().to_string()).unwrap_or_default()
                                                                } else { String::new() }
                                                            } else { String::new() }
                                                        } else { String::new() };

                                                        let label_rust = if let Ok(l_val) = env.call_method(&item_obj, "loadLabel", "(Landroid/content/Context;)Ljava/lang/CharSequence;", &[(&activity).into()]) {
                                                            if let Ok(l_obj) = l_val.l() {
                                                                if !l_obj.as_raw().is_null() {
                                                                    if let Ok(str_val) = env.call_method(&l_obj, "toString", "()Ljava/lang/String;", &[]) {
                                                                        if let Ok(str_obj) = str_val.l() {
                                                                            if !str_obj.as_raw().is_null() {
                                                                                env.get_string(&jni::objects::JString::from(str_obj)).map(|s| s.to_string_lossy().to_string()).unwrap_or_default()
                                                                            } else { String::new() }
                                                                        } else { String::new() }
                                                                    } else { String::new() }
                                                                } else { String::new() }
                                                            } else { String::new() }
                                                        } else { String::new() };

                                                        log::info!("TV input candidate: id='{}', parent='{}', label='{}'", id_rust, parent_rust, label_rust);
                                                        inputs.push(InputInfoItem {
                                                            id: id_rust,
                                                            parent_id: parent_rust,
                                                            label: label_rust,
                                                        });
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    let is_hdmi1 = target_lower == "hdmi1" || target_lower == "hw1" || target_lower == "hdmi 1";
                                    let is_hdmi2 = target_lower == "hdmi2" || target_lower == "hw2" || target_lower == "hdmi 2";
                                    let is_hdmi3 = target_lower == "hdmi3" || target_lower == "hw3" || target_lower == "hdmi 3";
                                    let is_av = target_lower == "av" || target_lower == "composite";

                                    let mut matched_parent_id = String::new();
                                    for item in &inputs {
                                        let lbl = item.label.to_lowercase();
                                        let id = item.id.to_lowercase();
                                        if is_hdmi1 {
                                            if lbl.contains("hdmi 1") || lbl == "hdmi1" || id.ends_with("/hw15") || id.ends_with("/hw1") || id.ends_with("/hdmi1") || id.contains("port1") || id.contains("port_1") {
                                                matched_parent_id = item.id.clone();
                                                break;
                                            }
                                        } else if is_hdmi2 {
                                            if lbl.contains("hdmi 2") || lbl == "hdmi2" || id.ends_with("/hw16") || id.ends_with("/hw2") || id.ends_with("/hdmi2") || id.contains("port2") || id.contains("port_2") {
                                                matched_parent_id = item.id.clone();
                                                break;
                                            }
                                        } else if is_hdmi3 {
                                            if lbl.contains("hdmi 3") || lbl == "hdmi3" || id.ends_with("/hw17") || id.ends_with("/hdmi3") || id.contains("port3") || id.contains("port_3") {
                                                matched_parent_id = item.id.clone();
                                                break;
                                            }
                                        } else if is_av {
                                            if lbl.contains("av") || lbl.contains("composite") || id.ends_with("/hw3") || id.contains("composite") {
                                                matched_parent_id = item.id.clone();
                                                break;
                                            }
                                        }
                                    }

                                    if !matched_parent_id.is_empty() {
                                        target_id = matched_parent_id.clone();
                                        for item in &inputs {
                                            if item.parent_id == matched_parent_id {
                                                log::info!("Matched CEC child device for port {}: {}", matched_parent_id, item.id);
                                                target_id = item.id.clone();
                                                break;
                                            }
                                        }
                                    } else if is_hdmi1 {
                                        for item in &inputs {
                                            if item.id.to_lowercase().contains("hdmi100004") || item.label.to_lowercase().contains("mac") {
                                                target_id = item.id.clone();
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if target_id.is_empty() {
            target_id = input_id.to_string();
        }

        let action = match env.new_string("android.intent.action.VIEW") {
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

        let uri_class = match env.find_class("android/net/Uri") {
            Ok(c) => c,
            Err(_) => return,
        };
        let target_jstring = match env.new_string(&target_id) {
            Ok(s) => s,
            Err(_) => return,
        };
        let encoded_val = match env.call_static_method(&uri_class, "encode", "(Ljava/lang/String;)Ljava/lang/String;", &[(&target_jstring).into()]) {
            Ok(v) => match v.l() {
                Ok(s) => s,
                Err(_) => return,
            },
            Err(_) => return,
        };
        let encoded_jstring = jni::objects::JString::from(encoded_val);
        let encoded_str = match env.get_string(&encoded_jstring) {
            Ok(s) => s.to_string_lossy().to_string(),
            Err(_) => target_id.clone(),
        };

        let uri_str = format!("content://android.media.tv/passthrough/{}", encoded_str);
        let uri_jstring = match env.new_string(&uri_str) {
            Ok(s) => s,
            Err(_) => return,
        };
        let uri_obj = match env.call_static_method(&uri_class, "parse", "(Ljava/lang/String;)Landroid/net/Uri;", &[(&uri_jstring).into()]) {
            Ok(v) => match v.l() {
                Ok(u) => u,
                Err(_) => return,
            },
            Err(_) => return,
        };

        let _ = env.call_method(&intent, "setData", "(Landroid/net/Uri;)Landroid/content/Intent;", &[(&uri_obj).into()]);
        let _ = env.call_method(&intent, "addFlags", "(I)Landroid/content/Intent;", &[0x10000000i32.into()]);

        let pm = match env.call_method(&activity, "getPackageManager", "()Landroid/content/pm/PackageManager;", &[]) {
            Ok(val) => match val.l() {
                Ok(obj) => obj,
                Err(_) => return,
            },
            Err(_) => return,
        };
        let resolved = match env.call_method(&intent, "resolveActivity", "(Landroid/content/pm/PackageManager;)Landroid/content/ComponentName;", &[(&pm).into()]) {
            Ok(val) => match val.l() {
                Ok(obj) => !obj.as_raw().is_null(),
                Err(_) => false,
            },
            Err(_) => false,
        };

        if resolved {
            log::info!("Launching TV passthrough HDMI input: {} (id: {})", input_id, target_id);
            let _ = env.call_method(&activity, "startActivity", "(Landroid/content/Intent;)V", &[(&intent).into()]);
        } else {
            log::warn!("No TV passthrough receiver found for input: {} - falling back to settings", input_id);
            launch_settings_env(&mut env, &activity);
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

        let pm = match env.call_method(&activity, "getPackageManager", "()Landroid/content/pm/PackageManager;", &[]) {
            Ok(val) => match val.l() {
                Ok(obj) => obj,
                Err(_) => return,
            },
            Err(_) => return,
        };

        // 1. Try Leanback Android TV settings first
        let tv_settings_pkg = match env.new_string("com.android.tv.settings") {
            Ok(s) => s,
            Err(_) => return,
        };
        let mut intent = match env.call_method(&pm, "getLeanbackLaunchIntentForPackage", "(Ljava/lang/String;)Landroid/content/Intent;", &[(&tv_settings_pkg).into()]) {
            Ok(val) => match val.l() {
                Ok(obj) => if !obj.as_raw().is_null() { Some(obj) } else { None },
                Err(_) => None,
            },
            Err(_) => None,
        };

        // 2. Fallback to standard android.settings.SETTINGS action
        if intent.is_none() {
            let action = match env.new_string("android.settings.SETTINGS") {
                Ok(s) => s,
                Err(_) => return,
            };
            let intent_class = match env.find_class("android/content/Intent") {
                Ok(c) => c,
                Err(_) => return,
            };
            intent = match env.new_object(&intent_class, "(Ljava/lang/String;)V", &[(&action).into()]) {
                Ok(obj) => Some(obj),
                Err(_) => None,
            };
        }

        if let Some(intent_obj) = intent {
            let _ = env.call_method(&intent_obj, "addFlags", "(I)Landroid/content/Intent;", &[0x10000000i32.into()]);
            log::info!("Launching Android TV System Settings");
            let _ = env.call_method(&activity, "startActivity", "(Landroid/content/Intent;)V", &[(&intent_obj).into()]);
        }
    }
}

#[cfg(target_os = "android")]
fn launch_home(app: &slint::android::AndroidApp) {
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

        let action = match env.new_string("android.intent.action.MAIN") {
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
        let category = match env.new_string("android.intent.category.HOME") {
            Ok(s) => s,
            Err(_) => return,
        };
        let _ = env.call_method(&intent, "addCategory", "(Ljava/lang/String;)Landroid/content/Intent;", &[(&category).into()]);
        let _ = env.call_method(&intent, "addFlags", "(I)Landroid/content/Intent;", &[0x10000000i32.into()]);
        log::info!("Launching TV Home chooser");
        let _ = env.call_method(&activity, "startActivity", "(Landroid/content/Intent;)V", &[(&intent).into()]);
    }
}

#[cfg(not(target_os = "android"))]
fn launch_package(_app: &slint::android::AndroidApp, package: &str) {
    println!("Mock launch package: {}", package);
}

#[cfg(not(target_os = "android"))]
fn launch_tv_input(_app: &slint::android::AndroidApp, input_id: &str) {
    println!("Mock launch TV input: {}", input_id);
}

#[cfg(not(target_os = "android"))]
fn launch_settings(_app: &slint::android::AndroidApp) {
    println!("Mock launch settings");
}

#[cfg(not(target_os = "android"))]
fn launch_home(_app: &slint::android::AndroidApp) {
    println!("Mock launch home");
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
        selected_detail_option: 0,
        settings_open: false,
        settings_detail: false,
        current_group: "background",
        wallpaper_idx: 6, // Bridge at dusk (matching real TV webview)
        layout_mode: "Dock", // Dock default (matching real TV webview)
        typography: "Manrope",
        material: "Frost",
        focus_style: "Glass, Soft",
        icon_pack: "CleanFrame default",
        art_mode: false,
        art_saved_notice: false,
        now_playing_visible: false,
        now_playing_title: "CleanFrame Ambient".to_string(),
        now_playing_artist: "The Frame Audio".to_string(),
        now_playing_playing: false,
        frost_cache: std::cell::RefCell::new(std::collections::HashMap::new()),
    }));

    main_window.set_clock_text("3:37".into());
    main_window.set_focused_caption("Prime Video".into());
    main_window.set_selected_app(3);
    main_window.set_wallpaper_idx(6);
    let (clean_frost, dock_frost) = state.borrow().get_frost_images(6);
    main_window.set_dynamic_frost_image(clean_frost);
    main_window.set_dock_frost_image(dock_frost);
    main_window.set_current_layout("Dock".into());
    main_window.set_now_playing_visible(false);

    let app_handle = Rc::new(app);

    // Live Clock Timer (Every 5 seconds)
    let clock_timer = slint::Timer::default();
    {
        let window_weak = main_window.as_weak();
        clock_timer.start(
            slint::TimerMode::Repeated,
            std::time::Duration::from_secs(5),
            move || {
                if let Some(win) = window_weak.upgrade() {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs();
                    // Local TV time display (12-hr format)
                    let hours = ((now / 3600) + 5) % 12;
                    let display_hours = if hours == 0 { 12 } else { hours };
                    let minutes = ((now + 1800) / 60) % 60;
                    win.set_clock_text(format!("{}:{:02}", display_hours, minutes).into());
                }
            },
        );
    }

    // KEY UP
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_key_up(move || {
            let mut s = state_clone.borrow_mut();
            if s.art_mode {
                s.art_mode = false;
            } else if s.settings_open {
                if s.settings_detail {
                    if s.selected_detail_option > -1 {
                        s.selected_detail_option -= 1;
                    }
                } else if s.selected_setting > 0 {
                    s.selected_setting -= 1;
                }
            } else if s.layout_mode == "Dock" {
                if s.active_zone == "apps" {
                    s.active_zone = "inputs";
                    s.selected_input = 0;
                } else if s.active_zone == "inputs" {
                    if s.now_playing_visible {
                        s.active_zone = "media";
                    } else {
                        s.active_zone = "util";
                        s.selected_util = 0;
                    }
                } else if s.active_zone == "media" {
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
                } else if s.active_zone == "inputs" {
                    s.active_zone = "media";
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
            if s.art_mode {
                s.art_mode = false;
            } else if s.settings_open {
                if s.settings_detail {
                    let max_opt = get_detail_options_len(s.current_group) as i32 - 1;
                    if s.selected_detail_option < max_opt {
                        s.selected_detail_option += 1;
                    }
                } else if s.selected_setting < 7 {
                    s.selected_setting += 1;
                }
            } else if s.layout_mode == "Dock" {
                if s.active_zone == "util" {
                    if s.now_playing_visible {
                        s.active_zone = "media";
                    } else {
                        s.active_zone = "inputs";
                        s.selected_input = 0;
                    }
                } else if s.active_zone == "media" {
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
                } else if s.active_zone == "media" {
                    s.active_zone = "inputs";
                    s.selected_input = 0;
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
            if s.art_mode {
                s.wallpaper_idx = (s.wallpaper_idx + WALLPAPERS.len() - 1) % WALLPAPERS.len();
                s.art_saved_notice = false;
            } else if s.settings_open {
                if s.settings_detail {
                    s.settings_detail = false;
                } else {
                    s.settings_open = false;
                }
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
                } else if s.active_zone == "util" || s.active_zone == "media" {
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
            if s.art_mode {
                s.wallpaper_idx = (s.wallpaper_idx + 1) % WALLPAPERS.len();
                s.art_saved_notice = false;
            } else if s.settings_open {
                if !s.settings_detail {
                    open_settings_detail(&mut s);
                }
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
                            s.selected_util = 3; // Settings
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
            if s.art_mode {
                // Save current artwork as permanent home wallpaper!
                s.art_saved_notice = true;
            } else if s.settings_open {
                if s.settings_detail {
                    if s.selected_detail_option == -1 {
                        s.settings_detail = false;
                    } else {
                        apply_detail_choice(&mut s, &app_ref);
                    }
                } else {
                    open_settings_detail(&mut s);
                }
            } else if s.active_zone == "apps" {
                if s.selected_app == 7 {
                    s.settings_open = true;
                    s.settings_detail = false;
                    s.selected_setting = 0;
                } else if s.selected_app < APP_PACKAGES.len() {
                    let pkg = APP_PACKAGES[s.selected_app];
                    launch_package(&app_ref, pkg);
                }
            } else if s.active_zone == "inputs" {
                let input_id = HDMI_IDS[s.selected_input];
                launch_tv_input(&app_ref, input_id);
            } else if s.active_zone == "util" {
                match s.selected_util {
                    0 => {
                        // Art Mode: Open ambient gallery view
                        s.art_mode = true;
                        s.art_saved_notice = false;
                    }
                    1 => {
                        // All apps: Open app store / catalog
                        launch_package(&app_ref, "com.aurora.store");
                    }
                    2 => {
                        // Inputs: Focus inputs shelf
                        s.active_zone = "inputs";
                        s.selected_input = 0;
                    }
                    3 => {
                        // Settings: Open settings overview
                        s.settings_open = true;
                        s.settings_detail = false;
                        s.selected_setting = 0;
                    }
                    _ => {}
                }
            } else if s.active_zone == "media" {
                s.now_playing_playing = !s.now_playing_playing;
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
            if s.art_mode {
                s.art_mode = false;
            } else if s.settings_open {
                if s.settings_detail {
                    s.settings_detail = false;
                } else {
                    s.settings_open = false;
                }
            } else if s.active_zone != "apps" {
                s.active_zone = "apps";
            }
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    // Initial UI synchronization
    {
        let s = state.borrow();
        sync_ui(&main_window, &s);
    }

    main_window.run().expect("Slint window run failed");
}

fn open_settings_detail(s: &mut AppState) {
    s.settings_detail = true;
    s.selected_detail_option = 0;
    match s.selected_setting {
        0 => s.current_group = "background",
        1 => s.current_group = "layout",
        2 => s.current_group = "typography",
        3 => s.current_group = "icons",
        4 => s.current_group = "focus",
        5 => s.current_group = "inputs",
        6 => s.current_group = "editor",
        7 => s.current_group = "samsung",
        _ => s.current_group = "background",
    }
}

fn get_detail_options_len(group: &str) -> usize {
    match group {
        "background" => WALLPAPERS.len(),
        "layout" => 2,
        "typography" => 3,
        "icons" => 2,
        "focus" => 4,
        "inputs" => 4,
        "editor" => 1,
        "samsung" => 2,
        _ => 2,
    }
}

fn apply_detail_choice(s: &mut AppState, app: &slint::android::AndroidApp) {
    match s.current_group {
        "background" => {
            if s.selected_detail_option >= 0 && (s.selected_detail_option as usize) < WALLPAPERS.len() {
                s.wallpaper_idx = s.selected_detail_option as usize;
            }
        }
        "layout" => {
            s.layout_mode = if s.selected_detail_option == 0 { "List" } else { "Dock" };
        }
        "typography" => {
            s.typography = match s.selected_detail_option {
                0 => "Manrope",
                1 => "Newsreader",
                _ => "Geist",
            };
        }
        "icons" => {
            s.icon_pack = if s.selected_detail_option == 0 { "CleanFrame default" } else { "Full Color" };
        }
        "focus" => {
            match s.selected_detail_option {
                0 => {
                    s.focus_style = "Glass, Soft";
                    s.material = "Soft";
                }
                1 => {
                    s.focus_style = "Minimal Pill";
                }
                2 => {
                    s.focus_style = "Glass, Soft";
                    s.material = "Frost";
                }
                _ => {
                    s.focus_style = "Glass, Soft";
                    s.material = "Clear tint";
                }
            }
        }
        "inputs" => {
            if s.selected_detail_option >= 0 && (s.selected_detail_option as usize) < HDMI_IDS.len() {
                let id = HDMI_IDS[s.selected_detail_option as usize];
                launch_tv_input(app, id);
            }
        }
        "samsung" => {
            if s.selected_detail_option == 0 {
                launch_settings(app);
            } else {
                launch_home(app);
            }
        }
        "editor" => {
            launch_package(app, "com.aurora.store");
        }
        _ => {}
    }
}

fn sync_ui(win: &MainWindow, s: &AppState) {
    win.set_active_zone(s.active_zone.into());
    win.set_selected_app(s.selected_app as i32);
    win.set_selected_input(s.selected_input as i32);
    win.set_selected_util(s.selected_util as i32);
    win.set_selected_setting(s.selected_setting as i32);
    win.set_selected_detail_option(s.selected_detail_option);
    win.set_settings_open(s.settings_open);
    win.set_settings_detail(s.settings_detail);
    win.set_wallpaper_idx(s.wallpaper_idx as i32);
    let (clean_frost, dock_frost) = s.get_frost_images(s.wallpaper_idx);
    win.set_dynamic_frost_image(clean_frost);
    win.set_dock_frost_image(dock_frost);
    win.set_current_layout(s.layout_mode.into());
    win.set_current_font(s.typography.into());
    win.set_current_material(s.material.into());
    win.set_current_focus_style(s.focus_style.into());
    win.set_current_icon_pack(s.icon_pack.into());
    win.set_now_playing_visible(s.now_playing_visible);

    // Art Mode sync
    win.set_art_mode(s.art_mode);
    if s.wallpaper_idx < WALLPAPERS.len() {
        let meta = &WALLPAPERS[s.wallpaper_idx];
        win.set_art_title(meta.name.into());
        win.set_art_artist(meta.credit.into());
    }
    win.set_art_saved_notice(s.art_saved_notice);

    // Caption under clock reflecting current focused item
    let caption_text = match s.active_zone {
        "apps" => APP_NAMES[s.selected_app.min(APP_NAMES.len() - 1)],
        "inputs" => HDMI_LABELS[s.selected_input.min(HDMI_LABELS.len() - 1)],
        "util" => UTIL_LABELS[s.selected_util.min(UTIL_LABELS.len() - 1)],
        "media" => &s.now_playing_title,
        _ => "",
    };
    win.set_focused_caption(caption_text.into());

    // Update settings overview readout values (8 rows matching real_tv_settings_opened.png)
    let settings = win.get_settings_list();
    if let Some(mut first) = settings.row_data(0) {
        if s.wallpaper_idx < WALLPAPERS.len() {
            first.value = WALLPAPERS[s.wallpaper_idx].name.into();
        }
        settings.set_row_data(0, first);
    }
    if let Some(mut second) = settings.row_data(1) {
        second.value = s.layout_mode.into();
        settings.set_row_data(1, second);
    }
    if let Some(mut third) = settings.row_data(2) {
        third.value = s.typography.into();
        settings.set_row_data(2, third);
    }
    if let Some(mut fourth) = settings.row_data(3) {
        fourth.value = s.icon_pack.into();
        settings.set_row_data(3, fourth);
    }
    if let Some(mut fifth) = settings.row_data(4) {
        fifth.value = s.focus_style.into();
        settings.set_row_data(4, fifth);
    }
    if let Some(mut sixth) = settings.row_data(5) {
        sixth.value = "Device dependent".into();
        settings.set_row_data(5, sixth);
    }
    if let Some(mut seventh) = settings.row_data(6) {
        seventh.value = "8 apps".into();
        settings.set_row_data(6, seventh);
    }
    if let Some(mut eighth) = settings.row_data(7) {
        eighth.value = "Home and TV settings".into();
        settings.set_row_data(7, eighth);
    }

    // Update Detail Submenu Options
    if s.settings_detail {
        match s.current_group {
            "background" => {
                win.set_detail_title("Background".into());
                win.set_detail_desc("Choose a photographic landscape or gallery wallpaper.".into());
                let list = Rc::new(VecModel::default());
                for (idx, w) in WALLPAPERS.iter().enumerate() {
                    list.push(DetailOption {
                        id: w.id.into(),
                        title: w.name.into(),
                        desc: format!("{} · {}", w.desc, w.credit).into(),
                        active: s.wallpaper_idx == idx,
                    });
                }
                win.set_detail_options(list.into());
            }
            "layout" => {
                win.set_detail_title("Layout".into());
                win.set_detail_desc("Choose between vertical list and horizontal liquid glass dock.".into());
                let list = Rc::new(VecModel::default());
                list.push(DetailOption {
                    id: "list".into(),
                    title: "List".into(),
                    desc: "Vertical typography column with quick right-side inputs & utilities".into(),
                    active: s.layout_mode == "List",
                });
                list.push(DetailOption {
                    id: "dock".into(),
                    title: "Dock".into(),
                    desc: "Horizontal liquid glass dock centered at the bottom of the screen".into(),
                    active: s.layout_mode == "Dock",
                });
                win.set_detail_options(list.into());
            }
            "typography" => {
                win.set_detail_title("Typography".into());
                win.set_detail_desc("Select typography styling for titles, clocks and labels.".into());
                let list = Rc::new(VecModel::default());
                list.push(DetailOption {
                    id: "manrope".into(),
                    title: "Manrope".into(),
                    desc: "Clean geometric modern sans-serif".into(),
                    active: s.typography == "Manrope",
                });
                list.push(DetailOption {
                    id: "newsreader".into(),
                    title: "Newsreader".into(),
                    desc: "Editorial optical-size serif typeface".into(),
                    active: s.typography == "Newsreader",
                });
                list.push(DetailOption {
                    id: "geist".into(),
                    title: "Geist".into(),
                    desc: "Precise modernist architectural grotesque".into(),
                    active: s.typography == "Geist",
                });
                win.set_detail_options(list.into());
            }
            "icons" => {
                win.set_detail_title("Icon Pack".into());
                win.set_detail_desc("Choose icon rendering aesthetics for app shortcuts.".into());
                let list = Rc::new(VecModel::default());
                list.push(DetailOption {
                    id: "mono".into(),
                    title: "CleanFrame default".into(),
                    desc: "Minimal white brand vector silhouettes with frosted glass contrast".into(),
                    active: s.icon_pack == "CleanFrame default",
                });
                list.push(DetailOption {
                    id: "color".into(),
                    title: "Full Color".into(),
                    desc: "Original brand colors on glass plates".into(),
                    active: s.icon_pack == "Full Color",
                });
                win.set_detail_options(list.into());
            }
            "focus" => {
                win.set_detail_title("Focus & Material".into());
                win.set_detail_desc("Configure highlight plate treatment and glass translucency.".into());
                let list = Rc::new(VecModel::default());
                list.push(DetailOption {
                    id: "glass_soft".into(),
                    title: "Glass, Soft".into(),
                    desc: "Liquid specular gradient with beveled glass glow and top rim".into(),
                    active: s.focus_style == "Glass, Soft" && s.material == "Soft",
                });
                list.push(DetailOption {
                    id: "minimal".into(),
                    title: "Minimal Pill".into(),
                    desc: "Clean understated outline with subtle frosted fill".into(),
                    active: s.focus_style == "Minimal Pill",
                });
                list.push(DetailOption {
                    id: "frost".into(),
                    title: "Frost".into(),
                    desc: "Liquid specular frosted glass with balanced scrim contrast".into(),
                    active: s.focus_style == "Glass, Soft" && s.material == "Frost",
                });
                list.push(DetailOption {
                    id: "tint".into(),
                    title: "Clear tint".into(),
                    desc: "Deep atmospheric obsidian tint for high-contrast legible text".into(),
                    active: s.focus_style == "Glass, Soft" && s.material == "Clear tint",
                });
                win.set_detail_options(list.into());
            }
            "inputs" => {
                win.set_detail_title("TV inputs".into());
                win.set_detail_desc("Choose a connected HDMI port to switch passthrough source.".into());
                let list = Rc::new(VecModel::default());
                list.push(DetailOption {
                    id: "hdmi1".into(),
                    title: "HDMI 1 · Mac".into(),
                    desc: "Connected · 4K 60Hz CEC Source".into(),
                    active: true,
                });
                list.push(DetailOption {
                    id: "hdmi2".into(),
                    title: "HDMI 2".into(),
                    desc: "No signal · Passthrough port".into(),
                    active: false,
                });
                list.push(DetailOption {
                    id: "hdmi3".into(),
                    title: "HDMI 3".into(),
                    desc: "Connected · Passthrough port".into(),
                    active: true,
                });
                list.push(DetailOption {
                    id: "av".into(),
                    title: "AV".into(),
                    desc: "No signal · Composite input".into(),
                    active: false,
                });
                win.set_detail_options(list.into());
            }
            "samsung" => {
                win.set_detail_title("Samsung & TV Settings".into());
                win.set_detail_desc("Launch external system settings and TV controls.".into());
                let list = Rc::new(VecModel::default());
                list.push(DetailOption {
                    id: "tv_settings".into(),
                    title: "Android TV Settings".into(),
                    desc: "Picture, display, sound, network, and device preferences".into(),
                    active: false,
                });
                list.push(DetailOption {
                    id: "tv_home".into(),
                    title: "Original TV Home".into(),
                    desc: "Return to default TV system launcher".into(),
                    active: false,
                });
                win.set_detail_options(list.into());
            }
            "editor" => {
                win.set_detail_title("Apps on Home".into());
                win.set_detail_desc("Manage installed TV applications and app store.".into());
                let list = Rc::new(VecModel::default());
                list.push(DetailOption {
                    id: "app_store".into(),
                    title: "Open App Store / Aurora".into(),
                    desc: "Discover, install and update Android TV apps".into(),
                    active: true,
                });
                win.set_detail_options(list.into());
            }
            _ => {
                win.set_detail_title("Settings".into());
                win.set_detail_desc("Configure launcher appearance and behaviors.".into());
            }
        }
    }
}
