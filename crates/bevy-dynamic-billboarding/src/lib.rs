pub mod texture_node;

pub use texture_node::TextureNode;

use bevy::prelude::*;

#[derive(Default)]
pub struct DynamicBillboardingPlugin;

impl Plugin for DynamicBillboardingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(create_billboard_with_distance.system());
    }
}

fn create_billboard_with_distance() {

}
