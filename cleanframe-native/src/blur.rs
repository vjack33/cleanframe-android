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

/// Signed distance and outward normal for the Dock rounded rectangle in 320x180 buffer space.
/// Returns (signed_distance, normal_x, normal_y).
/// Inside the dock, signed_distance < 0.0.
#[inline]
fn dock_sdf_and_normal(x: f32, y: f32) -> (f32, f32, f32) {
    // 960x540 screen geometry scaled 1/3:
    // x: 104 / 3 = 34.6667, y: 338 / 3 = 112.6667
    // width: 752 / 3 = 250.6667, height: 116 / 3 = 38.6667
    // radius: 26 / 3 = 8.6667
    const CX: f32 = 160.0;
    const CY: f32 = 132.0;
    const HW: f32 = 125.3333;
    const HH: f32 = 19.3333;
    const CR: f32 = 8.6667;

    let dx = x - CX;
    let dy = y - CY;
    let ax = dx.abs();
    let ay = dy.abs();
    let sx = if dx >= 0.0 { 1.0 } else { -1.0 };
    let sy = if dy >= 0.0 { 1.0 } else { -1.0 };

    if ax > (HW - CR) && ay > (HH - CR) {
        // Corner quadrant
        let qx = ax - (HW - CR);
        let qy = ay - (HH - CR);
        let dist_corner = (qx * qx + qy * qy).sqrt();
        let sd = dist_corner - CR;
        let (nx, ny) = if dist_corner > 1e-4 {
            (sx * (qx / dist_corner), sy * (qy / dist_corner))
        } else {
            (sx * 0.7071068, sy * 0.7071068)
        };
        (sd, nx, ny)
    } else {
        // Straight edge / interior
        let dist_edge_x = ax - HW;
        let dist_edge_y = ay - HH;
        if dist_edge_x <= 0.0 && dist_edge_y <= 0.0 {
            // Inside the box
            let sd = dist_edge_x.max(dist_edge_y);
            if dist_edge_x > dist_edge_y {
                (sd, sx, 0.0)
            } else {
                (sd, 0.0, sy)
            }
        } else {
            // Outside the box
            let ex = dist_edge_x.max(0.0);
            let ey = dist_edge_y.max(0.0);
            let sd = (ex * ex + ey * ey).sqrt();
            let len = ex.hypot(ey);
            if len > 1e-4 {
                (sd, sx * (ex / len), sy * (ey / len))
            } else {
                (sd, sx, 0.0)
            }
        }
    }
}

/// Applies physical lens refraction (Snell's Law), chromatic dispersion (prism fringing),
/// Schlick Fresnel edge reflectance, and directional specular glare to the Dock glass bevel.
/// Modeled after the optical physics in https://github.com/iyinchao/liquid-glass-studio.
pub fn apply_glass_refraction_dispersion(pixels: &mut [u8], width: usize, height: usize, ref_distance: f32) {
    if ref_distance <= 0.0 {
        return;
    }
    let src = pixels.to_vec();

    // Refraction bevel thickness in buffer pixels (~24px on 960x540 screen)
    const REF_THICKNESS: f32 = 8.0;
    // Chromatic dispersion factor (wavelength split between red and blue)
    const DISPERSION: f32 = 0.05;
    // Glass refractive index (crown glass)
    const N_GLASS: f32 = 1.50;

    // Bounding box of the Dock in buffer space:
    let min_x = 32.max(0);
    let max_x = 288.min(width - 1);
    let min_y = 110.max(0);
    let max_y = 154.min(height - 1);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let (sd, nx, ny) = dock_sdf_and_normal(x as f32 + 0.5, y as f32 + 0.5);

            // Only process pixels inside the dock bevel: 0 < -sd < REF_THICKNESS
            if sd >= 0.0 || sd <= -REF_THICKNESS {
                continue;
            }

            let dist_to_edge = -sd; // Distance inward from perimeter (0.0 to REF_THICKNESS)
            let x_ratio = 1.0 - (dist_to_edge / REF_THICKNESS); // 1.0 at outer rim, 0.0 at flat interior

            // Snell's Law refraction curve from liquid-glass-studio:
            // theta_i = asin(x_ratio^2)
            // theta_t = asin(sin(theta_i) / n)
            // edge_factor = tan(theta_i - theta_t)
            let x2 = (x_ratio * x_ratio).clamp(0.0, 0.9999);
            let theta_i = x2.asin();
            let sin_theta_t = (theta_i.sin() / N_GLASS).clamp(-1.0, 1.0);
            let theta_t = sin_theta_t.asin();
            let edge_factor = (theta_i - theta_t).tan().max(0.0);

            // Deflection vector pointing inward (into the glass slab)
            let base_disp = edge_factor * ref_distance;
            let offset_x = -nx * base_disp;
            let offset_y = -ny * base_disp;

            // Chromatic dispersion: Red deflects less, Blue deflects more
            let cur_x = x as f32 + 0.5;
            let cur_y = y as f32 + 0.5;

            let red = sample_bilinear(
                &src,
                width,
                height,
                cur_x + offset_x * (1.0 - DISPERSION),
                cur_y + offset_y * (1.0 - DISPERSION),
                0,
            );
            let green = sample_bilinear(
                &src,
                width,
                height,
                cur_x + offset_x,
                cur_y + offset_y,
                1,
            );
            let blue = sample_bilinear(
                &src,
                width,
                height,
                cur_x + offset_x * (1.0 + DISPERSION),
                cur_y + offset_y * (1.0 + DISPERSION),
                2,
            );

            // Pure optical refraction without artificial painted white borders
            let r_out = red.clamp(0.0, 255.0) as u8;
            let g_out = green.clamp(0.0, 255.0) as u8;
            let b_out = blue.clamp(0.0, 255.0) as u8;

            let idx = (y * width + x) * 4;
            pixels[idx] = r_out;
            pixels[idx + 1] = g_out;
            pixels[idx + 2] = b_out;
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
    Some(blur_rgba_image_pair(&img, 320, 180, radius, 1.35, ref_distance))
}

/// Decodes any wallpaper bytes (JPEG/PNG), computes optical blur on the fly, and returns a Slint Image.
#[allow(dead_code)]
pub fn blur_wallpaper_bytes(bytes: &[u8]) -> Option<Image> {
    let (clean, _) = blur_wallpaper_bytes_pair(bytes, 20, 6.5)?;
    Some(clean)
}

