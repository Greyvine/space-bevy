use bevy::math::Vec3;

pub const AU_TO_UNIT_SCALE: f32 = 149_597_870_691.0 * M_TO_UNIT_SCALE;
pub const KM_TO_UNIT_SCALE: f32 = 1_000.0 * M_TO_UNIT_SCALE;
pub const M_TO_UNIT_SCALE: f32 = 1.0 / 10_000_000.0;

pub fn convert_metres_to_units(input: Vec3) -> Vec3 {
    return input * M_TO_UNIT_SCALE;
}

pub fn print_scales() {
    println!("1m = {}", M_TO_UNIT_SCALE);
    println!("1km = {}", KM_TO_UNIT_SCALE);
    println!("1au = {}", AU_TO_UNIT_SCALE);
}
