use macroquad::prelude::*;

pub fn texts_horizontal(
    texts: &[String],
    mut startx: f32,
    y: f32,
    space: f32,
    font: u16,
    color: Color,
) -> f32 {
    for text in texts {
        let center = get_text_center(text, None, font, 1.0, 0.0);

        draw_text(text, startx, y, f32::from(font), color);

        startx += center.x * 2.0 + space;
    }
    startx
}
