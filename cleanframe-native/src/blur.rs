// Dynamic on-the-fly optical blur engine for CleanFrame
// Computes dual-pass StackBlur and saturation boost for any wallpaper at runtime.

use slint::{Image, Rgba8Pixel, SharedPixelBuffer};

/// Mario Klingemann's StackBlur algorithm in pure Rust.
/// O(N) complexity independent of radius, runs in ~1.5ms for 320x180.
pub fn stack_blur_rgba(pixels: &mut [u8], width: usize, height: usize, radius: usize) {
    if radius < 1 || width == 0 || height == 0 {
        return;
    }

    let wm = width - 1;
    let hm = height - 1;
    let div = radius + radius + 1;

    let mut r = vec![0i32; width * height];
    let mut g = vec![0i32; width * height];
    let mut b = vec![0i32; width * height];
    let mut a = vec![0i32; width * height];

    for y in 0..height {
        let row_offset = y * width;
        for x in 0..width {
            let idx = (row_offset + x) * 4;
            r[row_offset + x] = pixels[idx] as i32;
            g[row_offset + x] = pixels[idx + 1] as i32;
            b[row_offset + x] = pixels[idx + 2] as i32;
            a[row_offset + x] = pixels[idx + 3] as i32;
        }
    }

    let mut stack = vec![[0i32; 4]; div];

    // Horizontal pass
    for y in 0..height {
        let mut r_sum = 0i32;
        let mut g_sum = 0i32;
        let mut b_sum = 0i32;
        let mut a_sum = 0i32;
        let mut r_in_sum = 0i32;
        let mut g_in_sum = 0i32;
        let mut b_in_sum = 0i32;
        let mut a_in_sum = 0i32;
        let mut r_out_sum = 0i32;
        let mut g_out_sum = 0i32;
        let mut b_out_sum = 0i32;
        let mut a_out_sum = 0i32;

        let p_first = y * width;
        let pr = r[p_first];
        let pg = g[p_first];
        let pb = b[p_first];
        let pa = a[p_first];

        for i in 0..=radius {
            stack[i] = [pr, pg, pb, pa];
            let mul = (i + 1) as i32;
            r_sum += pr * mul;
            g_sum += pg * mul;
            b_sum += pb * mul;
            a_sum += pa * mul;
            r_out_sum += pr;
            g_out_sum += pg;
            b_out_sum += pb;
            a_out_sum += pa;
        }

        for i in 1..=radius {
            let p = y * width + (i.min(wm));
            let pr = r[p];
            let pg = g[p];
            let pb = b[p];
            let pa = a[p];
            stack[i + radius] = [pr, pg, pb, pa];
            let mul = (radius + 1 - i) as i32;
            r_sum += pr * mul;
            g_sum += pg * mul;
            b_sum += pb * mul;
            a_sum += pa * mul;
            r_in_sum += pr;
            g_in_sum += pg;
            b_in_sum += pb;
            a_in_sum += pa;
        }

        let mut stack_pointer = radius;

        for x in 0..width {
            let idx = y * width + x;
            let div_sum = ((radius + 1) * (radius + 1)) as i32;
            r[idx] = r_sum / div_sum;
            g[idx] = g_sum / div_sum;
            b[idx] = b_sum / div_sum;
            a[idx] = a_sum / div_sum;

            r_sum -= r_out_sum;
            g_sum -= g_out_sum;
            b_sum -= b_out_sum;
            a_sum -= a_out_sum;

            let stack_start = (stack_pointer + div - radius) % div;
            let p_stack = stack[stack_start];
            r_out_sum -= p_stack[0];
            g_out_sum -= p_stack[1];
            b_out_sum -= p_stack[2];
            a_out_sum -= p_stack[3];

            let p_next = y * width + ((x + radius + 1).min(wm));
            stack[stack_start] = [r[p_next], g[p_next], b[p_next], a[p_next]];

            r_in_sum += r[p_next];
            g_in_sum += g[p_next];
            b_in_sum += b[p_next];
            a_in_sum += a[p_next];

            r_sum += r_in_sum;
            g_sum += g_in_sum;
            b_sum += b_in_sum;
            a_sum += a_in_sum;

            stack_pointer = (stack_pointer + 1) % div;
            let p_cur = stack[stack_pointer];
            r_out_sum += p_cur[0];
            g_out_sum += p_cur[1];
            b_out_sum += p_cur[2];
            a_out_sum += p_cur[3];

            r_in_sum -= p_cur[0];
            g_in_sum -= p_cur[1];
            b_in_sum -= p_cur[2];
            a_in_sum -= p_cur[3];
        }
    }

    // Vertical pass
    for x in 0..width {
        let mut r_sum = 0i32;
        let mut g_sum = 0i32;
        let mut b_sum = 0i32;
        let mut a_sum = 0i32;
        let mut r_in_sum = 0i32;
        let mut g_in_sum = 0i32;
        let mut b_in_sum = 0i32;
        let mut a_in_sum = 0i32;
        let mut r_out_sum = 0i32;
        let mut g_out_sum = 0i32;
        let mut b_out_sum = 0i32;
        let mut a_out_sum = 0i32;

        let pr = r[x];
        let pg = g[x];
        let pb = b[x];
        let pa = a[x];

        for i in 0..=radius {
            stack[i] = [pr, pg, pb, pa];
            let mul = (i + 1) as i32;
            r_sum += pr * mul;
            g_sum += pg * mul;
            b_sum += pb * mul;
            a_sum += pa * mul;
            r_out_sum += pr;
            g_out_sum += pg;
            b_out_sum += pb;
            a_out_sum += pa;
        }

        for i in 1..=radius {
            let p = (i.min(hm)) * width + x;
            let pr = r[p];
            let pg = g[p];
            let pb = b[p];
            let pa = a[p];
            stack[i + radius] = [pr, pg, pb, pa];
            let mul = (radius + 1 - i) as i32;
            r_sum += pr * mul;
            g_sum += pg * mul;
            b_sum += pb * mul;
            a_sum += pa * mul;
            r_in_sum += pr;
            g_in_sum += pg;
            b_in_sum += pb;
            a_in_sum += pa;
        }

        let mut stack_pointer = radius;

        for y in 0..height {
            let idx = y * width + x;
            let div_sum = ((radius + 1) * (radius + 1)) as i32;
            r[idx] = r_sum / div_sum;
            g[idx] = g_sum / div_sum;
            b[idx] = b_sum / div_sum;
            a[idx] = a_sum / div_sum;

            r_sum -= r_out_sum;
            g_sum -= g_out_sum;
            b_sum -= b_out_sum;
            a_sum -= a_out_sum;

            let stack_start = (stack_pointer + div - radius) % div;
            let p_stack = stack[stack_start];
            r_out_sum -= p_stack[0];
            g_out_sum -= p_stack[1];
            b_out_sum -= p_stack[2];
            a_out_sum -= p_stack[3];

            let p_next = ((y + radius + 1).min(hm)) * width + x;
            stack[stack_start] = [r[p_next], g[p_next], b[p_next], a[p_next]];

            r_in_sum += r[p_next];
            g_in_sum += g[p_next];
            b_in_sum += b[p_next];
            a_in_sum += a[p_next];

            r_sum += r_in_sum;
            g_sum += g_in_sum;
            b_sum += b_in_sum;
            a_sum += a_in_sum;

            stack_pointer = (stack_pointer + 1) % div;
            let p_cur = stack[stack_pointer];
            r_out_sum += p_cur[0];
            g_out_sum += p_cur[1];
            b_out_sum += p_cur[2];
            a_out_sum += p_cur[3];

            r_in_sum -= p_cur[0];
            g_in_sum -= p_cur[1];
            b_in_sum -= p_cur[2];
            a_in_sum -= p_cur[3];
        }
    }

    for y in 0..height {
        let row_offset = y * width;
        for x in 0..width {
            let idx = (row_offset + x) * 4;
            pixels[idx] = r[row_offset + x].clamp(0, 255) as u8;
            pixels[idx + 1] = g[row_offset + x].clamp(0, 255) as u8;
            pixels[idx + 2] = b[row_offset + x].clamp(0, 255) as u8;
            pixels[idx + 3] = a[row_offset + x].clamp(0, 255) as u8;
        }
    }
}

