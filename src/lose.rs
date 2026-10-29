use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;

use macroquad::prelude::*;

pub async fn run(tm: TextureManager, mut score: i32,) -> (String, TextureManager, i32 ){

    let lose= StillImage::from_preload(
    tm.get_preload("assets/lose.png").unwrap(),
    screen_width(),  // Width
    screen_height(), // Height
    0.0,                   // X position
    0.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

let mut lbl_score = Label::new(format!("Score:\n{}", score), 320.0, 75.0, 60);
lbl_score.with_colors(RED, Some(BLACK));

    loop {
        clear_background(WHITE);
        lose.draw();
        lbl_score.draw();

        if is_key_pressed(KeyCode::Space) {
            score = 0;
            return ("menu".to_string(), tm, score);
        }

        next_frame().await;
    }
}