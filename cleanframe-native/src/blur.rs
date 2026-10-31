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

/// Downscales, blurs, and boosts saturation of any RGBA image on the fly, returning a Slint Image.
pub fn blur_rgba_image(
    img: &image::RgbaImage,
    target_width: u32,
    target_height: u32,
    radius: usize,
    saturation: f32,
) -> Image {
    let resized = image::imageops::resize(
        img,
        target_width,
        target_height,
        image::imageops::FilterType::Triangle,
    );
    let mut pixels = resized.into_raw();
    stack_blur_rgba(&mut pixels, target_width as usize, target_height as usize, radius);
    if saturation > 1.0 {
        boost_saturation(&mut pixels, saturation);
    }
    let mut buffer = SharedPixelBuffer::<Rgba8Pixel>::new(target_width, target_height);
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

/// Decodes any wallpaper bytes (JPEG/PNG), computes optical blur on the fly, and returns a Slint Image.
pub fn blur_wallpaper_bytes(bytes: &[u8]) -> Option<Image> {
    let img = image::load_from_memory(bytes).ok()?.to_rgba8();
    Some(blur_rgba_image(&img, 320, 180, 20, 1.35))
}
