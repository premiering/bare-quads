use std::sync::atomic::AtomicU64;

use bitflags::bitflags;
use cgmath::{Vector4, Vector2};

use crate::math::color::Color;

use super::drawable_2d::Drawable2D;

static DRAWABLE_STATE_COUNTER: AtomicU64 = AtomicU64::new(0);

// For some reason the macro doesn't allow me to put the bits inside its own implenentation.
pub struct AnchorBit {}

impl AnchorBit {
    pub const Y0: u32 = 1;
    pub const Y1: u32 = 1 << 1;
    pub const Y2: u32 = 1 << 2;
    pub const X0: u32 = 1 << 3;
    pub const X1: u32 = 1 << 4;
    pub const X2: u32 = 1 << 5;   
} 

bitflags! {
    #[derive(Clone, Copy)]
    pub struct Anchor: u32 {
        const Y0 = 1;
        const Y1 = 1 << 1;
        const Y2 = 1 << 2;
        const X0 = 1 << 3;
        const X1 = 1 << 4;
        const X2 = 1 << 5;   
        const TOP_LEFT = AnchorBit::Y0 |AnchorBit::X0;
        const TOP_CENTRE = AnchorBit::Y0 |AnchorBit::X1;
        const TOP_RIGHT = AnchorBit::Y0 |AnchorBit::X2;
        const CENTRE_LEFT = AnchorBit::Y1 |AnchorBit::X0;
        const CENTRE = AnchorBit::Y1 | AnchorBit::X1;
        const CENTRE_RIGHT = AnchorBit::Y1 | AnchorBit::X2;
        const BOTTOM_LEFT = AnchorBit::Y2 | AnchorBit::X0;
        const BOTTOM_CENTRE = AnchorBit::Y2 | AnchorBit::X1;
        const BOTTOM_RIGHT = AnchorBit::Y2 | AnchorBit::X2;
    }
}

/*#[derive(Clone, Copy)]
pub struct Anchor;

impl Anchor {
    static TOP_LEFT: AnchorBit = AnchorBit::Y0 |AnchorBit::X0;
    static TOP_CENTRE: AnchorBit = AnchorBit::Y0 |AnchorBit::X1;
    static TOP_RIGHT: AnchorBit = AnchorBit::Y0 |AnchorBit::X2;
    static CENTRE_LEFT: AnchorBit = AnchorBit::Y1 |AnchorBit::X0;
    static CENTRE: AnchorBit = AnchorBit::Y1 |AnchorBit::X1;
    static CENTRE_RIGHT: AnchorBit = AnchorBit::Y1 |AnchorBit::X2;
    static BOTTOM_LEFT: AnchorBit = AnchorBit::Y2 |AnchorBit::X0;
    static BOTTOM_CENTRE: AnchorBit = AnchorBit::Y2 |AnchorBit::X1;
    static BOTTOM_RIGHT: AnchorBit = AnchorBit::Y2 |AnchorBit::X2;
}*/

#[derive(Copy, Clone, PartialEq, Hash)]
pub struct DrawableId(u64);

impl DrawableId {
    pub fn dummy() -> DrawableId {
        return DrawableId(0);
    }
}

pub enum DrawNode2D {
    Quad(QuadDrawNode2D),
    Text(TextDrawNode2D),
}

pub struct QuadDrawNode2D {
    pub quad: TransformState2D,
    pub color: Vector4<f32>
}

pub struct TextDrawNode2D {
    pub transform: TransformState2D,
    pub text: String,
    pub scale: f32,
    pub color: Color,
    pub alignment: Anchor,
}

#[derive(Copy, Clone)]
// This represents a quad with relative and absolute sizing and positioning.
pub struct TransformState2D {
    pub abs_pos: Vector2<f32>,
    pub rel_pos: Vector2<f32>,
    pub abs_size: Vector2<f32>,
    pub rel_size: Vector2<f32>,
    pub origin: Anchor,
    pub alignment: Anchor,
}

impl TransformState2D {
    pub fn new() -> TransformState2D {
        return TransformState2D {
            abs_pos: Vector2::new(0.0, 0.0),
            rel_pos: Vector2::new(0.0, 0.0),
            abs_size: Vector2::new(0.0, 0.0),
            rel_size: Vector2::new(0.0, 0.0),
            origin: Anchor::TOP_LEFT,
            alignment: Anchor::TOP_LEFT,
        }
    }
}

pub struct DrawableState2D {
    id: DrawableId,
    pub drawable_transform: TransformState2D,
    pub children: Vec<Box<dyn Drawable2D>>,
    dirty: bool,// Whether the buffers for the drawable need to be recalculated
}

impl DrawableState2D {
    pub fn new() -> Self {
        return DrawableState2D {
            id: DrawableId(DRAWABLE_STATE_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)),
            drawable_transform: TransformState2D::new(),
            children: vec![],
            dirty: true,
        };
    }

    pub fn get_id(&mut self) -> DrawableId {
        return self.id.clone();
    }

    pub fn is_dirty(&mut self) -> bool {
        return self.dirty;
    }

    pub fn set_dirty(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    pub fn set_abs_pos(&mut self, x: f32, y: f32) {
        self.drawable_transform.abs_pos = Vector2::new(x, y);
        self.set_dirty(true);
    }

    pub fn set_rel_pos(&mut self, x: f32, y: f32) {
        self.drawable_transform.rel_pos = Vector2::new(x, y);
        self.set_dirty(true);
    }

    pub fn set_abs_size(&mut self, x: f32, y: f32) {
        self.drawable_transform.abs_size = Vector2::new(x, y);
        self.set_dirty(true);
    }

    pub fn set_rel_size(&mut self, x: f32, y: f32) {
        self.drawable_transform.rel_size = Vector2::new(x, y);
        self.set_dirty(true);
    }

    pub fn set_origin(&mut self, origin: Anchor) {
        self.drawable_transform.origin = origin;
        self.set_dirty(true);
    }

    pub fn set_alignment(&mut self, alignment: Anchor) {
        self.drawable_transform.alignment = alignment;
        self.set_dirty(true);
    }

    pub fn get_transform_state(&mut self) -> &TransformState2D {
        return &mut self.drawable_transform;
    }

    pub fn add_child(&mut self, drawable: Box<dyn Drawable2D>) {
        self.children.push(drawable);
        self.set_dirty(true);
    }

    pub fn get_children(&mut self) -> &mut Vec<Box<dyn Drawable2D>> {
        return &mut self.children;
    }
}