/// Optical vibrance / saturation boost to ensure diffused wallpaper colors glow through glass.
pub fn boost_saturation(pixels: &mut [u8], factor: f32) {
    for chunk in pixels.chunks_exact_mut(4) {
        let r = chunk[0] as f32;
        let g = chunk[1] as f32;
        let b = chunk[2] as f32;
        let lum = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        chunk[0] = (lum + (r - lum) * factor).clamp(0.0, 255.0) as u8;
        chunk[1] = (lum + (g - lum) * factor).clamp(0.0, 255.0) as u8;
        chunk[2] = (lum + (b - lum) * factor).clamp(0.0, 255.0) as u8;
    }
}

/// Bilinear sampling of a single color channel from an RGBA buffer.
#[inline(always)]
fn sample_bilinear(pixels: &[u8], width: usize, height: usize, x: f32, y: f32, c: usize) -> f32 {
    let x = x.clamp(0.0, (width - 1) as f32);
    let y = y.clamp(0.0, (height - 1) as f32);
    let x0 = x.floor() as usize;
    let y0 = y.floor() as usize;
    let x1 = (x0 + 1).min(width - 1);
    let y1 = (y0 + 1).min(height - 1);
    let fx = x - x0 as f32;
    let fy = y - y0 as f32;

    let p00 = pixels[(y0 * width + x0) * 4 + c] as f32;
    let p10 = pixels[(y0 * width + x1) * 4 + c] as f32;
    let p01 = pixels[(y1 * width + x0) * 4 + c] as f32;
    let p11 = pixels[(y1 * width + x1) * 4 + c] as f32;

    let top = p00 * (1.0 - fx) + p10 * fx;
    let bot = p01 * (1.0 - fx) + p11 * fx;
    top * (1.0 - fy) + bot * fy
}

