use bevy_hanabi::*;

fn setup(mut effects: ResMut<Assets<EffectAsset>>) {
    // Define a color gradient
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(1., 0., 0., 1.)); // Red
    gradient.add_key(1.0, Vec4::ZERO); // Transparent black

    // Create the effect asset
    let effect = effects.add(
        EffectAsset {
            name: "MyEffect".to_string(),
            // Maximum number of particles alive at a time
            capacity: 32768,
            // Spawn at a rate of 5 particles per second
            spawner: Spawner::rate(5.0.into()),
            ..Default::default()
        }
        // On spawn, randomly initialize the position and velocity
        // of the particle over a sphere of radius 2 units, with a
        // radial initial velocity of 6 units/sec away from the
        // sphere center.
        .init(PositionSphereModifier {
            center: Vec3::ZERO,
            radius: 2.,
            dimension: ShapeDimension::Surface,
            speed: 6.0.into(),
        })
        // Every frame, add a gravity-like acceleration downward
        .update(AccelModifier {
            accel: Vec3::new(0., -3., 0.),
        })
        // Render the particles with a color gradient over their
        // lifetime.
        .render(ColorOverLifetimeModifier { gradient }),
    );
}
