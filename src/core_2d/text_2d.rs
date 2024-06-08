use crate::math::color::Color;

use super::{drawable_2d::Drawable2D, drawable_state_2d::DrawableState2D};

struct Text2D {
    state: DrawableState2D,
    text: String,
    color: Color,
    scale: f32,
}

impl Text2D {
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_string();
        self.set_dirty(true);
    }

    pub fn set_color(&mut self, color: &Color) {
        self.color = color.clone();
        self.set_dirty(true);
    }

    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
        self.set_dirty(true);
    }
}

impl Drawable2D for Text2D {
    fn get_state(&mut self) -> &mut super::drawable_state_2d::DrawableState2D {
        return &mut self.state;
    }
}