#[allow(dead_code)]
#[inline(always)]
fn erf_approx(x: f32) -> f32 {
    let c = x.clamp(-4.0, 4.0);
    (c * (1.1283792 + 0.1003 * c * c)).tanh()
}

#[allow(dead_code)]
#[inline(always)]
fn phi(x: f32, sigma: f32) -> f32 {
    0.5 + 0.5 * erf_approx(x / (sigma.max(1e-3) * 1.4142135))
}

/// Signed distance and outward normal for a squircle with Apple continuous corners (exponent n = 2.79).
#[inline(always)]
pub fn sdg_squircle(dx: f32, dy: f32, hw: f32, hh: f32, r: f32) -> (f32, f32, f32) {
    let ax = dx.abs();
    let ay = dy.abs();
    let sx = if dx >= 0.0 { 1.0 } else { -1.0 };
    let sy = if dy >= 0.0 { 1.0 } else { -1.0 };

    let qx = ax - (hw - r);
    let qy = ay - (hh - r);

    const N: f32 = 2.79;
    if qx > 0.0 && qy > 0.0 {
        let qx_pow = qx.powf(N);
        let qy_pow = qy.powf(N);
        let l = (qx_pow + qy_pow).powf(1.0 / N);
        let gx = (qx / l).powf(N - 1.0);
        let gy = (qy / l).powf(N - 1.0);
        let gl = (gx * gx + gy * gy).sqrt().max(1e-5);
        let sd = (l - r) / gl;
        (sd, sx * gx / gl, sy * gy / gl)
    } else if qx > qy {
        (qx - r, sx, 0.0)
    } else {
        (qy - r, 0.0, sy)
    }
}

