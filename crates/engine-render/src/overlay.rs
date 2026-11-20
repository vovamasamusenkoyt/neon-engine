use crate::vertex::Vertex3D;

pub fn append_quad(
    verts: &mut Vec<Vertex3D>,
    inds: &mut Vec<u32>,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    color: [f32; 3],
) {
    let base = verts.len() as u32;
    let z = 0.0;
    verts.push(Vertex3D::new([x0, y0, z], color));
    verts.push(Vertex3D::new([x1, y0, z], color));
    verts.push(Vertex3D::new([x1, y1, z], color));
    verts.push(Vertex3D::new([x0, y1, z], color));

    inds.extend_from_slice(&[
        base, base + 1, base + 2,
        base, base + 2, base + 3,
        base, base + 2, base + 1,
        base, base + 3, base + 2,
    ]);
}

pub fn append_char_7seg(
    verts: &mut Vec<Vertex3D>,
    inds: &mut Vec<u32>,
    ch: char,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: [f32; 3],
) {
    let t = h * 0.16;
    let half_h = (h - t) * 0.5;

    let seg_mask = match ch {
        '0' => 0b00111111,
        '1' => 0b00000110,
        '2' => 0b01011011,
        '3' => 0b01001111,
        '4' => 0b01100110,
        '5' => 0b01101101,
        '6' => 0b01111101,
        '7' => 0b00000111,
        '8' => 0b01111111,
        '9' => 0b01101111,
        'F' => 0b01110001,
        'P' => 0b01110011,
        'S' => 0b01101101,
        _ => 0,
    };

    if ch == ':' {
        append_quad(verts, inds, x + w * 0.35, y + h * 0.25, x + w * 0.35 + t, y + h * 0.25 + t, color);
        append_quad(verts, inds, x + w * 0.35, y + h * 0.65, x + w * 0.35 + t, y + h * 0.65 + t, color);
        return;
    }
    if ch == '.' {
        append_quad(verts, inds, x + w * 0.3, y + h - t, x + w * 0.3 + t, y + h, color);
        return;
    }

    if seg_mask & 1 != 0 {
        append_quad(verts, inds, x, y, x + w, y + t, color);
    }
    if seg_mask & 2 != 0 {
        append_quad(verts, inds, x + w - t, y, x + w, y + half_h + t * 0.5, color);
    }
    if seg_mask & 4 != 0 {
        append_quad(verts, inds, x + w - t, y + half_h, x + w, y + h, color);
    }
    if seg_mask & 8 != 0 {
        append_quad(verts, inds, x, y + h - t, x + w, y + h, color);
    }
    if seg_mask & 16 != 0 {
        append_quad(verts, inds, x, y + half_h, x + t, y + h, color);
    }
    if seg_mask & 32 != 0 {
        append_quad(verts, inds, x, y, x + t, y + half_h + t * 0.5, color);
    }
    if seg_mask & 64 != 0 {
        append_quad(verts, inds, x, y + half_h - t * 0.5, x + w, y + half_h + t * 0.5, color);
    }
}

pub fn build_text_overlay(text: &str) -> (Vec<Vertex3D>, Vec<u32>) {
    let mut verts = Vec::new();
    let mut inds = Vec::new();

    let bg_color = [0.02, 0.03, 0.05];
    append_quad(&mut verts, &mut inds, -0.96, -0.94, -0.38, -0.78, bg_color);

    let text_color = [0.1, 1.0, 0.35];
    let mut cur_x = -0.94;
    let cur_y = -0.92;
    let char_w = 0.038;
    let char_h = 0.110;
    let spacing = 0.012;

    for ch in text.chars() {
        if ch == ' ' {
            cur_x += char_w * 0.5;
            continue;
        }
        append_char_7seg(&mut verts, &mut inds, ch, cur_x, cur_y, char_w, char_h, text_color);
        cur_x += char_w + spacing;
    }

    (verts, inds)
}
