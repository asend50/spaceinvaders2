use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;

use macroquad::prelude::*;

pub async fn run(tm: TextureManager, score: i32,) -> (String, TextureManager, i32 ){
    
    let menu= StillImage::from_preload(
    tm.get_preload("assets/menu.png").unwrap(),
    screen_width(),  // Width
    screen_height(), // Height
    0.0,                   // X position
    0.0,                   // Y position
    true,                  // Enable stretching
    1.0,                   // Zoom level (100%)
);

let mut lbl_score = Label::new(format!("Score:\n{}", score), 320.0, 525.0, 60);
lbl_score.with_colors(GREEN, Some(BLACK));

    loop {
        clear_background(WHITE);
        menu.draw();
        lbl_score.draw();

        if is_key_pressed(KeyCode::Space) {
            return ("game".to_string(), tm, score);
        }

        next_frame().await;
    }
}