/// Signed distance and outward normal for the Dock squircle in 960x540 buffer space.
/// Returns (signed_distance, normal_x, normal_y).
/// Inside the dock, signed_distance < 0.0.
#[inline(always)]
fn dock_sdf_and_normal(x: f32, y: f32) -> (f32, f32, f32) {
    // 960x540 screen geometry (exact 1:1 Slint viewport coordinates):
    // Dock is x: 104, y: 338, width: 752, height: 116, radius: 26
    const CX: f32 = 480.0;
    const CY: f32 = 396.0;
    const HW: f32 = 376.0;
    const HH: f32 = 58.0;
    const CR: f32 = 26.0;

    sdg_squircle(x - CX, y - CY, HW, HH, CR)
}

/// Squircle bevel slope: h(t) = (1 - (1 - t)^4)^(1/4).
/// Exact flat-to-curve profile from Apple iOS kit and liquid-rust.
#[inline(always)]
fn bevel_slope(t: f32) -> f32 {
    let u = 1.0 - t;
    let u3 = u * u * u;
    u3 * (1.0 - u3 * u).max(1e-5).powf(-0.75)
}

/// Lateral travel of a vertical ray refracted by a surface tilted by atan(slope).
/// Exact Snell's Law implementation from liquid-rust glass.wgsl.
#[inline(always)]
fn refract_offset(slope: f32, eta: f32) -> f32 {
    let theta = slope.atan();
    let sin_theta = theta.sin();
    (theta - (sin_theta / eta).clamp(-1.0, 1.0).asin()).tan()
}

/// Applies physical lens refraction (Snell's Law), chromatic dispersion (prism fringing),
/// and continuous curvature to the Dock glass bevel.
/// Modeled after optical physics in Apple Liquid Glass and liquid-rust (al1h3n/liquid-rust).
pub fn apply_glass_refraction_dispersion(pixels: &mut [u8], width: usize, height: usize, ref_distance: f32) {
    if ref_distance <= 0.0 {
        return;
    }
    let src = pixels.to_vec();

    // Refraction bevel thickness: 24 pixels on a 960x540 screen (~48px on 1080p TV)
    const REF_THICKNESS: f32 = 24.0;
    // Virtual glass index of refraction from liquid-rust
    const ETA: f32 = 1.5;
    // Chromatic dispersion factor from liquid-rust (optics.z * 0.25)
    const DISPERSION: f32 = 0.20;

    // Bounding box of the Dock in 960x540 buffer space:
    let min_x = 100.max(0);
    let max_x = 860.min(width - 1);
    let min_y = 334.max(0);
    let max_y = 458.min(height - 1);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let (sd, nx, ny) = dock_sdf_and_normal(x as f32 + 0.5, y as f32 + 0.5);

            // Only process pixels inside the dock bevel: 0 < -sd < REF_THICKNESS
            if sd >= 0.0 || sd <= -REF_THICKNESS {
                continue;
            }

            let dist_to_edge = -sd; // 0.0 at outer rim, REF_THICKNESS at flat interior
            let t = (dist_to_edge / REF_THICKNESS).clamp(0.0, 1.0);

            // liquid-rust squircle bevel slope & Snell's law refraction
            let slope = bevel_slope(t);
            // Smooth edge transition over the first 2 pixels so displacement meets wallpaper seamlessly
            let edge_blend = (t * 8.0).min(1.0);
            let reach = ref_distance * edge_blend;

            let off_g = reach * refract_offset(slope, ETA);
            let off_r = reach * refract_offset(slope, (ETA - DISPERSION).max(1.01));
            let off_b = reach * refract_offset(slope, ETA + DISPERSION);

            let cur_x = x as f32 + 0.5;
            let cur_y = y as f32 + 0.5;

            // Deflection vector pointing inward (into the glass slab along normal)
            let red = sample_bilinear(
                &src,
                width,
                height,
                cur_x - nx * off_r,
                cur_y - ny * off_r,
                0,
            );
            let green = sample_bilinear(
                &src,
                width,
                height,
                cur_x - nx * off_g,
                cur_y - ny * off_g,
                1,
            );
            let blue = sample_bilinear(
                &src,
                width,
                height,
                cur_x - nx * off_b,
                cur_y - ny * off_b,
                2,
            );

            let idx = (y * width + x) * 4;
            pixels[idx] = red.clamp(0.0, 255.0) as u8;
            pixels[idx + 1] = green.clamp(0.0, 255.0) as u8;
            pixels[idx + 2] = blue.clamp(0.0, 255.0) as u8;
        }
    }
}

