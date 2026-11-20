use engine_core::{
    CMenuManager, MenuPage, DARKMENUOPTION_COLOR, HEADER_SHADOW_COLOR, LABEL_COLOR,
    SCREEN_HEIGHT, SCREEN_WIDTH, SELECTIONBORDER_COLOR, SLIDEROFF_COLOR, SLIDERON_COLOR,
};
use crate::vertex::Vertex3D;

fn virt_to_ndc(vx: f32, vy: f32) -> [f32; 2] {
    [
        (vx / SCREEN_WIDTH) * 2.0 - 1.0,
        (vy / SCREEN_HEIGHT) * 2.0 - 1.0,
    ]
}

pub fn append_polygon_4(
    verts: &mut Vec<Vertex3D>,
    inds: &mut Vec<u32>,
    p0: [f32; 2],
    p1: [f32; 2],
    p2: [f32; 2],
    p3: [f32; 2],
    color: [f32; 3],
) {
    let base = verts.len() as u32;
    let n0 = virt_to_ndc(p0[0], p0[1]);
    let n1 = virt_to_ndc(p1[0], p1[1]);
    let n2 = virt_to_ndc(p2[0], p2[1]);
    let n3 = virt_to_ndc(p3[0], p3[1]);
    let z = 0.0;

    verts.push(Vertex3D::new([n0[0], n0[1], z], color));
    verts.push(Vertex3D::new([n1[0], n1[1], z], color));
    verts.push(Vertex3D::new([n2[0], n2[1], z], color));
    verts.push(Vertex3D::new([n3[0], n3[1], z], color));

    inds.extend_from_slice(&[
        base, base + 1, base + 2,
        base, base + 2, base + 3,
        base, base + 2, base + 1,
        base, base + 3, base + 2,
    ]);
}

pub fn append_rect(
    verts: &mut Vec<Vertex3D>,
    inds: &mut Vec<u32>,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    color: [f32; 3],
) {
    append_polygon_4(
        verts,
        inds,
        [x0, y0],
        [x1, y0],
        [x1, y1],
        [x0, y1],
        color,
    );
}

// Clean vector font replicating reVC CFont
fn append_stroke(
    verts: &mut Vec<Vertex3D>,
    inds: &mut Vec<u32>,
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
    color: [f32; 3],
) {
    append_polygon_4(verts, inds, [x0, y0], [x1, y0], [x1, y1], [x0, y1], color);
}

