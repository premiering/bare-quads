use std::{any::TypeId, sync::{Arc, atomic::{AtomicI64, AtomicU64}}, marker::PhantomData, rc::Rc, cell::RefCell};

use cgmath::{Vector2, Vector4};

use crate::math::color::Color;

use super::{drawable_state_2d::{DrawableState2D, DrawNode2D, DrawableId, TransformState2D, Anchor}, box_2d::Box2D};

pub struct SimpleDrawable2D {
    state: DrawableState2D,
}

impl Drawable2D for SimpleDrawable2D {
    fn get_state(&mut self) -> &mut DrawableState2D {
        return &mut self.state;
    }
}

impl SimpleDrawable2D {
    pub fn new() -> SimpleDrawable2D {
        let mut d = SimpleDrawable2D {
            state: DrawableState2D::new(),
        };
        let mut b2d = Box2D::new();
        b2d.set_color(Color::new(100, 100, 100, 50)).set_rel_size(1.0, 1.0);
        d.add_child(Box::new(b2d));
        return d;
    }
}

pub trait Drawable2D {
    fn get_state(&mut self) -> &mut DrawableState2D;

    fn get_id(&mut self) -> DrawableId {
        return self.get_state().get_id();
    } 

    fn update(&mut self) {
        for child in self.get_state().get_children() {
            child.update();
        }
    }

    // The goal is to get the root Container2D to the point where there are no more relative sizes/positions
    // In other words, all relative size/positions = 0.0
    fn draw(&mut self) -> Option<Vec<DrawNode2D>> {
        let mut nodes: Vec<DrawNode2D> = vec![];
        let state = self.get_state();
        let self_state = &state.drawable_transform;
        let children = &mut state.children;
        //println!("prayge");
        // Relative values go down the line if not found
        for child in children {
            let result = child.draw();
            if !result.is_some() {
                continue;
            }
            let mut new_nodes = result.expect("");
            //println!("sup");
            for mut node in &mut new_nodes {
                let mut transform: &mut TransformState2D;
                match node {
                    DrawNode2D::Quad(quad) => {
                        transform = &mut quad.quad;
                        // Apply the size information we know about ourselves and our child to the child.
                        transform.abs_size.x += transform.rel_size.x * self_state.abs_size.x;
                        transform.abs_size.y += transform.rel_size.y * self_state.abs_size.y;
                        transform.rel_size = self_state.rel_size;
                    }
                    DrawNode2D::Text(text) => {
                        transform = &mut text.transform;
                    }
                }

                process_anchors(&mut transform);
                // Apply the position information we know about ourselves and our child to the child.
                transform.abs_pos.x += transform.rel_pos.x * self_state.abs_size.x;
                transform.abs_pos.y += transform.rel_pos.y * self_state.abs_size.y;
                transform.rel_pos = self_state.rel_pos;
                transform.abs_pos += self_state.abs_pos;
            }
            nodes.append(&mut new_nodes);
        }
        return Some(nodes);
    }

    fn is_dirty(&mut self) -> bool {
        return self.get_state().is_dirty();
    }

    fn set_dirty(&mut self, dirty: bool) {
        self.get_state().set_dirty(dirty);
    }

    fn set_abs_pos(&mut self, x: f32, y: f32) {
        self.get_state().set_abs_pos(x, y);
    }
    
    fn set_rel_pos(&mut self, x: f32, y: f32) {
        self.get_state().set_rel_pos(x, y);
    }

    fn set_abs_size(&mut self, x: f32, y: f32) {
        self.get_state().set_abs_size(x, y);
    }

    fn set_rel_size(&mut self, x: f32, y: f32) {
        self.get_state().set_rel_size(x, y);
    }

    fn set_origin(&mut self, origin: Anchor) {
        self.get_state().set_origin(origin);
    }

    fn set_alignment(&mut self, alignment: Anchor) {
        self.get_state().set_alignment(alignment);
    }

    fn get_transform_state(&mut self) -> &TransformState2D {
        return self.get_state().get_transform_state();
    }

    fn add_child(&mut self, child: Box<dyn Drawable2D>) {
        self.get_state().add_child(child);
    }

    fn get_children(&mut self) -> &mut Vec<Box<dyn Drawable2D>> {
        return self.get_state().get_children();
    }
}

fn process_anchors(transform: &mut TransformState2D) {
    if transform.origin.intersects(Anchor::X1) {
        transform.rel_pos.x += 0.5;
    } else if transform.origin.intersects(Anchor::X2) {
        transform.rel_pos.x += 1.0;
    }

    if transform.origin.intersects(Anchor::Y1) {
        transform.rel_pos.y += 0.5;
    } else if transform.origin.intersects(Anchor::Y2) {
        transform.rel_pos.y += 1.0;
    }

    if transform.alignment.intersects(Anchor::X1) {
        transform.abs_pos.x -= transform.abs_size.x / 2.0;
        transform.rel_pos.x -= transform.rel_size.x / 2.0;
    } else if transform.alignment.intersects(Anchor::X2) {
        transform.abs_pos.x -= transform.abs_size.x;
        transform.rel_pos.x -= transform.rel_size.x;
    }

    if transform.alignment.intersects(Anchor::Y1) {
        transform.abs_pos.y -= transform.abs_size.y / 2.0;
        transform.rel_pos.y -= transform.rel_size.y / 2.0;
    } else if transform.alignment.intersects(Anchor::Y2) {
        transform.abs_pos.y -= transform.abs_size.y;
        transform.rel_pos.y -= transform.rel_size.y;
    }
}