fn pixels_to_image(pixels: &[u8], width: u32, height: u32) -> Image {
    let mut buffer = SharedPixelBuffer::<Rgba8Pixel>::new(width, height);
    let slice = buffer.make_mut_slice();
    for (i, chunk) in pixels.chunks_exact(4).enumerate() {
        slice[i] = Rgba8Pixel {
            r: chunk[0],
            g: chunk[1],
            b: chunk[2],
            a: chunk[3],
        };
    }
    Image::from_rgba8(buffer)
}

/// Downscales, blurs, and boosts saturation of any RGBA image on the fly,
/// returning both the clean blurred image (for general panels/settings)
/// and the optically refracted & chromatic-dispersed image (for the Dock).
pub fn blur_rgba_image_pair(
    img: &image::RgbaImage,
    target_width: u32,
    target_height: u32,
    radius: usize,
    saturation: f32,
    ref_distance: f32,
) -> (Image, Image) {
    let resized = image::imageops::resize(
        img,
        target_width,
        target_height,
        image::imageops::FilterType::Triangle,
    );
    let mut pixels = resized.into_raw();
    if radius > 0 {
        stack_blur_rgba(&mut pixels, target_width as usize, target_height as usize, radius);
        if saturation > 1.0 {
            boost_saturation(&mut pixels, saturation);
        }
    }

    // 1. Clean frost for general panels and settings modal
    let clean_img = pixels_to_image(&pixels, target_width, target_height);

    // 2. Optically refracted and chromatic-dispersed frost for the Dock
    let mut dock_pixels = pixels;
    if ref_distance > 0.0 {
        apply_glass_refraction_dispersion(
            &mut dock_pixels,
            target_width as usize,
            target_height as usize,
            ref_distance,
        );
    }
    let dock_img = pixels_to_image(&dock_pixels, target_width, target_height);

    (clean_img, dock_img)
}

/// Downscales, blurs, and boosts saturation of any RGBA image on the fly, returning a Slint Image.
#[allow(dead_code)]
pub fn blur_rgba_image(
    img: &image::RgbaImage,
    target_width: u32,
    target_height: u32,
    radius: usize,
    saturation: f32,
) -> Image {
    let (clean, _) = blur_rgba_image_pair(img, target_width, target_height, radius, saturation, 6.5);
    clean
}

/// Decodes any wallpaper bytes (JPEG/PNG), computes optical blur on the fly,
/// returning (clean_frost, dock_refracted_frost).
pub fn blur_wallpaper_bytes_pair(bytes: &[u8], radius: usize, ref_distance: f32) -> Option<(Image, Image)> {
    let img = image::load_from_memory(bytes).ok()?.to_rgba8();
    Some(blur_rgba_image_pair(&img, 960, 540, radius, 1.35, ref_distance))
}

/// Decodes any wallpaper bytes (JPEG/PNG), computes optical blur on the fly, and returns a Slint Image.
#[allow(dead_code)]
pub fn blur_wallpaper_bytes(bytes: &[u8]) -> Option<Image> {
    let (clean, _) = blur_wallpaper_bytes_pair(bytes, 40, 28.0)?;
    Some(clean)
}

