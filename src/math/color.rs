use cgmath::Vector4;

#[derive(Copy, Clone, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        return Color {
            r,
            g,
            b,
            a,
        }
    }

    pub fn set_r(&mut self, r: u8) {
        self.r = r;
    }

    pub fn set_g(&mut self, g: u8) {
        self.g = g;
    }

    pub fn set_b(&mut self, b: u8) {
        self.b = b;
    }

    pub fn set_a(&mut self, a: u8) {
        self.a = a;
    }

    pub fn to_vec4(&mut self) -> Vector4<f32> {
        return Vector4::new(self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, self.a as f32 / 255.0);
    }

    pub fn to_tuple(&mut self) -> (f32, f32, f32, f32) {
        return (self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, self.a as f32 / 255.0);
    }

    pub fn to_array(&mut self) -> [f32; 4] {
        return [self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, self.a as f32 / 255.0];
    }
}