use crate::framebuffer::Framebuffer;
use crate::maze::LEVEL_NAMES;
use crate::text;

pub fn draw_welcome_screen(fb: &mut Framebuffer, selected: usize) {
    fb.clear();

    let title = "MAZE RAYCASTER";
    let title_scale = 6;
    let title_w = text::text_width(title, title_scale);
    text::draw_text(
        fb,
        title,
        (fb.width.saturating_sub(title_w)) / 2,
        fb.height / 6,
        title_scale,
        0xffdd33,
    );

    let prompt = "ELIGE NIVEL CON FLECHAS";
    let prompt_scale = 3;
    let prompt_w = text::text_width(prompt, prompt_scale);
    text::draw_text(
        fb,
        prompt,
        (fb.width.saturating_sub(prompt_w)) / 2,
        fb.height / 6 + 90,
        prompt_scale,
        0xffffff,
    );

    let scale = 3;
    let spacing = 60;
    let start_y = fb.height / 6 + 150;
    for (i, name) in LEVEL_NAMES.iter().enumerate() {
        let label = format!("{} {}", i + 1, name);
        let color = if i == selected { 0x22ff22 } else { 0xaaaaaa };
        let w = text::text_width(&label, scale);
        text::draw_text(
            fb,
            &label,
            (fb.width.saturating_sub(w)) / 2,
            start_y + i * spacing,
            scale,
            color,
        );
    }

    let hint = "ENTER PARA COMENZAR - F PARA FPS";
    let hint_scale = 2;
    let hint_w = text::text_width(hint, hint_scale);
    text::draw_text(
        fb,
        hint,
        (fb.width.saturating_sub(hint_w)) / 2,
        fb.height - fb.height / 5,
        hint_scale,
        0xffffff,
    );
}