/// Renders an optical liquid glass focus lens texture with Snell's law refraction,
/// chromatic dispersion (prism fringing), directional specular rim glint,
/// soft inner rim shadow, and frost blur diffusion based on liquid-rust.
pub fn render_focus_lens_texture(
    width: u32,
    height: u32,
    radius: f32,
    refraction_dist: f32,
    _blur_radius: f32,
    material: &str,
) -> Image {
    let mut buffer = SharedPixelBuffer::<Rgba8Pixel>::new(width, height);
    let slice = buffer.make_mut_slice();

    let cx = width as f32 * 0.5;
    let cy = height as f32 * 0.5;
    let padding = 10.0f32;
    let hw = cx - padding;
    let hh = cy - padding;
    let depth = (16.0 + (refraction_dist / 48.0) * 32.0).clamp(16.0, 48.0);
    const ETA: f32 = 1.5;
    const DISPERSION: f32 = 0.20;

    let (base_rgb, base_alpha) = match material {
        "Obsidian Tint" => ([16.0f32, 22.0, 32.0], 0.14),
        "Clear" => ([255.0, 255.0, 255.0], 0.035), // Apple kit Clear tone lift (lift: 0.063 in liquid-rust, crystal clear, zero milky haze)
        "Regular" => ([240.0, 245.0, 255.0], 0.05),
        "Tinted" => ([0.0, 136.0, 255.0], 0.16),
        "Soft" => ([245.0, 242.0, 238.0], 0.04),
        _ /* Frost */ => ([235.0, 245.0, 255.0], 0.05),
    };

    for y in 0..height {
        let py = y as f32 + 0.5;
        let dy = py - cy;
        let row_offset = (y * width) as usize;

        for x in 0..width {
            let px = x as f32 + 0.5;
            let dx = px - cx;

            let (sd, _nx, ny) = sdg_squircle(dx, dy, hw, hh, radius);

            // Outside the lens boundary: 100% transparent (no crude drop shadow or dark border)
            if sd >= 0.0 {
                slice[row_offset + x as usize] = Rgba8Pixel { r: 0, g: 0, b: 0, a: 0 };
                continue;
            }

            let cov = (0.5 - sd).clamp(0.0, 1.0);
            let dist_to_edge = -sd;
            let t = (dist_to_edge / depth).clamp(0.0, 1.0);

            // Flat interior of lens (t >= 1.0): crystal clear glass tone lift
            if t >= 1.0 {
                slice[row_offset + x as usize] = Rgba8Pixel {
                    r: base_rgb[0] as u8,
                    g: base_rgb[1] as u8,
                    b: base_rgb[2] as u8,
                    a: (base_alpha * 255.0 * cov) as u8,
                };
                continue;
            }

            // Directional overhead specular glint: only on upper crown facing light L=(0,-1).
            // Zero on left/right/bottom (NO WHITE BORDER!).
            let facing = -ny;
            let specular = if facing > 0.25 && dy < 0.0 {
                let glint = (sd / 1.0).exp() * facing.powi(2);
                glint * 80.0
            } else {
                0.0
            };

            // Ambient bottom catch-light (very subtle soft ground bounce)
            let bottom_rim = if facing < -0.35 && dy > 0.0 {
                let glint = (sd / 1.0).exp() * (-facing).powi(2);
                glint * 24.0
            } else {
                0.0
            };

            // Outer refractive rim groove (matching liquid-rust inner shadow / edge occlusion)
            let rim_groove = if dist_to_edge < 2.4 {
                (1.0 - dist_to_edge / 2.4) * 32.0
            } else {
                0.0
            };

            // Snell's Law Refraction & Chromatic Dispersion prism fringing
            let mut cyan_fringe = 0.0f32;
            let mut amber_fringe = 0.0f32;

            if refraction_dist > 0.0 && t < 1.0 {
                let slope = bevel_slope(t);
                let _off_g = refraction_dist * refract_offset(slope, ETA);
                let off_r = refraction_dist * refract_offset(slope, (ETA - DISPERSION).max(1.01));
                let off_b = refraction_dist * refract_offset(slope, ETA + DISPERSION);

                let disp_sep = (off_b - off_r).abs();
                let cyan_factor = (1.0 - (t / 0.38).min(1.0)).powi(2) * (disp_sep / 3.0).min(1.0);
                cyan_fringe = cyan_factor * 28.0;

                let amber_factor = (1.0 - ((t - 0.45) / 0.25).abs()).max(0.0) * (disp_sep / 3.0).min(1.0);
                amber_fringe = amber_factor * 22.0;
            }

            let r_prism = amber_fringe;
            let g_prism = cyan_fringe * 0.72 + amber_fringe * 0.55;
            let b_prism = cyan_fringe * 1.15 + amber_fringe * 0.05;

            let light_col = specular + bottom_rim;
            let cur_r = (base_rgb[0] + light_col + r_prism - rim_groove).clamp(0.0, 255.0);
            let cur_g = (base_rgb[1] + light_col + g_prism - rim_groove).clamp(0.0, 255.0);
            let cur_b = (base_rgb[2] + light_col + b_prism - rim_groove).clamp(0.0, 255.0);

            let disp_alpha = cyan_fringe * 0.20 + amber_fringe * 0.16;
            let edge_alpha = specular * 0.32 + bottom_rim * 0.18 + disp_alpha + rim_groove * 0.20;
            let final_a = ((base_alpha * 255.0 + edge_alpha * (1.0 - t)) * cov).clamp(0.0, 255.0) as u8;

            slice[row_offset + x as usize] = Rgba8Pixel {
                r: cur_r as u8,
                g: cur_g as u8,
                b: cur_b as u8,
                a: final_a,
            };
        }
    }

    Image::from_rgba8(buffer)
}

