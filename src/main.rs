use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,
		      RapierPhysicsPlugin::<NoUserData>::default(),
		      RapierDebugRenderPlugin::default()
	))
        .add_systems(Startup, (setup_graphics, setup_physics))
        .add_systems(Update, (button_pressed, touch_pressed, test))
        .run();
}

fn drop_ball(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>) {

    let radius = 0.5;
    let mut rng = rand::thread_rng();

    let r = rng.gen_range(0.0..1.0);
    let g = rng.gen_range(0.0..1.0);
    let b = rng.gen_range(0.0..1.0);
    
    let pbr_bundle = PbrBundle {
        mesh: meshes.add(shape::Icosphere {
	    radius: radius,
	    subdivisions: 4
	}.try_into().unwrap()),
        material: materials.add(Color::rgb(r, g, b).into()),
	..Default::default()
    };

    commands
	.spawn(RigidBody::Dynamic)
	.insert(pbr_bundle)
	// .insert(Velocity::default())
	.insert(ExternalImpulse {
            impulse: Vec3::new(0.0, 2.0, 0.0),
            torque_impulse: Vec3::new(0.1, 0.2, 0.3),
	})
	// .insert(ExternalForce {
        //     force: Vec3::new(1.0, 2.0, 3.0),
        //     torque: Vec3::new(1.0, 2.0, 3.0),
	// })
	.insert(Collider::ball(radius))
	.insert(Restitution::coefficient(0.7))
	.insert(TransformBundle::from(
	    Transform::from_xyz(rng.gen_range(-2.0..2.0),
				5.0,
				rng.gen_range(-2.0..2.0))));
}

fn touch_pressed(mut commands: Commands, touches: Res<Touches>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {

    for finger in touches.iter() {
        if touches.just_pressed(finger.id()) {
	    drop_ball(&mut commands, &mut meshes, &mut materials);
            println!("A new touch with ID {} just began.", finger.id());
        }
        println!(
            "Finger {} is at position ({},{}), started from ({},{}).",
            finger.id(),
            finger.position().x,
            finger.position().y,
            finger.start_position().x,
            finger.start_position().y,
        );
    }
}

fn button_pressed(mut commands: Commands, mouse_button: Res<Input<MouseButton>>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {

    if mouse_button.pressed(MouseButton::Left) {
	drop_ball(&mut commands, &mut meshes, &mut materials);
    }
}
fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn test(mut query: Query<(&mut RigidBody, &mut Velocity)>) {
    // for (_body, mut v) in query.iter_mut() {
    // 	println!("{:?}", v);
    // 	v.linvel.x += 0.1;
    // }
}

fn setup_physics(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {

    let x_len = 100.0;
    let y_len = 1.0;
    let z_len = 100.0;
    let bbox = shape::Box {
	min_x: -x_len / 2.0,
	max_x: x_len / 2.0,
	min_y: -y_len / 2.0,
	max_y: y_len / 2.0,
	min_z: -z_len / 2.0,
	max_z: z_len / 2.0,
    };

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(bbox.into()),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
	    ..Default::default()
	})
        .insert(Collider::cuboid(x_len, y_len, z_len))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            ..Default::default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)),
        ..Default::default()
    });
}
