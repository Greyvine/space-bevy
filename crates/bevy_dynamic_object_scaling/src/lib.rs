pub mod tags;
pub mod events;

use bevy::prelude::*;
use events::ScalingCameraEvent;
use tags::{ScalingObjectTag, ScalingCameraTag};

#[derive(Default)]
pub struct DynamicObjectScalingPlugin;

impl Plugin for DynamicObjectScalingPlugin {

    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<ScalingCameraEvent>()
            .add_system(change_scale_with_distance.system());
    }

}

// fn change_scale_with_distance(
//     mut objects_query: Query<&mut Transform, With<ScalingObjectTag>>,
//     // camera_query: Query<&Transform, With<ScalingCameraTag>>,
// ) {
//     // let camera_transform = camera_query.single().expect("There should only ever be one player in the game!");
//     let camera_transform = Transform::identity();

//     for mut object_transform in objects_query.iter_mut() {
        
//         let distance = (camera_transform.translation).distance(object_transform.translation);

//         let scaling_factor = get_scaling_factor(distance);

//         object_transform.scale = Vec3::new(scaling_factor, scaling_factor, scaling_factor);

//         println!("Distance: {}, Scaling Factor: {}", distance, scaling_factor)

//     }



// }

fn change_scale_with_distance(
    mut translations: EventReader<ScalingCameraEvent>,
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
    // if (distance > 900.0) {
         
    // }
    500.0 / distance
}