/// Signed distance and outward normal for a perfect circle.
#[inline(always)]
pub fn sdg_circle(dx: f32, dy: f32, r: f32) -> (f32, f32, f32) {
    let dist = (dx * dx + dy * dy).sqrt();
    let sd = dist - r;
    let (nx, ny) = if dist > 1e-4 {
        (dx / dist, dy / dist)
    } else {
        (0.0, -1.0)
    };
    (sd, nx, ny)
}

/// Renders a circular optical liquid glass focus lens texture with Snell's law refraction,
/// 360° radial chromatic dispersion, directional spherical dome specular glint, and edge hairline burn.
pub fn render_round_focus_lens_texture(
    size: u32,
    radius: f32,
    refraction_dist: f32,
    _blur_radius: f32,
    material: &str,
) -> Image {
    let mut buffer = SharedPixelBuffer::<Rgba8Pixel>::new(size, size);
    let slice = buffer.make_mut_slice();

    let cx = size as f32 * 0.5;
    let cy = size as f32 * 0.5;
    let depth = (14.0 + (refraction_dist / 48.0) * 28.0).clamp(14.0, 42.0);
    const ETA: f32 = 1.5;
    const DISPERSION: f32 = 0.20;

    let (base_rgb, base_alpha) = match material {
        "Obsidian Tint" => ([16.0f32, 22.0, 32.0], 0.14),
        "Clear" => ([255.0, 255.0, 255.0], 0.035), // Apple kit Clear tone lift (lift: 0.063 in liquid-rust, crystal clear, zero milky haze)
        "Regular" => ([240.0, 245.0, 255.0], 0.05),
        "Tinted" => ([0.0, 136.0, 255.0], 0.16),
        "Soft" => ([245.0, 242.0, 238.0], 0.04),
        _ /* Frost */ => ([235.0, 245.0, 255.0], 0.05),
    };

    for y in 0..size {
        let py = y as f32 + 0.5;
        let dy = py - cy;
        let row_offset = (y * size) as usize;

        for x in 0..size {
            let px = x as f32 + 0.5;
            let dx = px - cx;

            let (sd, _nx, ny) = sdg_circle(dx, dy, radius);

            // Outside the lens boundary: 100% transparent (no crude drop shadow or dark border)
            if sd >= 0.0 {
                slice[row_offset + x as usize] = Rgba8Pixel { r: 0, g: 0, b: 0, a: 0 };
                continue;
            }

            let cov = (0.5 - sd).clamp(0.0, 1.0);
            let dist_to_edge = -sd;
            let t = (dist_to_edge / depth).clamp(0.0, 1.0);

            // Flat interior of lens (t >= 1.0): crystal clear glass tone lift
            if t >= 1.0 {
                slice[row_offset + x as usize] = Rgba8Pixel {
                    r: base_rgb[0] as u8,
                    g: base_rgb[1] as u8,
                    b: base_rgb[2] as u8,
                    a: (base_alpha * 255.0 * cov) as u8,
                };
                continue;
            }

            // Directional overhead specular glint: only on upper crown facing light L=(0,-1).
            // Zero on left/right/bottom (NO WHITE BORDER!).
            let facing = -ny;
            let specular = if facing > 0.25 && dy < 0.0 {
                let glint = (sd / 1.0).exp() * facing.powi(2);
                glint * 80.0
            } else {
                0.0
            };

            // Ambient bottom catch-light (very subtle soft ground bounce)
            let bottom_rim = if facing < -0.35 && dy > 0.0 {
                let glint = (sd / 1.0).exp() * (-facing).powi(2);
                glint * 24.0
            } else {
                0.0
            };

            // Outer refractive rim groove (matching liquid-rust inner shadow / edge occlusion)
            let rim_groove = if dist_to_edge < 2.4 {
                (1.0 - dist_to_edge / 2.4) * 32.0
            } else {
                0.0
            };

            // Snell Refraction & Chromatic Dispersion prism fringing
            let mut cyan_fringe = 0.0f32;
            let mut amber_fringe = 0.0f32;

            if refraction_dist > 0.0 && t < 1.0 {
                let slope = bevel_slope(t);
                let _off_g = refraction_dist * refract_offset(slope, ETA);
                let off_r = refraction_dist * refract_offset(slope, (ETA - DISPERSION).max(1.01));
                let off_b = refraction_dist * refract_offset(slope, ETA + DISPERSION);

                let disp_sep = (off_b - off_r).abs();
                let cyan_factor = (1.0 - (t / 0.38).min(1.0)).powi(2) * (disp_sep / 3.0).min(1.0);
                cyan_fringe = cyan_factor * 28.0;

                let amber_factor = (1.0 - ((t - 0.45) / 0.25).abs()).max(0.0) * (disp_sep / 3.0).min(1.0);
                amber_fringe = amber_factor * 22.0;
            }

            let r_prism = amber_fringe;
            let g_prism = cyan_fringe * 0.72 + amber_fringe * 0.55;
            let b_prism = cyan_fringe * 1.15 + amber_fringe * 0.05;

            let light_col = specular + bottom_rim;
            let cur_r = (base_rgb[0] + light_col + r_prism - rim_groove).clamp(0.0, 255.0);
            let cur_g = (base_rgb[1] + light_col + g_prism - rim_groove).clamp(0.0, 255.0);
            let cur_b = (base_rgb[2] + light_col + b_prism - rim_groove).clamp(0.0, 255.0);

            let disp_alpha = cyan_fringe * 0.20 + amber_fringe * 0.16;
            let edge_alpha = specular * 0.32 + bottom_rim * 0.18 + disp_alpha + rim_groove * 0.20;
            let final_a = ((base_alpha * 255.0 + edge_alpha * (1.0 - t)) * cov).clamp(0.0, 255.0) as u8;

            slice[row_offset + x as usize] = Rgba8Pixel {
                r: cur_r as u8,
                g: cur_g as u8,
                b: cur_b as u8,
                a: final_a,
            };
        }
    }

    Image::from_rgba8(buffer)
}


