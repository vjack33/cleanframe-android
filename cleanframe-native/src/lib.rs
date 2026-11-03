slint::include_modules!();

mod blur;
mod display4k;

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

const WALLPAPER_FULL: [&[u8]; 15] = [
    include_bytes!("../ui/assets/wallpapers/misty.jpg"),
    include_bytes!("../ui/assets/wallpapers/cove.jpg"),
    include_bytes!("../ui/assets/wallpapers/forest.jpg"),
    include_bytes!("../ui/assets/wallpapers/valley.jpg"),
    include_bytes!("../ui/assets/wallpapers/night.jpg"),
    include_bytes!("../ui/assets/wallpapers/peaks.jpg"),
    include_bytes!("../ui/assets/wallpapers/bridge.jpg"),
    include_bytes!("../ui/assets/wallpapers/ridge.jpg"),
    include_bytes!("../ui/assets/wallpapers/golden.jpg"),
    include_bytes!("../ui/assets/wallpapers/summit.jpg"),
    include_bytes!("../ui/assets/wallpapers/goldcoast.jpg"),
    include_bytes!("../ui/assets/wallpapers/sydney.jpg"),
    include_bytes!("../ui/assets/wallpapers/amber.jpg"),
    include_bytes!("../ui/assets/wallpapers/mint.jpg"),
    include_bytes!("../ui/assets/wallpapers/alpine.jpg"),
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

#[allow(dead_code)]
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

pub const BLUR_LEVEL_NAMES: [&str; 4] = ["Off", "Subtle", "Default", "Strong"];
pub const BLUR_LEVEL_DESCS: [&str; 4] = [
    "No blur · Crystal-clear transparent glass substrate",
    "Subtle diffusion · Light optical frost",
    "Default diffusion · Balanced silky frosted glass",
    "Strong diffusion · Deep frosted atmospheric glass",
];
pub const BLUR_RADII: [usize; 4] = [0, 16, 40, 75];

pub const REFRACTION_LEVEL_NAMES: [&str; 4] = ["Off", "Subtle", "Default", "Strong"];
pub const REFRACTION_LEVEL_DESCS: [&str; 4] = [
    "No refraction · Flat borderless glass substrate",
    "Subtle curvature · Gentle liquid bevel bend",
    "Default curvature · Balanced optical liquid glass bevel",
    "Strong curvature · Pronounced fluid lens edge refraction",
];
pub const REFRACTION_DISTANCES: [f32; 4] = [0.0, 14.0, 28.0, 48.0];

pub const FOCUS_REFRACTION_NAMES: [&str; 4] = ["Off", "Subtle", "Default", "Strong"];
pub const FOCUS_REFRACTION_DESCS: [&str; 4] = [
    "No edge refraction · Pure flat glass plate",
    "Subtle lens bend · Gentle optical deflection on bevel",
    "Default refraction · Balanced Apple Liquid Glass prism bevel",
    "Strong refraction · Deep refractive lens with chromatic dispersion",
];
pub const FOCUS_REFRACTION_DISTANCES: [f32; 4] = [0.0, 12.0, 26.0, 48.0];

pub const FOCUS_BLUR_NAMES: [&str; 4] = ["Off", "Subtle", "Default", "Strong"];
pub const FOCUS_BLUR_DESCS: [&str; 4] = [
    "Crystal clear · High-transparency optical substrate",
    "Subtle frost · Gentle micro-diffusion glow",
    "Default frost · Balanced silky frosted glass diffusion",
    "Strong frost · Deep atmospheric frosted glass diffusion",
];
pub const FOCUS_BLUR_RADII: [f32; 4] = [0.0, 8.0, 18.0, 32.0];

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
    pub refraction: f32, // Default: 1.00 (Apple liquid-rust demo default)
    pub appearance_preset: &'static str, // Default: "Clear" (Apple kit Clear glass)
    pub transparency: f32, // Default: 0.00 (Clear)
    pub dark_mode: bool,
    pub increase_contrast: bool,
    pub reduce_motion: bool,
    pub reduce_transparency: bool,
    pub physics: bool,
    pub liquid_merge: bool,
    pub in_appearance_panel: bool,
    pub appearance_focus_index: i32,
    pub blur_level: usize,
    pub layout_mode: &'static str,
    pub typography: &'static str,
    pub material: &'static str,
    pub focus_style: &'static str,
    pub icon_pack: String,
    pub installed_icon_packs: Vec<(String, String)>,
    pub icon_pack_mappings: std::cell::RefCell<std::collections::HashMap<String, std::collections::HashMap<String, String>>>,
    pub stock_vector_icons: std::collections::HashMap<String, slint::Image>,
    pub app_icon_cache: std::cell::RefCell<std::collections::HashMap<String, slint::Image>>,
    pub art_mode: bool,
    pub art_saved_notice: bool,
    pub now_playing_visible: bool,
    pub now_playing_title: String,
    pub now_playing_artist: String,
    pub now_playing_playing: bool,
    pub all_apps_open: bool,
    pub selected_all_app: usize,
    pub all_apps_action_open: bool,
    pub selected_all_app_action: usize,
    pub dock_indices: Vec<usize>,
    pub detail_back_text: &'static str,
    pub base_blur_cache: std::cell::RefCell<std::collections::HashMap<(usize, usize), (Vec<u8>, slint::Image)>>,
    pub dock_frost_cache: std::cell::RefCell<Option<(usize, usize, u32, slint::Image, slint::Image)>>,
    pub focus_lens_cache: std::cell::RefCell<Option<(slint::Image, slint::Image, slint::Image)>>,
}

impl AppState {
    pub fn get_frost_images(&self, idx: usize) -> (slint::Image, slint::Image) {
        let radius = if self.reduce_transparency {
            28
        } else if self.appearance_preset == "Clear" {
            8
        } else if self.appearance_preset == "Regular" {
            16
        } else {
            12
        };
        let ref_dist = self.refraction * 48.0;
        let ref_key = (ref_dist * 1000.0) as u32;

        let mut dock_cache = self.dock_frost_cache.borrow_mut();
        if let Some((c_idx, c_rad, c_ref, ref clean_img, ref dock_img)) = *dock_cache {
            if c_idx == idx && c_rad == radius && c_ref == ref_key {
                return (clean_img.clone(), dock_img.clone());
            }
        }

        // Fast base wallpaper lookup or computation (runs once per wallpaper/radius)
        let mut base_cache = self.base_blur_cache.borrow_mut();
        let (pixels, clean_img) = if let Some(entry) = base_cache.get(&(idx, radius)) {
            (entry.0.clone(), entry.1.clone())
        } else {
            let bytes = if idx < WALLPAPER_FULL.len() {
                WALLPAPER_FULL[idx]
            } else {
                WALLPAPER_FULL[0]
            };
            if let Some((p, img)) = blur::compute_base_blurred_wallpaper(bytes, radius) {
                base_cache.insert((idx, radius), (p.clone(), img.clone()));
                (p, img)
            } else {
                return (slint::Image::default(), slint::Image::default());
            }
        };

        // Instant optical Snell refraction applied directly on pre-blurred buffer (< 1ms CPU)
        let dock_img = blur::compute_refracted_dock_image(&pixels, ref_dist);
        *dock_cache = Some((idx, radius, ref_key, clean_img.clone(), dock_img.clone()));
        (clean_img, dock_img)
    }

    pub fn get_frost_image(&self, idx: usize) -> slint::Image {
        self.get_frost_images(idx).0
    }

