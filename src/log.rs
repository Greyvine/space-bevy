use crate::cameras::event::*;
use crate::controllers::event::*;
use bevy::prelude::*;

pub fn print_input_events(
    mut force_events: EventReader<ForceEvent>,
    mut yaw_events: EventReader<YawEvent>,
    mut pitch_events: EventReader<PitchEvent>,
) {
    for event in force_events.iter() {
        println!("{:?}", event);
    }
    for event in yaw_events.iter() {
        println!("{:?}", event);
    }
    for event in pitch_events.iter() {
        println!("{:?}", event);
    }
}