fn append_char(
    verts: &mut Vec<Vertex3D>,
    inds: &mut Vec<u32>,
    ch: char,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: [f32; 3],
) {
    let t = (h * 0.17).max(1.8);
    let half_h = (h - t) * 0.5;

    let ch = ch.to_ascii_uppercase();
    match ch {
        'A' => {
            append_stroke(verts, inds, x, y + t, x + t, y + h, color);
            append_stroke(verts, inds, x + w - t, y + t, x + w, y + h, color);
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_stroke(verts, inds, x, y + half_h, x + w, y + half_h + t, color);
        }
        'B' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x, y, x + w - t, y + t, color);
            append_stroke(verts, inds, x, y + half_h, x + w - t, y + half_h + t, color);
            append_stroke(verts, inds, x, y + h - t, x + w - t, y + h, color);
            append_stroke(verts, inds, x + w - t, y + t, x + w, y + half_h, color);
            append_stroke(verts, inds, x + w - t, y + half_h + t, x + w, y + h - t, color);
        }
        'C' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_stroke(verts, inds, x, y + h - t, x + w, y + h, color);
        }
        'D' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x, y, x + w - t, y + t, color);
            append_stroke(verts, inds, x, y + h - t, x + w - t, y + h, color);
            append_stroke(verts, inds, x + w - t, y + t, x + w, y + h - t, color);
        }
        'E' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_stroke(verts, inds, x, y + half_h, x + w * 0.75, y + half_h + t, color);
            append_stroke(verts, inds, x, y + h - t, x + w, y + h, color);
        }
        'F' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_stroke(verts, inds, x, y + half_h, x + w * 0.7, y + half_h + t, color);
        }
        'G' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_stroke(verts, inds, x, y + h - t, x + w, y + h, color);
            append_stroke(verts, inds, x + w - t, y + half_h, x + w, y + h, color);
            append_stroke(verts, inds, x + w * 0.5, y + half_h, x + w, y + half_h + t, color);
        }
        'H' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x + w - t, y, x + w, y + h, color);
            append_stroke(verts, inds, x, y + half_h, x + w, y + half_h + t, color);
        }
        'I' => {
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_stroke(verts, inds, x, y + h - t, x + w, y + h, color);
            append_stroke(verts, inds, x + (w - t) * 0.5, y, x + (w + t) * 0.5, y + h, color);
        }
        'K' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_polygon_4(verts, inds, [x + t, y + half_h], [x + w, y], [x + w, y + t], [x + t, y + half_h + t], color);
            append_polygon_4(verts, inds, [x + t, y + half_h], [x + w, y + h - t], [x + w, y + h], [x + t, y + half_h + t], color);
        }
        'L' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x, y + h - t, x + w, y + h, color);
        }
        'M' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x + w - t, y, x + w, y + h, color);
            append_polygon_4(verts, inds, [x, y], [x + w * 0.5, y + half_h], [x + w * 0.5, y + half_h + t], [x + t, y], color);
            append_polygon_4(verts, inds, [x + w, y], [x + w * 0.5, y + half_h], [x + w * 0.5, y + half_h + t], [x + w - t, y], color);
        }
        'N' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x + w - t, y, x + w, y + h, color);
            append_polygon_4(verts, inds, [x, y], [x + w - t, y + h], [x + w, y + h], [x + t, y], color);
        }
        'O' | '0' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x + w - t, y, x + w, y + h, color);
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_stroke(verts, inds, x, y + h - t, x + w, y + h, color);
        }
        'P' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_stroke(verts, inds, x, y + half_h, x + w, y + half_h + t, color);
            append_stroke(verts, inds, x + w - t, y, x + w, y + half_h, color);
        }
        'Q' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x + w - t, y, x + w, y + h, color);
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_stroke(verts, inds, x, y + h - t, x + w, y + h, color);
            append_polygon_4(verts, inds, [x + w * 0.5, y + half_h], [x + w, y + h], [x + w, y + h + t * 0.5], [x + w * 0.5, y + half_h + t], color);
        }
        'R' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_stroke(verts, inds, x, y + half_h, x + w, y + half_h + t, color);
            append_stroke(verts, inds, x + w - t, y, x + w, y + half_h, color);
            append_polygon_4(verts, inds, [x + t, y + half_h], [x + w - t, y + h], [x + w, y + h], [x + t + t, y + half_h], color);
        }
        'S' | '5' => {
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_stroke(verts, inds, x, y, x + t, y + half_h, color);
            append_stroke(verts, inds, x, y + half_h, x + w, y + half_h + t, color);
            append_stroke(verts, inds, x + w - t, y + half_h, x + w, y + h, color);
            append_stroke(verts, inds, x, y + h - t, x + w, y + h, color);
        }
        'T' => {
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_stroke(verts, inds, x + (w - t) * 0.5, y, x + (w + t) * 0.5, y + h, color);
        }
        'U' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x + w - t, y, x + w, y + h, color);
            append_stroke(verts, inds, x, y + h - t, x + w, y + h, color);
        }
        'V' => {
            append_polygon_4(verts, inds, [x, y], [x + w * 0.5 - t * 0.5, y + h], [x + w * 0.5 + t * 0.5, y + h], [x + t, y], color);
            append_polygon_4(verts, inds, [x + w, y], [x + w * 0.5 + t * 0.5, y + h], [x + w * 0.5 - t * 0.5, y + h], [x + w - t, y], color);
        }
        'W' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x + w - t, y, x + w, y + h, color);
            append_polygon_4(verts, inds, [x + t, y + h], [x + w * 0.5, y + half_h], [x + w * 0.5, y + half_h + t], [x + t, y + h - t], color);
            append_polygon_4(verts, inds, [x + w - t, y + h], [x + w * 0.5, y + half_h], [x + w * 0.5, y + half_h + t], [x + w - t, y + h - t], color);
        }
        'X' => {
            append_polygon_4(verts, inds, [x, y], [x + w, y + h], [x + w - t, y + h], [x, y + t], color);
            append_polygon_4(verts, inds, [x + w, y], [x, y + h], [x + t, y + h], [x + w, y + t], color);
        }
        'Y' => {
            append_polygon_4(verts, inds, [x, y], [x + w * 0.5, y + half_h], [x + w * 0.5 + t, y + half_h], [x + t, y], color);
            append_polygon_4(verts, inds, [x + w, y], [x + w * 0.5, y + half_h], [x + w * 0.5 - t, y + half_h], [x + w - t, y], color);
            append_stroke(verts, inds, x + (w - t) * 0.5, y + half_h, x + (w + t) * 0.5, y + h, color);
        }
        'Z' | '2' => {
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_polygon_4(verts, inds, [x + w, y], [x, y + h], [x + t, y + h], [x + w, y + t], color);
            append_stroke(verts, inds, x, y + h - t, x + w, y + h, color);
        }
        '1' => {
            append_stroke(verts, inds, x + w * 0.5 - t * 0.5, y, x + w * 0.5 + t * 0.5, y + h, color);
            append_stroke(verts, inds, x + w * 0.1, y + t * 1.5, x + w * 0.5, y, color);
            append_stroke(verts, inds, x + w * 0.1, y + h - t, x + w * 0.9, y + h, color);
        }
        '3' => {
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_stroke(verts, inds, x, y + half_h, x + w, y + half_h + t, color);
            append_stroke(verts, inds, x, y + h - t, x + w, y + h, color);
            append_stroke(verts, inds, x + w - t, y, x + w, y + h, color);
        }
        '4' => {
            append_stroke(verts, inds, x, y, x + t, y + half_h, color);
            append_stroke(verts, inds, x, y + half_h, x + w, y + half_h + t, color);
            append_stroke(verts, inds, x + w - t, y, x + w, y + h, color);
        }
        '6' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_stroke(verts, inds, x, y + half_h, x + w, y + half_h + t, color);
            append_stroke(verts, inds, x, y + h - t, x + w, y + h, color);
            append_stroke(verts, inds, x + w - t, y + half_h, x + w, y + h, color);
        }
        '7' => {
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_polygon_4(verts, inds, [x + w, y], [x + w * 0.3, y + h], [x + w * 0.3 + t, y + h], [x + w, y + t], color);
        }
        '8' => {
            append_stroke(verts, inds, x, y, x + t, y + h, color);
            append_stroke(verts, inds, x + w - t, y, x + w, y + h, color);
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_stroke(verts, inds, x, y + half_h, x + w, y + half_h + t, color);
            append_stroke(verts, inds, x, y + h - t, x + w, y + h, color);
        }
        '9' => {
            append_stroke(verts, inds, x, y, x + t, y + half_h, color);
            append_stroke(verts, inds, x + w - t, y, x + w, y + h, color);
            append_stroke(verts, inds, x, y, x + w, y + t, color);
            append_stroke(verts, inds, x, y + half_h, x + w, y + half_h + t, color);
            append_stroke(verts, inds, x, y + h - t, x + w, y + h, color);
        }
        ':' => {
            append_rect(verts, inds, x + w * 0.35, y + h * 0.25, x + w * 0.35 + t, y + h * 0.25 + t, color);
            append_rect(verts, inds, x + w * 0.35, y + h * 0.65, x + w * 0.35 + t, y + h * 0.65 + t, color);
        }
        '/' => {
            append_polygon_4(verts, inds, [x + w, y], [x, y + h], [x + t, y + h], [x + w, y + t], color);
        }
        '-' => {
            append_stroke(verts, inds, x, y + half_h, x + w, y + half_h + t, color);
        }
        '.' => {
            append_rect(verts, inds, x + w * 0.3, y + h - t, x + w * 0.3 + t, y + h, color);
        }
        _ => {}
    }
}

