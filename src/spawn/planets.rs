use crate::gravity::{body::*, resources::Gravity};

use crate::scale::*;
use bevy::prelude::*;
use bevy_dynamic_billboarding::tags::FirstPass;
use bevy_dynamic_object_scaling::tags::ScalingObjectTag;

pub fn spawn_planets(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut g: ResMut<Gravity>,
) {
    const DAY: f32 = 86_400.0;
    g.0 *= DAY * DAY * 10.0f32.powi(-6) / 1.5f32.powi(3);

    let sun = BodyBundle::new(1_988_500.0, Vec3::ZERO, Vec3::ZERO);
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 695_508.0 * KM_TO_UNIT_SCALE * 0.1,
                subdivisions: 10,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE.into(),
                roughness: 0.6,
                emissive: Color::WHITE,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(FirstPass)
        .insert(Name::new("sun"))
        .insert(ScalingObjectTag)
        .insert_bundle(sun)
        .insert(Light {
            color: Color::WHITE,
            intensity: 0.5 * AU_TO_UNIT_SCALE,
            range: 0.25 * AU_TO_UNIT_SCALE,
            ..Default::default()
        });

    macro_rules! spawn_planet {
        ($name:ident, m=$mass:literal, pos=($($pos:literal),+), vel=($($vel:literal),+), r=$radius:literal, col=$col:expr $(,)?) => {
            let $name = BodyBundle::new($mass, AU_TO_UNIT_SCALE * Vec3::new($($pos),+), AU_TO_UNIT_SCALE * Vec3::new($($vel),+));
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: $radius * KM_TO_UNIT_SCALE,
                        subdivisions: 5,
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: $col.into(),
                        roughness: 0.6,
                        reflectance: 0.1,
                        emissive: $col.into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .insert(ScalingObjectTag)
                .insert(Name::new(stringify!($name)))
                .insert_bundle($name);
        };
    }

    // Data pulled from JPL Horizons as of 2021-04-18
    // https://ssd.jpl.nasa.gov/horizons.cgi
    #[rustfmt::skip]
    spawn_planet!(
        mercury,
        m=0.3302,
        pos=(3.044170697902298E-01, 1.295114876282963E-01, -1.734104195212369E-02),
        vel=(-1.648628006573339E-02, 2.713585294570181E-02, 3.729745700066048E-03),
        r=2439.7,
        col=Color::GRAY,
    );

    #[rustfmt::skip]
    spawn_planet!(
        venus,
        m=4.868,
        pos=(5.387247476293335E-01, 4.820230339302334E-01, -2.447215630265642E-02),
        vel=(-1.354845714410186E-02, 1.498631588335955E-02, 9.874886299710420E-04),
        r=6051.84,
        col=Color::ORANGE,
    );

    #[rustfmt::skip]
    spawn_planet!(
        earth,
        m=5.97219,
        pos=(-8.873674344461769E-01, -4.697992257377307E-01, 2.381003809013169E-05),
        vel=(7.775921491692710E-03, -1.526923260035268E-02, 1.329236295796724E-07),
        r=6371.01,
        col=Color::TURQUOISE,
    );
    #[rustfmt::skip]
    spawn_planet!(
        mars,
        m=0.64171 ,
        pos=(-7.669365607923907E-01, 1.437715683938847E+00, 4.894216325150345E-02),
        vel=(-1.181841087219943E-02, -5.396860897762226E-03, 1.768153357356463E-04),
        r=3389.92,
        col=Color::RED,
    );
    #[rustfmt::skip]
    spawn_planet!(
        jupiter,
        m=1898.187,
        pos=(3.638338491378654E+00, -3.517196054099748E+00, -6.679350348303023E-02),
        vel=(5.159638546395391E-03, 5.787459942412818E-03, -1.394560955359292E-04),
        r=69911.0,
        col=Color::BISQUE,
    );
    #[rustfmt::skip]
    spawn_planet!(
        saturn,
        m=568.34,
        pos=(5.946821461107053E+00, -8.000786524501104E+00, -9.757186586148088E-02),
        vel=(4.173453543382942E-03, 3.320093983241896E-03, -2.235785645393874E-04),
        r=58232.0,
        col=Color::GOLD,
    );
    #[rustfmt::skip]
    spawn_planet!(
        uranus,
        m=86.813,
        pos=(1.507889019392361E+01, 1.276651492152234E+01, -1.479475386482554E-01),
        vel=(-2.565701401124483E-03, 2.824133197172000E-03, 4.363663945419187E-05),
        r=25362.0,
        col=Color::AQUAMARINE,
    );
    #[rustfmt::skip]
    spawn_planet!(
        neptune,
        m=102.4126,
        pos=(2.951580077181258E+01, -4.898113153026739E+00, -5.794227616270428E-01),
        vel=(4.988324362083494E-04, 3.122660147661985E-03, -7.542919141146281E-05),
        r=24622.0,
        col=Color::BLUE
    );
    #[rustfmt::skip]
    spawn_planet!(
        pluto,
        m=0.013030,
        pos=(1.437474170944128E+01, -3.109027718169479E+01, -8.297576366914019E-01),
        vel=(2.929346098298212E-03, 6.560315763737425E-04, -9.025427350060328E-04),
        r=11880.3,
        col=Color::GRAY,
    );
}