    pub fn get_focus_lens_images(&self) -> (slint::Image, slint::Image, slint::Image) {
        let mut cache = self.focus_lens_cache.borrow_mut();
        if let Some(tuple) = cache.as_ref() {
            return tuple.clone();
        }
        let ref_dist = self.refraction * 48.0;
        let blur_r = if self.reduce_transparency {
            26.0
        } else if self.appearance_preset == "Clear" {
            6.0
        } else {
            14.0
        };
        // Dock squircle lens: 132x92 at 2x = 264x184, radius = 40.0
        let dock_lens = blur::render_focus_lens_texture(264, 184, 40.0, ref_dist, blur_r, self.appearance_preset);
        // List capsule lens: 260x48 at 2x = 520x96, radius = 48.0 (capsule)
        let list_lens = blur::render_focus_lens_texture(520, 96, 48.0, ref_dist * 0.85, blur_r * 0.85, self.appearance_preset);
        // Round circular lens: 92x92 at 2x = 184x184, radius = 82.0 (circle)
        let round_lens = blur::render_round_focus_lens_texture(184, 82.0, ref_dist, blur_r, self.appearance_preset);
        let tuple = (dock_lens, list_lens, round_lens);
        *cache = Some(tuple.clone());
        tuple
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
            log::warn!("Package not found on TV: {}, launching store search", package);
            let search_term = match package {
                p if p.contains("spotify") => "spotify",
                p if p.contains("amazon") || p.contains("prime") => "prime video",
                p if p.contains("netflix") => "netflix",
                p if p.contains("apple") => "apple tv",
                p if p.contains("youtube") || p.contains("smarttube") => "youtube",
                p if p.contains("plex") => "plex",
                p if p.contains("jellyfin") => "jellyfin",
                p if p.contains("disney") => "disney+",
                p if p.contains("tubi") => "tubi",
                p if p.contains("stremio") => "stremio",
                p if p.contains("store") || p.contains("aurora") => "aurora store",
                other => other.split('.').last().unwrap_or(other),
            };
            launch_store_search(app, search_term);
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
fn query_installed_icon_packs(app: &slint::android::AndroidApp) -> Vec<(String, String)> {
    let mut result = Vec::new();
    unsafe {
        let vm_ptr = app.vm_as_ptr() as *mut jni::sys::JavaVM;
        if vm_ptr.is_null() { return result; }
        let vm = match jni::JavaVM::from_raw(vm_ptr) {
            Ok(v) => v,
            Err(_) => return result,
        };
        let mut env = match vm.attach_current_thread() {
            Ok(e) => e,
            Err(_) => return result,
        };
        let activity = jni::objects::JObject::from_raw(app.activity_as_ptr() as jni::sys::jobject);

        let pm = match env.call_method(&activity, "getPackageManager", "()Landroid/content/pm/PackageManager;", &[]) {
            Ok(val) => match val.l() {
                Ok(obj) => obj,
                Err(_) => return result,
            },
            Err(_) => return result,
        };

        let intent_class = match env.find_class("android/content/Intent") {
            Ok(c) => c,
            Err(_) => return result,
        };

        let mut seen = std::collections::HashSet::new();
        let actions = [
            "org.adw.launcher.THEMES",
            "com.novalauncher.THEME",
            "com.teslacoilsw.launcher.THEME",
            "com.anddoes.launcher.THEME",
            "com.gau.go.launcherex.theme",
            "com.fede.launcher.THEME_ICONPACK",
            "com.dlto.atom.launcher.THEME",
            "ginlemon.smartlauncher.THEMES",
            "com.sonymobile.home.ICON_PACK",
            "app.lawnchair.icons.THEME",
            "ch.deletescape.lawnchair.ICONPACK",
        ];

        for action in actions {
            if let Ok(act_str) = env.new_string(action) {
                if let Ok(intent_obj) = env.new_object(&intent_class, "(Ljava/lang/String;)V", &[(&act_str).into()]) {
                    if let Ok(list_val) = env.call_method(&pm, "queryIntentActivities", "(Landroid/content/Intent;I)Ljava/util/List;", &[(&intent_obj).into(), 0i32.into()]) {
                        if let Ok(list_obj) = list_val.l() {
                            if !list_obj.as_raw().is_null() {
                                if let Ok(size_val) = env.call_method(&list_obj, "size", "()I", &[]) {
                                    let size = size_val.i().unwrap_or(0);
                                    for i in 0..size {
                                        if let Ok(item_val) = env.call_method(&list_obj, "get", "(I)Ljava/lang/Object;", &[i.into()]) {
                                            if let Ok(resolve_info) = item_val.l() {
                                                if let Ok(act_info_val) = env.get_field(&resolve_info, "activityInfo", "Landroid/content/pm/ActivityInfo;") {
                                                    if let Ok(act_info) = act_info_val.l() {
                                                        if let Ok(pkg_val) = env.get_field(&act_info, "packageName", "Ljava/lang/String;") {
                                                            if let Ok(pkg_obj) = pkg_val.l() {
                                                                let pkg_opt = {
                                                                    let pkg_jstring = jni::objects::JString::from(pkg_obj);
                                                                    env.get_string(&pkg_jstring).map(|s| s.to_string_lossy().to_string()).ok()
                                                                };
                                                                if let Some(pkg) = pkg_opt {
                                                                    if pkg != "com.cleanframe.tv" && !seen.contains(&pkg) {
                                                                        seen.insert(pkg.clone());
                                                                        let mut label = pkg.clone();
                                                                        if let Ok(label_val) = env.call_method(&resolve_info, "loadLabel", "(Landroid/content/pm/PackageManager;)Ljava/lang/CharSequence;", &[(&pm).into()]) {
                                                                            if let Ok(label_obj) = label_val.l() {
                                                                                if !label_obj.as_raw().is_null() {
                                                                                    if let Ok(s_val) = env.call_method(&label_obj, "toString", "()Ljava/lang/String;", &[]) {
                                                                                        if let Ok(s_obj) = s_val.l() {
                                                                                            let lbl_opt = {
                                                                                                let s_j = jni::objects::JString::from(s_obj);
                                                                                                env.get_string(&s_j).map(|s| s.to_string_lossy().to_string()).ok()
                                                                                            };
                                                                                            if let Some(lbl) = lbl_opt {
                                                                                                label = lbl;
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        result.push((pkg, label));
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
                            }
                        }
                    }
                }
            }
        }
    }
    result
}

#[cfg(not(target_os = "android"))]
fn query_installed_icon_packs(_app: &slint::android::AndroidApp) -> Vec<(String, String)> {
    vec![
        ("com.whicons.iconpack".to_string(), "Whicons (White Icons)".to_string()),
        ("com.delta.iconpack".to_string(), "Delta (Pastel Icons)".to_string()),
    ]
}

fn extract_xml_attr(part: &str, attr: &str) -> Option<String> {
    for quote in ['"', '\''] {
        let pattern_space = format!("{} = {}", attr, quote);
        let pattern = format!("{}={}", attr, quote);
        let start = if let Some(idx) = part.find(&pattern) {
            Some(idx + pattern.len())
        } else if let Some(idx) = part.find(&pattern_space) {
            Some(idx + pattern_space.len())
        } else {
            None
        };
        if let Some(s) = start {
            let rest = &part[s..];
            if let Some(end) = rest.find(quote) {
                return Some(rest[..end].to_string());
            }
        }
    }
    None
}

fn parse_appfilter_xml_str(text: &str, map: &mut std::collections::HashMap<String, String>) {
    for part in text.split('<') {
        if part.starts_with("item ") {
            let comp_opt = extract_xml_attr(part, "component");
            let draw_opt = extract_xml_attr(part, "drawable");
            if let (Some(comp_raw), Some(draw)) = (comp_opt, draw_opt) {
                let mut comp = comp_raw.trim();
                if comp.starts_with("ComponentInfo{") && comp.ends_with('}') {
                    comp = &comp[14..comp.len() - 1];
                }
                let comp_clean = comp.trim().to_lowercase();
                if let Some((pkg, _act)) = comp_clean.split_once('/') {
                    map.insert(comp_clean.clone(), draw.clone());
                    map.entry(pkg.to_string()).or_insert_with(|| draw.clone());
                } else {
                    map.insert(comp_clean, draw);
                }
            }
        }
    }
}

#[cfg(target_os = "android")]
fn parse_xml_resource_parser(
    env: &mut jni::JNIEnv,
    parser: &jni::objects::JObject,
    map: &mut std::collections::HashMap<String, String>,
) {
    loop {
        let event_val = match env.call_method(parser, "next", "()I", &[]) {
            Ok(v) => v.i().unwrap_or(1),
            Err(_) => break,
        };
        // 1 == END_DOCUMENT
        if event_val == 1 { break; }
        // 2 == START_TAG
        if event_val == 2 {
            if let Ok(name_val) = env.call_method(parser, "getName", "()Ljava/lang/String;", &[]) {
                if let Ok(name_obj) = name_val.l() {
                    let jname = jni::objects::JString::from(name_obj);
                    let tag_name = env.get_string(&jname).map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
                    if tag_name == "item" {
                        let count_val = env.call_method(parser, "getAttributeCount", "()I", &[]);
                        let count = count_val.map(|v| v.i().unwrap_or(0)).unwrap_or(0);
                        let mut comp_opt: Option<String> = None;
                        let mut draw_opt: Option<String> = None;
                        for i in 0..count {
                            if let Ok(name_val) = env.call_method(parser, "getAttributeName", "(I)Ljava/lang/String;", &[i.into()]) {
                                if let Ok(name_obj) = name_val.l() {
                                    if !name_obj.as_raw().is_null() {
                                        let attr_str = {
                                            let jn = jni::objects::JString::from(name_obj);
                                            env.get_string(&jn).map(|s| s.to_string_lossy().to_string()).unwrap_or_default()
                                        };
                                        if attr_str == "component" {
                                            if let Ok(v_val) = env.call_method(parser, "getAttributeValue", "(I)Ljava/lang/String;", &[i.into()]) {
                                                if let Ok(v_obj) = v_val.l() {
                                                    if !v_obj.as_raw().is_null() {
                                                        let jv = jni::objects::JString::from(v_obj);
                                                        comp_opt = env.get_string(&jv).map(|s| s.to_string_lossy().to_string()).ok();
                                                    }
                                                }
                                            }
                                        } else if attr_str == "drawable" {
                                            if let Ok(v_val) = env.call_method(parser, "getAttributeValue", "(I)Ljava/lang/String;", &[i.into()]) {
                                                if let Ok(v_obj) = v_val.l() {
                                                    if !v_obj.as_raw().is_null() {
                                                        let jv = jni::objects::JString::from(v_obj);
                                                        draw_opt = env.get_string(&jv).map(|s| s.to_string_lossy().to_string()).ok();
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if let (Some(comp_raw), Some(draw)) = (comp_opt, draw_opt) {
                            let mut comp = comp_raw.trim();
                            if comp.starts_with("ComponentInfo{") && comp.ends_with('}') {
                                comp = &comp[14..comp.len() - 1];
                            }
                            let comp_clean = comp.trim().to_lowercase();
                            if let Some((pkg, _act)) = comp_clean.split_once('/') {
                                map.insert(comp_clean.clone(), draw.clone());
                                map.entry(pkg.to_string()).or_insert_with(|| draw.clone());
                            } else {
                                map.insert(comp_clean, draw);
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(target_os = "android")]
fn load_icon_pack_appfilter(app: &slint::android::AndroidApp, pack_pkg: &str) -> std::collections::HashMap<String, String> {
    let mut map = std::collections::HashMap::new();
    unsafe {
        let vm_ptr = app.vm_as_ptr() as *mut jni::sys::JavaVM;
        if vm_ptr.is_null() { return map; }
        let vm = match jni::JavaVM::from_raw(vm_ptr) {
            Ok(v) => v,
            Err(_) => return map,
        };
        let mut env = match vm.attach_current_thread() {
            Ok(e) => e,
            Err(_) => return map,
        };
        let activity = jni::objects::JObject::from_raw(app.activity_as_ptr() as jni::sys::jobject);

        let pack_jstr = match env.new_string(pack_pkg) {
            Ok(s) => s,
            Err(_) => return map,
        };

        // Try 1: Open from Assets (assets/appfilter.xml, etc.)
        let mut loaded = false;
        if let Ok(ctx_val) = env.call_method(&activity, "createPackageContext", "(Ljava/lang/String;I)Landroid/content/Context;", &[(&pack_jstr).into(), 2i32.into()]) {
            if let Ok(pack_ctx) = ctx_val.l() {
                if !pack_ctx.as_raw().is_null() {
                    if let Ok(assets_val) = env.call_method(&pack_ctx, "getAssets", "()Landroid/content/res/AssetManager;", &[]) {
                        if let Ok(assets) = assets_val.l() {
                            for fname in ["appfilter.xml", "appfilter_theme.xml", "theme_resources.xml", "appmap.xml"] {
                                if let Ok(fn_jstr) = env.new_string(fname) {
                                    if let Ok(is_val) = env.call_method(&assets, "open", "(Ljava/lang/String;)Ljava/io/InputStream;", &[(&fn_jstr).into()]) {
                                        if let Ok(is) = is_val.l() {
                                            if !is.as_raw().is_null() {
                                                let chunk_size = 32768;
                                                if let Ok(jbyte_arr) = env.new_byte_array(chunk_size) {
                                                    let mut buf = Vec::new();
                                                    let mut temp_buf = vec![0i8; chunk_size as usize];
                                                    loop {
                                                        if let Ok(n_val) = env.call_method(&is, "read", "([B)I", &[(&jbyte_arr).into()]) {
                                                            let n = n_val.i().unwrap_or(-1);
                                                            if n <= 0 { break; }
                                                            if env.get_byte_array_region(&jbyte_arr, 0, &mut temp_buf[..n as usize]).is_ok() {
                                                                let u8_slice: &[u8] = std::slice::from_raw_parts(temp_buf.as_ptr() as *const u8, n as usize);
                                                                buf.extend_from_slice(u8_slice);
                                                            } else {
                                                                break;
                                                            }
                                                        } else {
                                                            break;
                                                        }
                                                    }
                                                    let _ = env.call_method(&is, "close", "()V", &[]);
                                                    if !buf.is_empty() {
                                                        let text = String::from_utf8_lossy(&buf);
                                                        parse_appfilter_xml_str(&text, &mut map);
                                                        if !map.is_empty() {
                                                            loaded = true;
                                                            log::info!("Loaded {} appfilter mappings from assets/{} for {}", map.len(), fname, pack_pkg);
                                                            break;
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            let _ = env.exception_clear();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                let _ = env.exception_clear();
            }
        } else {
            let _ = env.exception_clear();
        }

        if loaded {
            return map;
        }

        // Try 2: res/xml/appfilter (compiled XML in res/xml/)
        if let Ok(pm_val) = env.call_method(&activity, "getPackageManager", "()Landroid/content/pm/PackageManager;", &[]) {
            if let Ok(pm) = pm_val.l() {
                if let Ok(res_val) = env.call_method(&pm, "getResourcesForApplication", "(Ljava/lang/String;)Landroid/content/res/Resources;", &[(&pack_jstr).into()]) {
                    if let Ok(res) = res_val.l() {
                        if let Ok(xml_type) = env.new_string("xml") {
                            for xml_name in ["appfilter", "appfilter_theme", "theme_resources", "appmap"] {
                                if let Ok(name_jstr) = env.new_string(xml_name) {
                                    if let Ok(id_val) = env.call_method(&res, "getIdentifier", "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)I", &[(&name_jstr).into(), (&xml_type).into(), (&pack_jstr).into()]) {
                                        let res_id = id_val.i().unwrap_or(0);
                                        if res_id != 0 {
                                            if let Ok(parser_val) = env.call_method(&res, "getXml", "(I)Landroid/content/res/XmlResourceParser;", &[res_id.into()]) {
                                                if let Ok(parser) = parser_val.l() {
                                                    if !parser.as_raw().is_null() {
                                                        parse_xml_resource_parser(&mut env, &parser, &mut map);
                                                        let _ = env.call_method(&parser, "close", "()V", &[]);
                                                        if !map.is_empty() {
                                                            log::info!("Loaded {} appfilter mappings from res/xml/{} for {}", map.len(), xml_name, pack_pkg);
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
                }
            }
        }
    }
    map
}

#[cfg(not(target_os = "android"))]
fn load_icon_pack_appfilter(_app: &slint::android::AndroidApp, _pack_pkg: &str) -> std::collections::HashMap<String, String> {
    std::collections::HashMap::new()
}

#[cfg(target_os = "android")]
fn load_app_icon_image(
    app: &slint::android::AndroidApp,
    item_id: &str,
    pkg: &str,
    app_name: &str,
    icon_pack_pkg: Option<&str>,
    appfilter: Option<&std::collections::HashMap<String, String>>,
) -> Option<slint::Image> {
    unsafe {
        let vm_ptr = app.vm_as_ptr() as *mut jni::sys::JavaVM;
        if vm_ptr.is_null() { return None; }
        let vm = jni::JavaVM::from_raw(vm_ptr).ok()?;
        let mut env = vm.attach_current_thread().ok()?;
        let activity = jni::objects::JObject::from_raw(app.activity_as_ptr() as jni::sys::jobject);

        let pm_val = env.call_method(&activity, "getPackageManager", "()Landroid/content/pm/PackageManager;", &[]).ok()?;
        let pm = pm_val.l().ok()?;

        let mut drawable_opt: Option<jni::objects::JObject> = None;

        // 1. Try loading from external icon pack if specified
        if let Some(pack_pkg) = icon_pack_pkg {
            if let Ok(pack_str) = env.new_string(pack_pkg) {
                if let Ok(res_val) = env.call_method(&pm, "getResourcesForApplication", "(Ljava/lang/String;)Landroid/content/res/Resources;", &[(&pack_str).into()]) {
                    if let Ok(res) = res_val.l() {
                        let mut candidates: Vec<String> = Vec::new();

                        // A. Check appfilter.xml mappings
                        if let Some(map) = appfilter {
                            let pkg_lower = pkg.to_lowercase();
                            if let Some(target) = map.get(&pkg_lower) {
                                candidates.push(target.clone());
                            }
                            // Canonical TV-to-Mobile aliases
                            let alias = match pkg {
                                "org.smarttube.stable" | "com.liskovsoft.smarttubetv.beta" | "com.liskovsoft.videomanager" => Some("com.google.android.youtube"),
                                "com.spotify.tv.android" => Some("com.spotify.music"),
                                "com.netflix.ninja" => Some("com.netflix.mediaclient"),
                                "com.amazon.amazonvideo.livingroom" => Some("com.amazon.avod.thirdpartyclient"),
                                "com.apple.atve.androidtv.appletv" => Some("com.apple.android.tv"),
                                "android.settings.SETTINGS" | "settings" => Some("com.android.settings"),
                                "cm.aptoidetv.pt" => Some("cm.aptoide.pt"),
                                "org.jellyfin.androidtv" => Some("org.jellyfin.mobile"),
                                _ => None,
                            };
                            if let Some(al) = alias {
                                if let Some(target) = map.get(al) {
                                    if !candidates.contains(target) {
                                        candidates.push(target.clone());
                                    }
                                }
                            }
                        }

                        // B. Smart known aliases based on item_id
                        let id_aliases: &[&str] = match item_id {
                            "youtube" => &["youtube", "youtube_tv", "smarttube", "ic_youtube"],
                            "spotify" => &["spotify", "spotify_music", "ic_spotify"],
                            "appletv" => &["apple_tv", "appletv", "apple", "ic_apple_tv"],
                            "primevideo" => &["amazon_prime_video", "prime_video", "amazon_video", "primevideo", "amazon_prime"],
                            "netflix" => &["netflix", "ic_netflix"],
                            "aurora" => &["aurora_store", "aurora", "aurorastore"],
                            "plex" => &["plex", "plexamp"],
                            "settings" => &["settings", "generic_settings", "preferences", "tv_settings"],
                            "jellyfin" => &["jellyfin"],
                            "tubi" => &["tubi", "tubi_tv"],
                            "disneyplus" => &["disney_plus", "disneyplus", "disney"],
                            "stremio" => &["stremio"],
                            "livetv" => &["live_tv", "livetv"],
                            "mediaplayer" => &["media_player", "mediaplayer", "gallery"],
                            "aptoide" => &["aptoide_tv", "aptoide"],
                            "projectivy" => &["projectivy", "projectivy_launcher", "launcher"],
                            _ => &[],
                        };
                        for a in id_aliases {
                            if !candidates.contains(&a.to_string()) {
                                candidates.push(a.to_string());
                            }
                        }

                        // C. Item Name variations (e.g. "Prime Video" -> "prime_video")
                        let clean_name = app_name.to_lowercase().replace(' ', "_").replace('+', "_plus").replace('-', "_");
                        if !clean_name.is_empty() && !candidates.contains(&clean_name) {
                            candidates.push(clean_name.clone());
                            let compact = clean_name.replace('_', "");
                            if !candidates.contains(&compact) {
                                candidates.push(compact);
                            }
                        }

                        // D. Clean package & short name
                        let clean_pkg = pkg.replace('.', "_").to_lowercase();
                        if !candidates.contains(&clean_pkg) {
                            candidates.push(clean_pkg);
                        }

                        let short_name = pkg.split('.').last().unwrap_or(pkg).to_lowercase();
                        if !candidates.contains(&short_name) {
                            candidates.push(short_name);
                        }

                        // Try resolving each candidate
                        if let Ok(def_type) = env.new_string("drawable") {
                            for cand in &candidates {
                                if let Ok(name_str) = env.new_string(cand) {
                                    if let Ok(id_val) = env.call_method(&res, "getIdentifier", "(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)I", &[(&name_str).into(), (&def_type).into(), (&pack_str).into()]) {
                                        let id = id_val.i().unwrap_or(0);
                                        if id != 0 {
                                            if let Ok(d_val) = env.call_method(&res, "getDrawable", "(I)Landroid/graphics/drawable/Drawable;", &[id.into()]) {
                                                if let Ok(d) = d_val.l() {
                                                    if !d.as_raw().is_null() {
                                                        drawable_opt = Some(d);
                                                        break;
                                                    }
                                                }
                                            } else {
                                                let _ = env.exception_clear();
                                            }
                                        }
                                    } else {
                                        let _ = env.exception_clear();
                                    }
                                }
                            }
                        }
                    }
                } else {
                    let _ = env.exception_clear();
                }
            }
        }

        // 2. Fallback to system application icon
        if drawable_opt.is_none() {
            if let Ok(pkg_str) = env.new_string(pkg) {
                if let Ok(d_val) = env.call_method(&pm, "getApplicationIcon", "(Ljava/lang/String;)Landroid/graphics/drawable/Drawable;", &[(&pkg_str).into()]) {
                    if let Ok(d) = d_val.l() {
                        if !d.as_raw().is_null() {
                            drawable_opt = Some(d);
                        }
                    }
                } else {
                    let _ = env.exception_clear();
                }
            }
        }

        let drawable = drawable_opt?;

        // 3. Render Drawable into Bitmap (128x128 ARGB_8888 for high definition clarity)
        let size = 128i32;
        let config_class = env.find_class("android/graphics/Bitmap$Config").ok()?;
        let argb_field = env.get_static_field(&config_class, "ARGB_8888", "Landroid/graphics/Bitmap$Config;").ok()?;
        let argb_obj = argb_field.l().ok()?;

        let bmp_class = env.find_class("android/graphics/Bitmap").ok()?;
        let bmp_val = env.call_static_method(&bmp_class, "createBitmap", "(IILandroid/graphics/Bitmap$Config;)Landroid/graphics/Bitmap;", &[size.into(), size.into(), (&argb_obj).into()]).ok()?;
        let bmp = bmp_val.l().ok()?;

        let canvas_class = env.find_class("android/graphics/Canvas").ok()?;
        let canvas = env.new_object(canvas_class, "(Landroid/graphics/Bitmap;)V", &[(&bmp).into()]).ok()?;

        let _ = env.call_method(&drawable, "setBounds", "(IIII)V", &[0i32.into(), 0i32.into(), size.into(), size.into()]);
        let _ = env.call_method(&drawable, "draw", "(Landroid/graphics/Canvas;)V", &[(&canvas).into()]);

        let byte_buffer_class = env.find_class("java/nio/ByteBuffer").ok()?;
        let buf_val = env.call_static_method(byte_buffer_class, "allocate", "(I)Ljava/nio/ByteBuffer;", &[(size * size * 4).into()]).ok()?;
        let buf = buf_val.l().ok()?;

        let _ = env.call_method(&bmp, "copyPixelsToBuffer", "(Ljava/nio/Buffer;)V", &[(&buf).into()]);
        let array_val = env.call_method(&buf, "array", "()[B", &[]).ok()?;
        let jbytes = jni::objects::JByteArray::from(array_val.l().ok()?);

        let len = env.get_array_length(&jbytes).ok()? as usize;
        let mut raw_i8 = vec![0i8; len];
        env.get_byte_array_region(&jbytes, 0, &mut raw_i8).ok()?;
        let raw_pixels: Vec<u8> = raw_i8.into_iter().map(|b| b as u8).collect();

        Some(blur::pixels_to_image(&raw_pixels, size as u32, size as u32))
    }
}

#[cfg(not(target_os = "android"))]
fn load_app_icon_image(
    _app: &slint::android::AndroidApp,
    _item_id: &str,
    _pkg: &str,
    _app_name: &str,
    _icon_pack_pkg: Option<&str>,
    _appfilter: Option<&std::collections::HashMap<String, String>>,
) -> Option<slint::Image> {
    None
}

#[cfg(target_os = "android")]
fn launch_store_search(app: &slint::android::AndroidApp, query: &str) {
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

        let uri_class = match env.find_class("android/net/Uri") {
            Ok(c) => c,
            Err(_) => return,
        };
        let uri_str = match env.new_string(format!("market://search?q={}", query)) {
            Ok(s) => s,
            Err(_) => return,
        };
        let uri_obj = match env.call_static_method(uri_class, "parse", "(Ljava/lang/String;)Landroid/net/Uri;", &[(&uri_str).into()]) {
            Ok(val) => match val.l() {
                Ok(o) => o,
                Err(_) => return,
            },
            Err(_) => return,
        };

        let intent_class = match env.find_class("android/content/Intent") {
            Ok(c) => c,
            Err(_) => return,
        };
        let action_view = match env.new_string("android.intent.action.VIEW") {
            Ok(s) => s,
            Err(_) => return,
        };
        if let Ok(intent) = env.new_object(intent_class, "(Ljava/lang/String;Landroid/net/Uri;)V", &[(&action_view).into(), (&uri_obj).into()]) {
            let _ = env.call_method(&intent, "addFlags", "(I)Landroid/content/Intent;", &[0x10000000i32.into()]);
            let _ = env.call_method(&activity, "startActivity", "(Landroid/content/Intent;)V", &[(&intent).into()]);
        }
    }
}

#[cfg(not(target_os = "android"))]
fn launch_store_search(_app: &slint::android::AndroidApp, query: &str) {
    println!("Mock launch store search for: {}", query);
}

#[cfg(target_os = "android")]
fn query_installed_apps(app: &slint::android::AndroidApp) -> Vec<(String, String)> {
    let mut result = Vec::new();
    unsafe {
        let vm_ptr = app.vm_as_ptr() as *mut jni::sys::JavaVM;
        if vm_ptr.is_null() { return result; }
        let vm = match jni::JavaVM::from_raw(vm_ptr) {
            Ok(v) => v,
            Err(_) => return result,
        };
        let mut env = match vm.attach_current_thread() {
            Ok(e) => e,
            Err(_) => return result,
        };
        let activity = jni::objects::JObject::from_raw(app.activity_as_ptr() as jni::sys::jobject);

        let pm = match env.call_method(&activity, "getPackageManager", "()Landroid/content/pm/PackageManager;", &[]) {
            Ok(val) => match val.l() {
                Ok(obj) => obj,
                Err(_) => return result,
            },
            Err(_) => return result,
        };

        let action_main = match env.new_string("android.intent.action.MAIN") {
            Ok(s) => s,
            Err(_) => return result,
        };
        let cat_leanback = match env.new_string("android.intent.category.LEANBACK_LAUNCHER") {
            Ok(s) => s,
            Err(_) => return result,
        };
        let cat_launcher = match env.new_string("android.intent.category.LAUNCHER") {
            Ok(s) => s,
            Err(_) => return result,
        };

        let intent_class = match env.find_class("android/content/Intent") {
            Ok(c) => c,
            Err(_) => return result,
        };

        let mut seen_packages = std::collections::HashSet::new();

        for cat in [&cat_leanback, &cat_launcher] {
            if let Ok(intent_obj) = env.new_object(&intent_class, "(Ljava/lang/String;)V", &[(&action_main).into()]) {
                let _ = env.call_method(&intent_obj, "addCategory", "(Ljava/lang/String;)Landroid/content/Intent;", &[cat.into()]);
                if let Ok(list_val) = env.call_method(&pm, "queryIntentActivities", "(Landroid/content/Intent;I)Ljava/util/List;", &[(&intent_obj).into(), 0i32.into()]) {
                    if let Ok(list_obj) = list_val.l() {
                        if !list_obj.as_raw().is_null() {
                            if let Ok(size_val) = env.call_method(&list_obj, "size", "()I", &[]) {
                                let size = size_val.i().unwrap_or(0);
                                for i in 0..size {
                                    if let Ok(item_val) = env.call_method(&list_obj, "get", "(I)Ljava/lang/Object;", &[i.into()]) {
                                        if let Ok(resolve_info) = item_val.l() {
                                            if let Ok(act_info_val) = env.get_field(&resolve_info, "activityInfo", "Landroid/content/pm/ActivityInfo;") {
                                                if let Ok(act_info) = act_info_val.l() {
                                                    if let Ok(pkg_val) = env.get_field(&act_info, "packageName", "Ljava/lang/String;") {
                                                        if let Ok(pkg_obj) = pkg_val.l() {
                                                            let pkg_jstring = jni::objects::JString::from(pkg_obj);
                                                            let pkg_opt = match env.get_string(&pkg_jstring) {
                                                                Ok(s) => Some(s.to_string_lossy().to_string()),
                                                                Err(_) => None,
                                                            };
                                                            if let Some(pkg) = pkg_opt {
                                                                if pkg != "com.cleanframe.tv" && !seen_packages.contains(&pkg) {
                                                                    seen_packages.insert(pkg.clone());
                                                                    let mut app_label = pkg.clone();
                                                                    if let Ok(label_val) = env.call_method(&resolve_info, "loadLabel", "(Landroid/content/pm/PackageManager;)Ljava/lang/CharSequence;", &[(&pm).into()]) {
                                                                        if let Ok(label_obj) = label_val.l() {
                                                                            if !label_obj.as_raw().is_null() {
                                                                                if let Ok(str_val) = env.call_method(&label_obj, "toString", "()Ljava/lang/String;", &[]) {
                                                                                    if let Ok(str_obj) = str_val.l() {
                                                                                        let str_j = jni::objects::JString::from(str_obj);
                                                                                        let label_opt = match env.get_string(&str_j) {
                                                                                            Ok(s) => Some(s.to_string_lossy().to_string()),
                                                                                            Err(_) => None,
                                                                                        };
                                                                                        if let Some(lbl) = label_opt {
                                                                                            app_label = lbl;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    result.push((pkg, app_label));
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
                        }
                    }
                }
            }
        }
    }
    result
}

#[cfg(not(target_os = "android"))]
fn query_installed_apps(_app: &slint::android::AndroidApp) -> Vec<(String, String)> {
    Vec::new()
}

fn populate_installed_apps_into_ui(win: &MainWindow, app: &slint::android::AndroidApp) {
    let installed = query_installed_apps(app);
    if installed.is_empty() { return; }
    let all_model = win.get_all_apps_list();
    let total = all_model.row_count();
    let mut existing_pkgs = std::collections::HashSet::new();

    let full_model = Rc::new(slint::VecModel::default());
    for i in 0..total {
        if let Some(item) = all_model.row_data(i) {
            existing_pkgs.insert(item.pkg.to_string());
            full_model.push(item);
        }
    }

    let fallback_icon = if let Some(first) = all_model.row_data(0) {
        first.icon
    } else {
        slint::Image::default()
    };

    for (pkg, label) in installed {
        if !existing_pkgs.contains(&pkg) {
            existing_pkgs.insert(pkg.clone());
            let mut h: u32 = 0;
            for b in pkg.bytes() { h = h.wrapping_mul(31).wrapping_add(b as u32); }
            let r = ((h & 0xFF) % 180 + 75) as u8;
            let g = (((h >> 8) & 0xFF) % 180 + 75) as u8;
            let b = (((h >> 16) & 0xFF) % 180 + 75) as u8;
            let brand_color = slint::Color::from_rgb_u8(r, g, b);

            let app_icon = load_app_icon_image(app, &pkg, &pkg, &label, None, None)
                .unwrap_or_else(|| fallback_icon.clone());

            full_model.push(AppItem {
                id: pkg.clone().into(),
                name: label.into(),
                pkg: pkg.into(),
                icon: app_icon,
                brand_color,
                in_dock: false,
            });
        }
    }

    win.set_all_apps_list(full_model.into());
}

fn apply_icon_pack_to_models(win: &MainWindow, s: &mut AppState, app: &slint::android::AndroidApp) {
    s.app_icon_cache.borrow_mut().clear();
    let all_model = win.get_all_apps_list();
    let total = all_model.row_count();
    let updated_all = Rc::new(slint::VecModel::default());

    let is_custom = s.icon_pack != "CleanFrame default" && s.icon_pack != "Full Color";
    win.set_current_icon_pack(s.icon_pack.clone().into());
    win.set_is_custom_icon_pack(is_custom);

    let icon_pack_pkg: Option<String> = if s.icon_pack == "CleanFrame default" || s.icon_pack == "Full Color" || s.icon_pack == "System Default" {
        None
    } else {
        s.installed_icon_packs.iter().find(|(p, name)| *name == s.icon_pack || *p == s.icon_pack).map(|(p, _)| p.clone())
    };

    if let Some(ref pack_pkg) = icon_pack_pkg {
        if !s.icon_pack_mappings.borrow().contains_key(pack_pkg) {
            let mappings = load_icon_pack_appfilter(app, pack_pkg);
            s.icon_pack_mappings.borrow_mut().insert(pack_pkg.clone(), mappings);
        }
    }

    let appfilter_opt = if let Some(ref pack_pkg) = icon_pack_pkg {
        s.icon_pack_mappings.borrow().get(pack_pkg).cloned()
    } else {
        None
    };

    for i in 0..total {
        if let Some(mut item) = all_model.row_data(i) {
            if s.icon_pack == "CleanFrame default" || s.icon_pack == "Full Color" {
                if let Some(svg) = s.stock_vector_icons.get(&item.pkg.to_string()) {
                    item.icon = svg.clone();
                }
            } else {
                // System Default or External Icon Pack
                let cache_key = format!("{}:{}:{}", s.icon_pack, item.id, item.pkg);
                let mut cache = s.app_icon_cache.borrow_mut();
                let img = if let Some(cached) = cache.get(&cache_key) {
                    cached.clone()
                } else {
                    let loaded = load_app_icon_image(
                        app,
                        item.id.as_str(),
                        item.pkg.as_str(),
                        item.name.as_str(),
                        icon_pack_pkg.as_deref(),
                        appfilter_opt.as_ref(),
                    );
                    let chosen = loaded.unwrap_or_else(|| {
                        s.stock_vector_icons.get(&item.pkg.to_string()).cloned().unwrap_or(item.icon.clone())
                    });
                    cache.insert(cache_key, chosen.clone());
                    chosen
                };
                item.icon = img;
            }
            updated_all.push(item);
        }
    }
    win.set_all_apps_list(updated_all.into());
    update_dock_and_all_apps(win, s);
}

fn update_dock_and_all_apps(win: &MainWindow, s: &mut AppState) {
    let all_model = win.get_all_apps_list();
    let total = all_model.row_count();
    let dock_model = Rc::new(slint::VecModel::default());

    for i in 0..total {
        if let Some(mut item) = all_model.row_data(i) {
            item.in_dock = s.dock_indices.contains(&i);
            all_model.set_row_data(i, item);
        }
    }

    for &idx in &s.dock_indices {
        if let Some(item) = all_model.row_data(idx) {
            dock_model.push(item);
        }
    }
    win.set_apps(dock_model.into());

    if s.selected_app >= s.dock_indices.len() && !s.dock_indices.is_empty() {
        s.selected_app = s.dock_indices.len() - 1;
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
    log::info!("CleanFrame Native Rust starting on Android TV (4K Ultra HD mode enabled)...");

    let display_info = display4k::configure_4k_display(&app);

    slint::android::init(app.clone()).expect("Failed to initialize Slint on Android");

    let main_window = MainWindow::new().expect("Failed to create MainWindow");
    display4k::enforce_4k_surface(&app, main_window.window(), &display_info);
    let installed_icon_packs = query_installed_icon_packs(&app);

    let mut stock_vector_icons = std::collections::HashMap::new();
    let init_all = main_window.get_all_apps_list();
    for i in 0..init_all.row_count() {
        if let Some(item) = init_all.row_data(i) {
            stock_vector_icons.insert(item.pkg.to_string(), item.icon);
        }
    }

    let state = Rc::new(RefCell::new(AppState {
        active_zone: "apps",
        selected_app: 3, // Default to Prime Video
        selected_input: 0,
        selected_util: 0,
        selected_setting: 0,
        selected_detail_option: 0,
        settings_open: false,
        settings_detail: false,
        current_group: "dock_nav",
        wallpaper_idx: 6, // Bridge at dusk
        refraction: 1.00, // DEFAULT: 1.00 (liquid-rust demo exact default)
        appearance_preset: "Clear", // DEFAULT: Clear (Apple kit Clear glass)
        transparency: 0.00, // DEFAULT: 0.00 Clear
        dark_mode: true,
        increase_contrast: false,
        reduce_motion: false,
        reduce_transparency: false,
        physics: true,
        liquid_merge: true,
        in_appearance_panel: false,
        appearance_focus_index: 3, // Focus on Refraction Slider!
        blur_level: 1,
        layout_mode: "Dock",
        typography: "Manrope",
        material: "Clear",
        focus_style: "Glass Plate",
        icon_pack: "CleanFrame default".to_string(),
        installed_icon_packs,
        stock_vector_icons,
        icon_pack_mappings: std::cell::RefCell::new(std::collections::HashMap::new()),
        app_icon_cache: std::cell::RefCell::new(std::collections::HashMap::new()),
        art_mode: false,
        art_saved_notice: false,
        now_playing_visible: false,
        now_playing_title: "CleanFrame Ambient".to_string(),
        now_playing_artist: "The Frame Audio".to_string(),
        now_playing_playing: false,
        all_apps_open: false,
        selected_all_app: 0,
        all_apps_action_open: false,
        selected_all_app_action: 0,
        dock_indices: vec![0, 1, 2, 3, 4, 5, 6, 7],
        detail_back_text: "‹ Settings",
        base_blur_cache: std::cell::RefCell::new(std::collections::HashMap::new()),
        dock_frost_cache: std::cell::RefCell::new(None),
        focus_lens_cache: std::cell::RefCell::new(None),
    }));

    populate_installed_apps_into_ui(&main_window, &app);
    update_dock_and_all_apps(&main_window, &mut state.borrow_mut());

    main_window.set_clock_text("3:37".into());
    main_window.set_focused_caption("Prime Video".into());
    main_window.set_selected_app(3);
    main_window.set_wallpaper_idx(6);
    let (clean_frost, dock_frost) = state.borrow().get_frost_images(6);
    main_window.set_dynamic_frost_image(clean_frost);
    main_window.set_dock_frost_image(dock_frost);
    let (dock_lens, list_lens, round_lens) = state.borrow().get_focus_lens_images();
    main_window.set_focus_lens_image(dock_lens);
    main_window.set_focus_list_lens_image(list_lens);
    main_window.set_round_focus_lens_image(round_lens);
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

    // 4K Surface Verification Timer (Every 500ms)
    let surface_timer = slint::Timer::default();
    {
        let window_weak = main_window.as_weak();
        let app_handle_timer = app_handle.clone();
        let disp_info = display_info;
        surface_timer.start(
            slint::TimerMode::Repeated,
            std::time::Duration::from_millis(500),
            move || {
                if let Some(win) = window_weak.upgrade() {
                    display4k::enforce_4k_surface(&app_handle_timer, win.window(), &disp_info);
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
            } else if s.all_apps_open {
                if s.all_apps_action_open {
                    if s.selected_all_app_action > 0 {
                        s.selected_all_app_action -= 1;
                    }
                } else if s.selected_all_app >= 4 {
                    s.selected_all_app -= 4;
                }
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
            let total_all_apps = window_weak.upgrade().map(|w| w.get_all_apps_list().row_count()).unwrap_or(16);
            if s.art_mode {
                s.art_mode = false;
            } else if s.all_apps_open {
                if s.all_apps_action_open {
                    if s.selected_all_app_action < 1 {
                        s.selected_all_app_action += 1;
                    }
                } else if s.selected_all_app + 4 < total_all_apps {
                    s.selected_all_app += 4;
                }
            } else if s.settings_open {
                if s.settings_detail {
                    let max_opt = get_detail_options_len(&s) as i32 - 1;
                    if s.selected_detail_option < max_opt {
                        s.selected_detail_option += 1;
                    }
                } else if s.selected_setting < 3 {
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
            } else if s.all_apps_open {
                if !s.all_apps_action_open && s.selected_all_app > 0 && s.selected_all_app % 4 > 0 {
                    s.selected_all_app -= 1;
                }
            } else if s.settings_open {
                if s.settings_detail {
                    if s.selected_detail_option == -1 {
                        handle_settings_back(&mut s);
                    } else if s.current_group == "focus_item" {
                        match s.selected_detail_option {
                            0 => {
                                s.focus_style = match s.focus_style {
                                    "Minimal Type" => "Round",
                                    "Round" => "Glass Plate",
                                    _ => "Minimal Type",
                                };
                                s.focus_lens_cache.borrow_mut().take();
                            }
                            1 => {
                                s.refraction = (s.refraction - 0.05).max(0.0);
                                s.dock_frost_cache.borrow_mut().take();
                                s.focus_lens_cache.borrow_mut().take();
                            }
                            2 => {
                                s.appearance_preset = match s.appearance_preset {
                                    "Tinted" => "Regular",
                                    "Regular" => "Clear",
                                    _ => "Tinted",
                                };
                                s.material = s.appearance_preset;
                                s.transparency = if s.appearance_preset == "Clear" { 0.0 } else if s.appearance_preset == "Regular" { 0.5 } else { 0.85 };
                                s.dock_frost_cache.borrow_mut().take();
                                s.focus_lens_cache.borrow_mut().take();
                            }
                            3 => {
                                s.transparency = (s.transparency - 0.05).max(0.0);
                                s.focus_lens_cache.borrow_mut().take();
                            }
                            _ => handle_settings_back(&mut s),
                        }
                    } else {
                        handle_settings_back(&mut s);
                    }
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
            let total_all_apps = window_weak.upgrade().map(|w| w.get_all_apps_list().row_count()).unwrap_or(16);
            if s.art_mode {
                s.wallpaper_idx = (s.wallpaper_idx + 1) % WALLPAPERS.len();
                s.art_saved_notice = false;
            } else if s.all_apps_open {
                if !s.all_apps_action_open && s.selected_all_app + 1 < total_all_apps && s.selected_all_app % 4 < 3 {
                    s.selected_all_app += 1;
                }
            } else if s.settings_open {
                if s.settings_detail {
                    if s.current_group == "dock_nav" {
                        if s.selected_detail_option == 0 {
                            s.current_group = "focus_item";
                            s.selected_detail_option = 0;
                            s.detail_back_text = "‹ Dock & Navigation";
                        } else {
                            s.current_group = "dock_settings";
                            s.selected_detail_option = 0;
                            s.detail_back_text = "‹ Dock & Navigation";
                        }
                    } else if s.current_group == "focus_item" {
                        match s.selected_detail_option {
                            0 => {
                                s.focus_style = match s.focus_style {
                                    "Glass Plate" => "Round",
                                    "Round" => "Minimal Type",
                                    _ => "Glass Plate",
                                };
                                s.focus_lens_cache.borrow_mut().take();
                            }
                            1 => {
                                s.refraction = (s.refraction + 0.05).min(1.0);
                                s.dock_frost_cache.borrow_mut().take();
                                s.focus_lens_cache.borrow_mut().take();
                            }
                            2 => {
                                s.appearance_preset = match s.appearance_preset {
                                    "Clear" => "Regular",
                                    "Regular" => "Tinted",
                                    _ => "Clear",
                                };
                                s.material = s.appearance_preset;
                                s.transparency = if s.appearance_preset == "Clear" { 0.0 } else if s.appearance_preset == "Regular" { 0.5 } else { 0.85 };
                                s.dock_frost_cache.borrow_mut().take();
                                s.focus_lens_cache.borrow_mut().take();
                            }
                            3 => {
                                s.transparency = (s.transparency + 0.05).min(1.0);
                                s.focus_lens_cache.borrow_mut().take();
                            }
                            _ => {}
                        }
                    } else if s.current_group == "dock_settings" {
                        match s.selected_detail_option {
                            0 => {
                                s.layout_mode = if s.layout_mode == "Dock" { "List" } else { "Dock" };
                            }
                            1 => {
                                s.material = match s.material {
                                    "Frost" => "Soft",
                                    "Soft" => "Obsidian Tint",
                                    "Obsidian Tint" => "Clear",
                                    _ => "Frost",
                                };
                                s.focus_lens_cache.borrow_mut().take();
                            }
                            2 => {
                                s.blur_level = (s.blur_level + 1) % BLUR_LEVEL_NAMES.len();
                                s.base_blur_cache.borrow_mut().clear();
                                s.dock_frost_cache.borrow_mut().take();
                            }
                            3 => {
                                s.current_group = "icons";
                                s.selected_detail_option = 0;
                                s.detail_back_text = "‹ Dock Settings";
                            }
                            _ => {}
                        }
                    } else if s.current_group == "wallpaper_type" {
                        if s.selected_detail_option == 0 {
                            s.current_group = "background";
                            s.selected_detail_option = s.wallpaper_idx as i32;
                            s.detail_back_text = "‹ Wallpaper, Icons & Typography";
                        } else if s.selected_detail_option == 1 {
                            s.current_group = "icons";
                            s.selected_detail_option = 0;
                            s.detail_back_text = "‹ Wallpaper, Icons & Typography";
                        } else {
                            s.typography = match s.typography {
                                "Manrope" => "Newsreader",
                                "Newsreader" => "Geist",
                                _ => "Manrope",
                            };
                        }
                    } else if s.current_group == "system_inputs" {
                        if s.selected_detail_option == 0 {
                            s.current_group = "inputs";
                            s.selected_detail_option = 0;
                            s.detail_back_text = "‹ System & Inputs";
                        } else if s.selected_detail_option == 1 {
                            s.all_apps_open = true;
                            s.selected_all_app = 0;
                            s.settings_open = false;
                        } else {
                            s.current_group = "samsung";
                            s.selected_detail_option = 0;
                            s.detail_back_text = "‹ System & Inputs";
                        }
                    }
                } else {
                    open_settings_detail(&mut s);
                }
            } else if s.layout_mode == "Dock" {
                if s.active_zone == "apps" {
                    if s.selected_app + 1 < s.dock_indices.len() {
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
            } else if s.all_apps_open {
                if s.all_apps_action_open {
                    if s.selected_all_app_action == 0 {
                        // Launch app
                        if let Some(win) = window_weak.upgrade() {
                            let catalog = win.get_all_apps_list();
                            if let Some(item) = catalog.row_data(s.selected_all_app) {
                                s.all_apps_open = false;
                                s.all_apps_action_open = false;
                                if item.pkg == "android.settings.SETTINGS" || item.pkg == "settings" {
                                    s.settings_open = true;
                                    s.settings_detail = false;
                                    s.in_appearance_panel = false;
                                    s.selected_setting = 0;
                                } else {
                                    launch_package(&app_ref, item.pkg.as_str());
                                }
                            }
                        }
                    } else {
                        // Toggle Dock (Pin / Unpin)
                        let app_idx = s.selected_all_app;
                        if let Some(pos) = s.dock_indices.iter().position(|&x| x == app_idx) {
                            if s.dock_indices.len() > 1 {
                                s.dock_indices.remove(pos);
                                if s.selected_app >= s.dock_indices.len() {
                                    s.selected_app = s.dock_indices.len().saturating_sub(1);
                                }
                            }
                        } else {
                            s.dock_indices.push(app_idx);
                        }
                        if let Some(win) = window_weak.upgrade() {
                            update_dock_and_all_apps(&win, &mut s);
                        }
                        s.all_apps_action_open = false;
                    }
                } else {
                    // Open Action Modal for currently focused app
                    s.all_apps_action_open = true;
                    s.selected_all_app_action = 0;
                }
            } else if s.settings_open {
                if s.settings_detail {
                    if s.selected_detail_option == -1 {
                        handle_settings_back(&mut s);
                    } else {
                        if let Some(w) = window_weak.upgrade() { handle_detail_ok(&w, &mut s, &app_ref); }
                    }
                } else {
                    open_settings_detail(&mut s);
                }
            } else if s.active_zone == "apps" {
                if let Some(win) = window_weak.upgrade() {
                    let apps = win.get_apps();
                    if let Some(item) = apps.row_data(s.selected_app) {
                        if item.pkg == "android.settings.SETTINGS" || item.pkg == "settings" || item.id == "settings" {
                            s.settings_open = true;
                            s.settings_detail = false;
                            s.in_appearance_panel = false;
                            s.selected_setting = 0;
                        } else {
                            launch_package(&app_ref, item.pkg.as_str());
                        }
                    }
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
                        // All apps: Open All Applications Studio!
                        s.all_apps_open = true;
                        s.selected_all_app = 0;
                        s.all_apps_action_open = false;
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
                        s.in_appearance_panel = false;
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
            } else if s.all_apps_open {
                if s.all_apps_action_open {
                    s.all_apps_action_open = false;
                } else {
                    s.all_apps_open = false;
                }
            } else if s.settings_open {
                if s.settings_detail {
                    handle_settings_back(&mut s);
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

    // Set Refraction directly (from touch / pointer or slider)
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_set_refraction(move |val| {
            let mut s = state_clone.borrow_mut();
            s.refraction = val.clamp(0.0, 1.0);
            s.dock_frost_cache.borrow_mut().take();
            s.focus_lens_cache.borrow_mut().take();
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    // Set Transparency directly
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_set_transparency(move |val| {
            let mut s = state_clone.borrow_mut();
            s.transparency = val.clamp(0.0, 1.0);
            s.focus_lens_cache.borrow_mut().take();
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    // Set Appearance Preset directly
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_set_appearance_preset(move |preset| {
            let mut s = state_clone.borrow_mut();
            let p_str = preset.as_str();
            s.appearance_preset = match p_str {
                "Regular" => "Regular",
                "Tinted" => "Tinted",
                _ => "Clear",
            };
            s.material = s.appearance_preset;
            if s.appearance_preset == "Clear" {
                s.transparency = 0.0;
            } else if s.appearance_preset == "Regular" {
                s.transparency = 0.5;
            } else {
                s.transparency = 0.85;
            }
            s.dock_frost_cache.borrow_mut().take();
            s.focus_lens_cache.borrow_mut().take();
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    // Toggle Chip directly
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_toggle_chip(move |chip| {
            let mut s = state_clone.borrow_mut();
            match chip.as_str() {
                "dark" => s.dark_mode = !s.dark_mode,
                "contrast" => s.increase_contrast = !s.increase_contrast,
                "reduce_motion" => s.reduce_motion = !s.reduce_motion,
                "reduce_transparency" => s.reduce_transparency = !s.reduce_transparency,
                "physics" => s.physics = !s.physics,
                "liquid_merge" => s.liquid_merge = !s.liquid_merge,
                _ => {}
            }
            s.dock_frost_cache.borrow_mut().take();
            s.focus_lens_cache.borrow_mut().take();
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    // DASHBOARD & INTERACTION CLICK HANDLERS
    // 1. App Clicked (Dock or List)
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        let app_ref = app_handle.clone();
        main_window.on_app_clicked(move |idx| {
            let mut s = state_clone.borrow_mut();
            let u_idx = idx.max(0) as usize;
            s.active_zone = "apps";
            s.selected_app = u_idx;
            if let Some(win) = window_weak.upgrade() {
                let apps = win.get_apps();
                if let Some(item) = apps.row_data(u_idx) {
                    if item.pkg == "android.settings.SETTINGS" || item.pkg == "settings" || item.id == "settings" {
                        s.settings_open = true;
                        s.settings_detail = false;
                        s.in_appearance_panel = false;
                        s.selected_setting = 0;
                    } else {
                        launch_package(&app_ref, item.pkg.as_str());
                    }
                }
                sync_ui(&win, &s);
            }
        });
    }

    // 2. Utilities Clicked
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_util_clicked(move |u_idx| {
            let mut s = state_clone.borrow_mut();
            s.active_zone = "util";
            s.selected_util = u_idx.max(0) as usize;
            match u_idx {
                0 => {
                    s.art_mode = true;
                    s.art_saved_notice = false;
                }
                1 => {
                    s.all_apps_open = true;
                    s.selected_all_app = 0;
                    s.all_apps_action_open = false;
                }
                2 => {
                    s.active_zone = "inputs";
                    s.selected_input = 0;
                }
                3 => {
                    s.settings_open = true;
                    s.settings_detail = false;
                    s.in_appearance_panel = false;
                    s.selected_setting = 0;
                }
                _ => {}
            }
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    // 3. HDMI Input Clicked
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        let app_ref = app_handle.clone();
        main_window.on_input_clicked(move |h_idx| {
            let mut s = state_clone.borrow_mut();
            s.active_zone = "inputs";
            s.selected_input = h_idx.max(0) as usize;
            if h_idx >= 0 && (h_idx as usize) < HDMI_IDS.len() {
                let input_id = HDMI_IDS[h_idx as usize];
                launch_tv_input(&app_ref, input_id);
            }
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    // 4. Media Player Clicked
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_media_clicked(move || {
            let mut s = state_clone.borrow_mut();
            s.active_zone = "media";
            s.now_playing_playing = !s.now_playing_playing;
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    // 5. All Apps Item Clicked
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_all_app_clicked(move |idx| {
            let mut s = state_clone.borrow_mut();
            s.selected_all_app = idx.max(0) as usize;
            s.all_apps_action_open = true;
            s.selected_all_app_action = 0;
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    // 6. All Apps Action Clicked (0: Launch, 1: Toggle Dock)
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        let app_ref = app_handle.clone();
        main_window.on_all_app_action_clicked(move |action_idx| {
            let mut s = state_clone.borrow_mut();
            s.selected_all_app_action = action_idx.max(0) as usize;
            if action_idx == 0 {
                if let Some(win) = window_weak.upgrade() {
                    let catalog = win.get_all_apps_list();
                    if let Some(item) = catalog.row_data(s.selected_all_app) {
                        s.all_apps_open = false;
                        s.all_apps_action_open = false;
                        if item.pkg == "android.settings.SETTINGS" || item.pkg == "settings" {
                            s.settings_open = true;
                            s.settings_detail = false;
                            s.in_appearance_panel = false;
                            s.selected_setting = 0;
                        } else {
                            launch_package(&app_ref, item.pkg.as_str());
                        }
                    }
                }
            } else {
                let app_idx = s.selected_all_app;
                if let Some(pos) = s.dock_indices.iter().position(|&x| x == app_idx) {
                    if s.dock_indices.len() > 1 {
                        s.dock_indices.remove(pos);
                        if s.selected_app >= s.dock_indices.len() {
                            s.selected_app = s.dock_indices.len().saturating_sub(1);
                        }
                    }
                } else {
                    s.dock_indices.push(app_idx);
                }
                if let Some(win) = window_weak.upgrade() {
                    update_dock_and_all_apps(&win, &mut s);
                }
                s.all_apps_action_open = false;
            }
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    // 7. Settings Category Clicked
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_setting_category_clicked(move |idx| {
            let mut s = state_clone.borrow_mut();
            s.selected_setting = idx.max(0) as usize;
            open_settings_detail(&mut s);
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    // 8. Settings Back Clicked
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_settings_back_clicked(move || {
            let mut s = state_clone.borrow_mut();
            handle_settings_back(&mut s);
            if let Some(win) = window_weak.upgrade() {
                sync_ui(&win, &s);
            }
        });
    }

    // 9. Detail Option Clicked
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        let app_ref = app_handle.clone();
        main_window.on_detail_option_clicked(move |opt_idx| {
            let mut s = state_clone.borrow_mut();
            s.selected_detail_option = opt_idx;
            if let Some(w) = window_weak.upgrade() {
                handle_detail_ok(&w, &mut s, &app_ref);
                sync_ui(&w, &s);
            }
        });
    }

    // 10. Art Mode Clicked (Dismiss)
    {
        let window_weak = main_window.as_weak();
        let state_clone = state.clone();
        main_window.on_art_mode_clicked(move || {
            let mut s = state_clone.borrow_mut();
            s.art_mode = false;
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

fn handle_settings_back(s: &mut AppState) {
    match s.current_group {
        "focus_item" => {
            s.current_group = "dock_nav";
            s.selected_detail_option = 0;
            s.detail_back_text = "‹ Settings";
        }
        "dock_settings" => {
            s.current_group = "dock_nav";
            s.selected_detail_option = 1;
            s.detail_back_text = "‹ Settings";
        }
        "icons" => {
            if s.detail_back_text.contains("Wallpaper") {
                s.current_group = "wallpaper_type";
                s.selected_detail_option = 1;
                s.detail_back_text = "‹ Settings";
            } else {
                s.current_group = "dock_settings";
                s.selected_detail_option = 3;
                s.detail_back_text = "‹ Dock & Navigation";
            }
        }
        "background" | "typography" => {
            s.current_group = "wallpaper_type";
            s.selected_detail_option = 0;
            s.detail_back_text = "‹ Settings";
        }
        "inputs" | "samsung" | "editor" => {
            s.current_group = "system_inputs";
            s.selected_detail_option = 0;
            s.detail_back_text = "‹ Settings";
        }
        _ => {
            s.settings_detail = false;
        }
    }
}

fn open_settings_detail(s: &mut AppState) {
    s.settings_detail = true;
    s.selected_detail_option = 0;
    match s.selected_setting {
        0 => {
            s.current_group = "dock_nav";
            s.detail_back_text = "‹ Settings";
        }
        1 => {
            s.current_group = "wallpaper_type";
            s.detail_back_text = "‹ Settings";
        }
        2 => {
            s.current_group = "visual_effects";
            s.detail_back_text = "‹ Settings";
        }
        3 => {
            s.current_group = "system_inputs";
            s.detail_back_text = "‹ Settings";
        }
        _ => {
            s.current_group = "dock_nav";
            s.detail_back_text = "‹ Settings";
        }
    }
}

fn get_detail_options_len(s: &AppState) -> usize {
    match s.current_group {
        "dock_nav" => 2,
        "focus_item" => 4,
        "dock_settings" => 4,
        "wallpaper_type" => 3,
        "visual_effects" => 6,
        "system_inputs" => 3,
        "background" => WALLPAPERS.len(),
        "typography" => 3,
        "inputs" => HDMI_IDS.len(),
        "samsung" => 2,
        "editor" => 1,
        "icons" => 3 + s.installed_icon_packs.len() + 1,
        _ => 2,
    }
}

fn handle_detail_ok(win: &MainWindow, s: &mut AppState, app: &slint::android::AndroidApp) {
    match s.current_group {
        "dock_nav" => {
            if s.selected_detail_option == 0 {
                s.current_group = "focus_item";
                s.selected_detail_option = 0;
                s.detail_back_text = "‹ Dock & Navigation";
            } else {
                s.current_group = "dock_settings";
                s.selected_detail_option = 0;
                s.detail_back_text = "‹ Dock & Navigation";
            }
        }
        "focus_item" => {
            match s.selected_detail_option {
                0 => {
                    s.focus_style = match s.focus_style {
                        "Glass Plate" => "Round",
                        "Round" => "Minimal Type",
                        _ => "Glass Plate",
                    };
                    s.focus_lens_cache.borrow_mut().take();
                }
                1 => {
                    s.refraction = if s.refraction >= 1.0 { 0.0 } else { (s.refraction + 0.25).min(1.0) };
                    s.dock_frost_cache.borrow_mut().take();
                    s.focus_lens_cache.borrow_mut().take();
                }
                2 => {
                    s.appearance_preset = match s.appearance_preset {
                        "Clear" => "Regular",
                        "Regular" => "Tinted",
                        _ => "Clear",
                    };
                    s.material = s.appearance_preset;
                    s.transparency = if s.appearance_preset == "Clear" { 0.0 } else if s.appearance_preset == "Regular" { 0.5 } else { 0.85 };
                    s.dock_frost_cache.borrow_mut().take();
                    s.focus_lens_cache.borrow_mut().take();
                }
                3 => {
                    s.transparency = if s.transparency >= 1.0 { 0.0 } else { (s.transparency + 0.25).min(1.0) };
                    s.focus_lens_cache.borrow_mut().take();
                }
                _ => {}
            }
        }
        "dock_settings" => {
            match s.selected_detail_option {
                0 => {
                    s.layout_mode = if s.layout_mode == "Dock" { "List" } else { "Dock" };
                }
                1 => {
                    s.material = match s.material {
                        "Frost" => "Soft",
                        "Soft" => "Obsidian Tint",
                        "Obsidian Tint" => "Clear",
                        _ => "Frost",
                    };
                    s.focus_lens_cache.borrow_mut().take();
                }
                2 => {
                    s.blur_level = (s.blur_level + 1) % BLUR_LEVEL_NAMES.len();
                    s.base_blur_cache.borrow_mut().clear();
                    s.dock_frost_cache.borrow_mut().take();
                }
                3 => {
                    s.current_group = "icons";
                    s.selected_detail_option = 0;
                    s.detail_back_text = "‹ Dock Settings";
                }
                _ => {}
            }
        }
        "icons" => {
            let total_installed = s.installed_icon_packs.len();
            match s.selected_detail_option {
                0 => {
                    s.icon_pack = "CleanFrame default".to_string();
                    apply_icon_pack_to_models(win, s, app);
                }
                1 => {
                    s.icon_pack = "Full Color".to_string();
                    apply_icon_pack_to_models(win, s, app);
                }
                2 => {
                    s.icon_pack = "System Default".to_string();
                    apply_icon_pack_to_models(win, s, app);
                }
                opt if opt >= 3 && (opt as usize) < 3 + total_installed => {
                    let idx = opt as usize - 3;
                    s.icon_pack = s.installed_icon_packs[idx].1.clone();
                    apply_icon_pack_to_models(win, s, app);
                }
                _ => {
                    launch_store_search(app, "icon pack");
                }
            }
        }
        "wallpaper_type" => {
            if s.selected_detail_option == 0 {
                s.current_group = "background";
                s.selected_detail_option = s.wallpaper_idx as i32;
                s.detail_back_text = "‹ Wallpaper, Icons & Typography";
            } else if s.selected_detail_option == 1 {
                s.current_group = "icons";
                s.selected_detail_option = 0;
                s.detail_back_text = "‹ Wallpaper, Icons & Typography";
            } else {
                s.typography = match s.typography {
                    "Manrope" => "Newsreader",
                    "Newsreader" => "Geist",
                    _ => "Manrope",
                };
            }
        }
        "visual_effects" => {
            match s.selected_detail_option {
                0 => {
                    s.dark_mode = !s.dark_mode;
                    s.focus_lens_cache.borrow_mut().take();
                }
                1 => s.physics = !s.physics,
                2 => s.liquid_merge = !s.liquid_merge,
                3 => s.increase_contrast = !s.increase_contrast,
                4 => s.reduce_motion = !s.reduce_motion,
                5 => {
                    s.reduce_transparency = !s.reduce_transparency;
                    s.dock_frost_cache.borrow_mut().take();
                    s.focus_lens_cache.borrow_mut().take();
                }
                _ => {}
            }
        }
        "system_inputs" => {
            if s.selected_detail_option == 0 {
                s.current_group = "inputs";
                s.selected_detail_option = 0;
                s.detail_back_text = "‹ System & Inputs";
            } else if s.selected_detail_option == 1 {
                s.all_apps_open = true;
                s.selected_all_app = 0;
                s.settings_open = false;
            } else {
                s.current_group = "samsung";
                s.selected_detail_option = 0;
                s.detail_back_text = "‹ System & Inputs";
            }
        }
        "background" => {
            if s.selected_detail_option >= 0 && (s.selected_detail_option as usize) < WALLPAPERS.len() {
                s.wallpaper_idx = s.selected_detail_option as usize;
                s.dock_frost_cache.borrow_mut().take();
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
    let (dock_lens, list_lens, round_lens) = s.get_focus_lens_images();
    win.set_focus_lens_image(dock_lens);
    win.set_focus_list_lens_image(list_lens);
    win.set_round_focus_lens_image(round_lens);
    win.set_current_layout(s.layout_mode.into());
    win.set_current_font(s.typography.into());
    win.set_current_material(s.material.into());
    win.set_current_focus_style(s.focus_style.into());
    win.set_current_icon_pack(s.icon_pack.clone().into());
    win.set_is_custom_icon_pack(s.icon_pack != "CleanFrame default" && s.icon_pack != "Full Color");
    win.set_now_playing_visible(s.now_playing_visible);

    // Liquid Glass Refraction & Appearance Controls
    win.set_refraction_value(s.refraction);
    win.set_refraction_display(format!("{:.2}", s.refraction).into());
    win.set_refraction_subcaption(format!("Material {{ refraction: {:.2}, depth: {:.1}pt, η: 1.50 }}", s.refraction, 12.0 + 40.0 * s.refraction).into());
    win.set_appearance_preset(s.appearance_preset.into());
    win.set_transparency_value(s.transparency);
    win.set_transparency_display(if s.appearance_preset == "Clear" {
        "Clear".into()
    } else if s.appearance_preset == "Tinted" {
        "Tinted".into()
    } else {
        format!("{:.2}", s.transparency).into()
    });
    win.set_opt_dark(s.dark_mode);
    win.set_opt_contrast(s.increase_contrast);
    win.set_opt_reduce_motion(s.reduce_motion);
    win.set_opt_reduce_transparency(s.reduce_transparency);
    win.set_opt_physics(s.physics);
    win.set_opt_liquid_merge(s.liquid_merge);
    win.set_in_appearance_panel(s.in_appearance_panel);
    win.set_appearance_focus_index(s.appearance_focus_index);

    // All Apps Studio State
    win.set_all_apps_open(s.all_apps_open);
    win.set_selected_all_app(s.selected_all_app as i32);
    win.set_all_apps_action_open(s.all_apps_action_open);
    win.set_selected_all_app_action(s.selected_all_app_action as i32);

    // Art Mode sync
    win.set_art_mode(s.art_mode);
    if s.wallpaper_idx < WALLPAPERS.len() {
        let meta = &WALLPAPERS[s.wallpaper_idx];
        win.set_art_title(meta.name.into());
        win.set_art_artist(meta.credit.into());
    }
    win.set_art_saved_notice(s.art_saved_notice);

    // Caption under clock reflecting current focused item
    let app_name_holder: String;
    let caption_text = match s.active_zone {
        "apps" => {
            if let Some(item) = win.get_apps().row_data(s.selected_app) {
                app_name_holder = item.name.to_string();
                app_name_holder.as_str()
            } else if s.selected_app < APP_NAMES.len() {
                APP_NAMES[s.selected_app]
            } else {
                ""
            }
        }
        "inputs" => HDMI_LABELS[s.selected_input.min(HDMI_LABELS.len() - 1)],
        "util" => UTIL_LABELS[s.selected_util.min(UTIL_LABELS.len() - 1)],
        "media" => &s.now_playing_title,
        _ => "",
    };
    win.set_focused_caption(caption_text.into());

    // Update settings overview readout values (4 rows)
    let settings = win.get_settings_list();
    if let Some(mut r) = settings.row_data(0) {
        r.value = format!("Focus: {} · Dock: {}", s.focus_style, s.layout_mode).into();
        settings.set_row_data(0, r);
    }
    if let Some(mut r) = settings.row_data(1) {
        let wp_name = if s.wallpaper_idx < WALLPAPERS.len() { WALLPAPERS[s.wallpaper_idx].name } else { "Photo" };
        r.value = format!("{} · {}", wp_name, s.typography).into();
        settings.set_row_data(1, r);
    }
    if let Some(mut r) = settings.row_data(2) {
        let mut v = Vec::new();
        if s.dark_mode { v.push("Dark"); }
        if s.physics { v.push("Physics"); }
        if s.liquid_merge { v.push("Liquid"); }
        let desc = if v.is_empty() { "Standard".to_string() } else { v.join(" · ") };
        r.value = desc.into();
        settings.set_row_data(2, r);
    }
    if let Some(mut r) = settings.row_data(3) {
        r.value = "HDMI · Apps · System".into();
        settings.set_row_data(3, r);
    }

    win.set_detail_back_text(s.detail_back_text.into());

    // Update Detail Submenu Options
    if s.settings_detail {
        match s.current_group {
            "dock_nav" => {
                win.set_detail_title("Dock & Navigation".into());
                win.set_detail_desc("Configure liquid focus lens optics, Snell refraction depth, and dock layout.".into());
                let list = Rc::new(VecModel::default());
                list.push(DetailOption {
                    id: "nav_focus_item".into(),
                    title: "Focus Item Settings".into(),
                    desc: format!("Style: {} · Refraction: {:.2} · {}", s.focus_style, s.refraction, s.appearance_preset).into(),
                    active: true,
                });
                list.push(DetailOption {
                    id: "nav_dock_settings".into(),
                    title: "Dock Settings".into(),
                    desc: format!("Layout: {} · Material: {} · {}", s.layout_mode, s.material, s.icon_pack).into(),
                    active: false,
                });
                win.set_detail_options(list.into());
            }
            "focus_item" => {
                win.set_detail_title("Focus Item Settings".into());
                win.set_detail_desc("Adjust focus lens shape, Snell optical refraction, and crystal clear material.".into());
                let list = Rc::new(VecModel::default());
                list.push(DetailOption {
                    id: "focus_style".into(),
                    title: format!("Focus Style: {}", s.focus_style).into(),
                    desc: "Round 360° lens orb · Glass Plate · Minimal Type (OK to cycle)".into(),
                    active: true,
                });
                list.push(DetailOption {
                    id: "refraction".into(),
                    title: format!("Snell Refraction: {:.2}", s.refraction).into(),
                    desc: format!("Snell Bevel (η = 1.50, depth {:.1}pt) · Left / Right adjust", 12.0 + 40.0 * s.refraction).into(),
                    active: s.refraction > 0.0,
                });
                list.push(DetailOption {
                    id: "appearance".into(),
                    title: format!("Appearance: {} Glass", s.appearance_preset).into(),
                    desc: "Clear (crystal sheer) · Regular Frost · Tinted (OK to cycle)".into(),
                    active: s.appearance_preset == "Clear",
                });
                list.push(DetailOption {
                    id: "transparency".into(),
                    title: format!("Transparency: {:.2}", s.transparency).into(),
                    desc: "Optical transmission substrate level · Left / Right adjust".into(),
                    active: s.transparency > 0.0,
                });
                win.set_detail_options(list.into());
            }
            "dock_settings" => {
                win.set_detail_title("Dock Settings".into());
                win.set_detail_desc("Configure launcher dock geometry, frosted diffusion, and icon aesthetic.".into());
                let list = Rc::new(VecModel::default());
                list.push(DetailOption {
                    id: "layout".into(),
                    title: format!("Layout Mode: {}", s.layout_mode).into(),
                    desc: "Horizontal liquid glass dock vs vertical column (OK to toggle)".into(),
                    active: s.layout_mode == "Dock",
                });
                list.push(DetailOption {
                    id: "material".into(),
                    title: format!("Dock Material: {}", s.material).into(),
                    desc: "Frost · Soft · Obsidian Tint · Clear (OK to cycle)".into(),
                    active: s.material == "Clear" || s.material == "Frost",
                });
                list.push(DetailOption {
                    id: "blur".into(),
                    title: format!("Optical Blur: {}", BLUR_LEVEL_NAMES[s.blur_level.min(BLUR_LEVEL_NAMES.len() - 1)]).into(),
                    desc: "Subtle frost · Deep blur · Glass blur · Off (OK to cycle)".into(),
                    active: s.blur_level > 0,
                });
                list.push(DetailOption {
                    id: "nav_icons".into(),
                    title: format!("Icon Pack: {}", s.icon_pack).into(),
                    desc: "CleanFrame default · Full Color · System · Installed Theme Packs (OK to open)".into(),
                    active: s.icon_pack == "CleanFrame default",
                });
                win.set_detail_options(list.into());
            }
            "icons" => {
                win.set_detail_title("Icon Pack".into());
                win.set_detail_desc("Select app icon pack aesthetic or choose an installed theme pack.".into());
                let list = Rc::new(VecModel::default());
                list.push(DetailOption {
                    id: "pack_default".into(),
                    title: "CleanFrame default".into(),
                    desc: "Minimalist white brand vector silhouettes on liquid glass".into(),
                    active: s.icon_pack == "CleanFrame default",
                });
                list.push(DetailOption {
                    id: "pack_color".into(),
                    title: "Full Color".into(),
                    desc: "Authentic vibrant brand colors on liquid glass plates".into(),
                    active: s.icon_pack == "Full Color",
                });
                list.push(DetailOption {
                    id: "pack_system".into(),
                    title: "System Default".into(),
                    desc: "Original stock application icons extracted from the system".into(),
                    active: s.icon_pack == "System Default",
                });
                for (pkg, name) in &s.installed_icon_packs {
                    list.push(DetailOption {
                        id: format!("ext_{}", pkg).into(),
                        title: name.clone().into(),
                        desc: format!("External theme pack · {}", pkg).into(),
                        active: s.icon_pack == *name || s.icon_pack == *pkg,
                    });
                }
                list.push(DetailOption {
                    id: "nav_store_icons".into(),
                    title: "+ Browse Icon Packs on Store".into(),
                    desc: "Discover, install and download new icon packs from Aurora / Play Store".into(),
                    active: false,
                });
                win.set_detail_options(list.into());
            }
            "wallpaper_type" => {
                win.set_detail_title("Wallpaper, Icons & Typography".into());
                win.set_detail_desc("Select curated photographic landscapes, icon packs, or fine typography.".into());
                let list = Rc::new(VecModel::default());
                let wp_name = if s.wallpaper_idx < WALLPAPERS.len() { WALLPAPERS[s.wallpaper_idx].name } else { "Photo" };
                list.push(DetailOption {
                    id: "nav_bg".into(),
                    title: format!("Background Wallpaper: {}", wp_name).into(),
                    desc: "15 curated photographic landscapes and fine art galleries (OK to open)".into(),
                    active: true,
                });
                list.push(DetailOption {
                    id: "nav_icons".into(),
                    title: format!("Icon Pack: {}", s.icon_pack).into(),
                    desc: "CleanFrame default · Full Color · System · Installed Theme Packs (OK to open)".into(),
                    active: true,
                });
                list.push(DetailOption {
                    id: "nav_font".into(),
                    title: format!("Typography: {}", s.typography).into(),
                    desc: "Manrope sans-serif · Newsreader serif · Geist grotesque (OK to cycle)".into(),
                    active: false,
                });
                win.set_detail_options(list.into());
            }
            "visual_effects" => {
                win.set_detail_title("Visual Effects & Accessibility".into());
                win.set_detail_desc("Configure liquid glass fluid simulations, dark mode, and accessibility.".into());
                let list = Rc::new(VecModel::default());
                list.push(DetailOption {
                    id: "dark".into(),
                    title: "Dark Mode".into(),
                    desc: "Deep obsidian glass styling for dim viewing rooms".into(),
                    active: s.dark_mode,
                });
                list.push(DetailOption {
                    id: "physics".into(),
                    title: "Snell Physics & Springs".into(),
                    desc: "Fluid spring dynamics and squash/stretch lens momentum".into(),
                    active: s.physics,
                });
                list.push(DetailOption {
                    id: "liquid_merge".into(),
                    title: "Liquid Glass Merge".into(),
                    desc: "Viscous fluid metaball fusion between adjacent lenses".into(),
                    active: s.liquid_merge,
                });
                list.push(DetailOption {
                    id: "contrast".into(),
                    title: "Increase Contrast".into(),
                    desc: "High-contrast text labels and luminous rim highlights".into(),
                    active: s.increase_contrast,
                });
                list.push(DetailOption {
                    id: "motion".into(),
                    title: "Reduce Motion".into(),
                    desc: "Disable fluid bounce and morphing animations".into(),
                    active: s.reduce_motion,
                });
                list.push(DetailOption {
                    id: "transparency_opt".into(),
                    title: "Reduce Transparency".into(),
                    desc: "Solid matte plates with high legibility".into(),
                    active: s.reduce_transparency,
                });
                win.set_detail_options(list.into());
            }
            "system_inputs" => {
                win.set_detail_title("System & Inputs".into());
                win.set_detail_desc("External TV HDMI source passthrough, app manager, and system settings.".into());
                let list = Rc::new(VecModel::default());
                list.push(DetailOption {
                    id: "nav_inputs".into(),
                    title: "TV Inputs & HDMI".into(),
                    desc: "Switch active source between HDMI 1, 2, 3, and AV passthrough".into(),
                    active: true,
                });
                list.push(DetailOption {
                    id: "nav_apps".into(),
                    title: "Installed Applications".into(),
                    desc: "Open All Apps Studio & Aurora Application Store".into(),
                    active: false,
                });
                list.push(DetailOption {
                    id: "nav_tv".into(),
                    title: "Android TV & System Settings".into(),
                    desc: "Display, sound, network, and default launcher settings".into(),
                    active: false,
                });
                win.set_detail_options(list.into());
            }
            "background" => {
                win.set_detail_title("Background Wallpaper".into());
                win.set_detail_desc("Choose a photographic landscape or gallery artwork.".into());
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
            "inputs" => {
                win.set_detail_title("TV Inputs".into());
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
            _ => {
                win.set_detail_title("Settings".into());
                win.set_detail_desc("Configure launcher appearance and behaviors.".into());
            }
        }
    }
}
