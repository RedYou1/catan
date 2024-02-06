use macroquad::prelude::*;

pub fn texts_vertical(
    texts: &[String],
    centerx: f32,
    mut starty: f32,
    space: f32,
    font: u16,
    color: Color,
) -> f32 {
    for text in texts {
        let center = get_text_center(text, None, font, 1.0, 0.0);

        starty -= center.y * 2.0;

        draw_text(text, centerx - center.x, starty, f32::from(font), color);

        starty += space;
    }
    starty
}
