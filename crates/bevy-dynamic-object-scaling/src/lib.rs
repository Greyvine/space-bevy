pub mod events;
pub mod tags;

use bevy::prelude::*;
use events::ScalingTranslationEvent;
use tags::ScalingObjectTag;

#[derive(Default)]
pub struct DynamicObjectScalingPlugin;

impl Plugin for DynamicObjectScalingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ScalingTranslationEvent>()
            .add_system(change_scale_with_distance.system());
    }
}

fn change_scale_with_distance(
    mut translations: EventReader<ScalingTranslationEvent>,
    mut query: Query<&mut Transform, With<ScalingObjectTag>>,
) {
    for translation in translations.iter() {
        for mut transform in query.iter_mut() {
            let distance = (translation).distance(transform.translation);
            if distance > 500.0 {
                let scaling_factor = get_scaling_factor(distance);
                transform.scale = Vec3::new(scaling_factor, scaling_factor, scaling_factor);
            }
        }
    }
}

fn get_scaling_factor(distance: f32) -> f32 {
    500.0 / distance
}
