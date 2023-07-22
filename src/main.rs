use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,
		      RapierPhysicsPlugin::<NoUserData>::default(),
		      RapierDebugRenderPlugin::default()))
        .add_systems(Startup, (setup_graphics, setup_physics))
    // .add_system(print_ball_altitude)
        .add_systems(Update, (button_pressed, touch_pressed)) 
        .run();
}

fn drop_ball(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    commands
	.spawn(RigidBody::Dynamic)
	.insert(Collider::ball(0.5))
	.insert(Restitution::coefficient(0.7))
	.insert(TransformBundle::from(
	    Transform::from_xyz(rng.gen_range(-2.0..2.0),
				5.0,
				rng.gen_range(-2.0..2.0))));
}

fn touch_pressed(mut commands: Commands, touches: Res<Touches>) {

    for finger in touches.iter() {
        if touches.just_pressed(finger.id()) {
	    drop_ball(&mut commands);
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

fn button_pressed(mut commands: Commands, mouse_button: Res<Input<MouseButton>>) {

    if mouse_button.pressed(MouseButton::Left) {
	drop_ball(&mut commands);
    }
}
fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(0.5))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
}
