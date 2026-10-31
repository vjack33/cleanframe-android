// Native Display & Surface Manager for Android TV

#[cfg(target_os = "android")]
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct DisplayInfo4K {
    pub is_4k_panel: bool,
    pub width: i32,
    pub height: i32,
    pub scale_factor: f32,
}

#[cfg(target_os = "android")]
pub fn configure_4k_display(app: &slint::android::AndroidApp) -> DisplayInfo4K {
    let mut is_4k_panel = false;
    let mut real_w = 1920;
    let mut real_h = 1080;

    unsafe {
        let vm_ptr = app.vm_as_ptr() as *mut jni::sys::JavaVM;
        if !vm_ptr.is_null() {
            if let Ok(vm) = jni::JavaVM::from_raw(vm_ptr) {
                if let Ok(mut env) = vm.attach_current_thread() {
                    let activity = jni::objects::JObject::from_raw(app.activity_as_ptr() as jni::sys::jobject);

                    if let Ok(window_val) = env.call_method(&activity, "getWindow", "()Landroid/view/Window;", &[]) {
                        if let Ok(window) = window_val.l() {
                            if let Ok(wm_val) = env.call_method(&window, "getWindowManager", "()Landroid/view/WindowManager;", &[]) {
                                if let Ok(wm) = wm_val.l() {
                                    if let Ok(disp_val) = env.call_method(&wm, "getDefaultDisplay", "()Landroid/view/Display;", &[]) {
                                        if let Ok(display) = disp_val.l() {
                                            // Query physical display modes
                                            if let Ok(modes_val) = env.call_method(&display, "getSupportedModes", "()[Landroid/view/Display$Mode;", &[]) {
                                                if let Ok(modes_obj) = modes_val.l() {
                                                    let modes_array: &jni::objects::JObjectArray = (&modes_obj).into();
                                                    if let Ok(len) = env.get_array_length(modes_array) {
                                                        for i in 0..len {
                                                            if let Ok(mode_obj) = env.get_object_array_element(modes_array, i) {
                                                                let w = env.call_method(&mode_obj, "getPhysicalWidth", "()I", &[]).map(|v| v.i().unwrap_or(0)).unwrap_or(0);
                                                                let h = env.call_method(&mode_obj, "getPhysicalHeight", "()I", &[]).map(|v| v.i().unwrap_or(0)).unwrap_or(0);
                                                                if w >= 3840 && h >= 2160 {
                                                                    is_4k_panel = true;
                                                                    break;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }

                                            // Query active real display size
                                            if let Ok(point_cls) = env.find_class("android/graphics/Point") {
                                                if let Ok(point) = env.new_object(&point_cls, "()V", &[]) {
                                                    if env.call_method(&display, "getRealSize", "(Landroid/graphics/Point;)V", &[(&point).into()]).is_ok() {
                                                        if let (Ok(x), Ok(y)) = (env.get_field(&point, "x", "I"), env.get_field(&point, "y", "I")) {
                                                            let px = x.i().unwrap_or(1920);
                                                            let py = y.i().unwrap_or(1080);
                                                            if px > 0 && py > 0 {
                                                                real_w = px;
                                                                real_h = py;
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

    // Auto-detect resolution:
    // If the device window compositor supports native 4K (e.g. 4K emulator or unlocked TV), use 3840x2160 @ scale 4.0.
    // If the device compositor is 1080p (e.g. standard Android TV BSP with max_graphics_width = 1920), use 1920x1080 @ scale 2.0.
    let (target_w, target_h, scale_factor) = if real_w >= 3840 && real_h >= 2160 {
        (3840, 2160, 4.0f32)
    } else {
        (1920, 1080, 2.0f32)
    };

    if let Some(w) = app.native_window() {
        let res = w.set_buffers_geometry(target_w, target_h, None);
        log::info!(
            "CleanFrame Display: Target {}x{} buffer set: {:?}. Real size: {}x{}, Physical 4K panel: {}",
            target_w, target_h, res, real_w, real_h, is_4k_panel
        );
    }

    DisplayInfo4K {
        is_4k_panel,
        width: target_w,
        height: target_h,
        scale_factor,
    }
}

#[cfg(target_os = "android")]
pub fn enforce_4k_surface(app: &slint::android::AndroidApp, win: &slint::Window, info: &DisplayInfo4K) {
    if let Some(w) = app.native_window() {
        if w.width() != info.width || w.height() != info.height {
            let res = w.set_buffers_geometry(info.width, info.height, None);
            log::info!(
                "CleanFrame Display: Enforcing {}x{} buffer geometry: {:?}",
                info.width, info.height, res
            );
        }
    }
    win.dispatch_event(slint::platform::WindowEvent::ScaleFactorChanged {
        scale_factor: info.scale_factor,
    });
    win.dispatch_event(slint::platform::WindowEvent::Resized {
        size: slint::LogicalSize::new(960.0, 540.0),
    });
}
