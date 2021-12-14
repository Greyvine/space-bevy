pub const G: f32 = 6.67430e-11_f32;

pub struct Gravity(pub f32);

impl Default for Gravity {
    fn default() -> Self {
        Self(G)
    }
}
