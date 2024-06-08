use crate::math::color::Color;

use super::{drawable_2d::Drawable2D, drawable_state_2d::{DrawableState2D, DrawNode2D, QuadDrawNode2D}};

pub struct Box2D {
    state: DrawableState2D,
    color: Color,
}

impl Box2D {
    pub fn new() -> Box2D {
        return Box2D {
            state: DrawableState2D::new(),
            color: Color::new(255, 255, 255, 255),
        }
    }

    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self.state.set_dirty(true);
        return self;
    }
}

impl Drawable2D for Box2D {
    fn get_state(&mut self) -> &mut super::drawable_state_2d::DrawableState2D {
        return &mut self.state;
    }

    fn draw(&mut self) -> Option<Vec<super::drawable_state_2d::DrawNode2D>> {
        let node = DrawNode2D::Quad(QuadDrawNode2D {
            quad: self.state.get_transform_state().clone(),
            color: self.color.to_vec4(),
        });
        return Some(vec![node]);
    }
}