pub fn get_string_width(text: &str, char_w: f32, spacing: f32) -> f32 {
    let mut w = 0.0;
    for ch in text.chars() {
        if ch == ' ' {
            w += char_w * 0.55;
        } else {
            w += char_w + spacing;
        }
    }
    w
}

pub fn print_string(
    verts: &mut Vec<Vertex3D>,
    inds: &mut Vec<u32>,
    text: &str,
    mut x: f32,
    y: f32,
    char_w: f32,
    char_h: f32,
    spacing: f32,
    color: [f32; 3],
    drop_shadow: bool,
) {
    if drop_shadow {
        let shadow_x = x + 2.0;
        let shadow_y = y + 2.0;
        let mut cur_sx = shadow_x;
        for ch in text.chars() {
            if ch == ' ' {
                cur_sx += char_w * 0.55;
                continue;
            }
            append_char(verts, inds, ch, cur_sx, shadow_y, char_w, char_h, [0.0, 0.0, 0.0]);
            cur_sx += char_w + spacing;
        }
    }

    for ch in text.chars() {
        if ch == ' ' {
            x += char_w * 0.55;
            continue;
        }
        append_char(verts, inds, ch, x, y, char_w, char_h, color);
        x += char_w + spacing;
    }
}

pub fn build_menu_overlay(
    menu: &CMenuManager,
    fps_text: &str,
) -> (Vec<Vertex3D>, Vec<u32>) {
    let mut verts = Vec::new();
    let mut inds = Vec::new();

    // =========================================================================
    // 1. DrawBackground (Frontend.cpp:2300)
    // 4 dark border polygons framing the menu window, leaving center open for 3D
    // =========================================================================
    let bg_color = [0.02, 0.015, 0.03]; // Dark border
    let top_left = [75.0, 32.0];
    let top_right = [585.0, 48.0];
    let bottom_left = [45.0, 420.0];
    let bottom_right = [605.0, 395.0];

    // Left border polygon
    append_polygon_4(&mut verts, &mut inds, [0.0, 0.0], top_left, bottom_left, [0.0, SCREEN_HEIGHT], bg_color);
    // Top border polygon
    append_polygon_4(&mut verts, &mut inds, [0.0, 0.0], [SCREEN_WIDTH, 0.0], top_right, top_left, bg_color);
    // Right border polygon
    append_polygon_4(&mut verts, &mut inds, top_right, [SCREEN_WIDTH, 0.0], [SCREEN_WIDTH, SCREEN_HEIGHT], bottom_right, bg_color);
    // Bottom border polygon
    append_polygon_4(&mut verts, &mut inds, bottom_left, bottom_right, [SCREEN_WIDTH, SCREEN_HEIGHT], [0.0, SCREEN_HEIGHT], bg_color);

    // Subtle 30% dark tint over the central viewing trapezoid so 3D scene is visible
    let tint_color = [0.05, 0.03, 0.08];
    append_polygon_4(&mut verts, &mut inds, top_left, top_right, bottom_right, bottom_left, tint_color);

    // =========================================================================
    // 2. DrawStandardMenus - Page Header (Frontend.cpp:983)
    // Printed at top-right: SCREEN_STRETCH_FROM_RIGHT(MENUHEADER_POS_X) = 630.0
    // Shadow offset (-7, +7) in (30, 30, 30), Foreground in LABEL_COLOR
    // =========================================================================
    let header_text = match menu.current_page {
        MenuPage::StartMenu => "MAIN MENU",
        MenuPage::NewGame => "START GAME",
        MenuPage::Options => "OPTIONS",
        MenuPage::DisplaySettings => "DISPLAY SETUP",
        MenuPage::SoundSettings => "AUDIO SETUP",
        MenuPage::Stats => "STATS",
    };

    let header_char_w = 14.0;
    let header_char_h = 24.0;
    let header_spacing = 3.0;
    let header_str_w = get_string_width(header_text, header_char_w, header_spacing);
    let header_right_x = 630.0;
    let header_x = header_right_x - header_str_w;
    let header_y = 12.0;

    // Header Shadow offset (-7.0, +7.0)
    print_string(
        &mut verts,
        &mut inds,
        header_text,
        header_x - 7.0,
        header_y + 7.0,
        header_char_w,
        header_char_h,
        header_spacing,
        HEADER_SHADOW_COLOR,
        false,
    );
    // Header Foreground (LABEL_COLOR = (255, 150, 225))
    print_string(
        &mut verts,
        &mut inds,
        header_text,
        header_x,
        header_y,
        header_char_w,
        header_char_h,
        header_spacing,
        LABEL_COLOR,
        false,
    );

    // Subtitle under header
    let sub_text = "NEON ENGINE // REVC VULKAN";
    let sub_w = get_string_width(sub_text, 6.5, 1.8);
    print_string(
        &mut verts,
        &mut inds,
        sub_text,
        header_right_x - sub_w,
        header_y + 28.0,
        6.5,
        11.0,
        1.8,
        [0.0, 0.95, 0.95], // Cyan drop-accent
        true,
    );

    // =========================================================================
    // 3. Menu Options & Selection Highlight (Frontend.cpp:1439 - menuOptionHighlight)
    // =========================================================================
    let char_w = 11.5;
    let char_h = 20.0;
    let spacing = 2.5;

    for (idx, item) in menu.items.iter().enumerate() {
        let is_selected = idx == menu.selected_option;
        let str_w = get_string_width(&item.label, char_w, spacing);

        let item_x = if item.is_centered {
            item.x - str_w * 0.5
        } else {
            item.x
        };
        let item_y = item.y;

        // If selected: Draw authentic SELECTIONBORDER_COLOR (25, 130, 70) highlight box
        if is_selected {
            let pad_x = 10.0;
            let pad_y = 4.0;
            let hl_left = item_x - pad_x;
            let hl_right = item_x + str_w + pad_x;
            let hl_top = item_y - pad_y;
            let hl_bottom = item_y + char_h + pad_y;

            // Draw selection box in SELECTIONBORDER_COLOR (Dark Vintage Green)
            append_polygon_4(
                &mut verts,
                &mut inds,
                [hl_left - 4.0, hl_top],
                [hl_right + 4.0, hl_top],
                [hl_right + 4.0, hl_bottom],
                [hl_left - 4.0, hl_bottom],
                SELECTIONBORDER_COLOR,
            );
        }

        // Option text: LABEL_COLOR for selected, DARKMENUOPTION_COLOR for unselected
        let text_color = if is_selected {
            LABEL_COLOR
        } else {
            DARKMENUOPTION_COLOR
        };

        print_string(
            &mut verts,
            &mut inds,
            &item.label,
            item_x,
            item_y,
            char_w,
            char_h,
            spacing,
            text_color,
            true, // with drop shadow
        );
    }

    // =========================================================================
    // 4. Sliders in Audio / Display Settings (Frontend.cpp:1560)
    // =========================================================================
    if menu.current_page == MenuPage::SoundSettings {
        let draw_slider = |verts: &mut Vec<Vertex3D>, inds: &mut Vec<u32>, y: f32, level: f32| {
            let start_x = 360.0;
            let bar_w = 8.0;
            let gap = 5.0;
            let num_bars = 16;
            let active_count = (level * num_bars as f32).round() as usize;

            for i in 0..num_bars {
                let bx = start_x + (i as f32) * (bar_w + gap);
                let bar_h = 6.0 + (i as f32) * 1.0;
                let by = y + 16.0 - bar_h;
                let color = if i < active_count { SLIDERON_COLOR } else { SLIDEROFF_COLOR };
                append_rect(verts, inds, bx, by, bx + bar_w, y + 16.0, color);
            }
        };

        draw_slider(&mut verts, &mut inds, 120.0, menu.music_volume);
        draw_slider(&mut verts, &mut inds, 155.0, menu.sfx_volume);
    }

    // =========================================================================
    // 5. Stats Screen (Frontend.cpp:973)
    // =========================================================================
    if menu.current_page == MenuPage::Stats {
        let stat_y = 100.0;
        let step = 26.0;

        print_string(&mut verts, &mut inds, "DISTANCE TRAVELED: 1337.50 KM", 80.0, stat_y, 9.5, 17.0, 2.0, LABEL_COLOR, true);
        print_string(&mut verts, &mut inds, "HIGHEST RECORDED FPS: 8080 FPS", 80.0, stat_y + step, 9.5, 17.0, 2.0, LABEL_COLOR, true);
        print_string(&mut verts, &mut inds, "VULKAN GRAPHICS PIPELINE: ACTIVE", 80.0, stat_y + step * 2.0, 9.5, 17.0, 2.0, LABEL_COLOR, true);
        print_string(&mut verts, &mut inds, "HARDWARE DEPTH BUFFER: D24 / D32", 80.0, stat_y + step * 3.0, 9.5, 17.0, 2.0, LABEL_COLOR, true);
    }

    // =========================================================================
    // 6. Footer bar with FPS and controls hint
    // =========================================================================
    print_string(
        &mut verts,
        &mut inds,
        "NAVIGATE: UP/DOWN OR TOUCH // ENTER: SELECT // ESC: BACK",
        45.0,
        SCREEN_HEIGHT - 22.0,
        6.5,
        11.0,
        1.5,
        [0.6, 0.6, 0.7],
        true,
    );

    let fps_disp = format!("{} FPS", fps_text.trim_start_matches("FPS: ").trim());
    print_string(
        &mut verts,
        &mut inds,
        &fps_disp,
        SCREEN_WIDTH - 120.0,
        SCREEN_HEIGHT - 22.0,
        7.0,
        12.0,
        1.5,
        [0.1, 1.0, 0.35],
        true,
    );

    (verts, inds)
}
