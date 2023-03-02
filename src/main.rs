use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Bevy game".to_string(), // ToDo
            canvas: Some("#bevy".to_owned()),
            ..Default::default()
        },
        ..default()
    }));

    //RPG 颜色转化%255
    app.insert_resource(ClearColor(Color::rgb(1., 1., 0.87)));

    app.add_startup_system(step);
    app.run();
}

fn step(
    mut commands:Commands
){
    //生产照相机
    // Spawn the camera
    let camera_bundle =  Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1000.),
        projection: OrthographicProjection {
            scale: 0.5,
            ..default()
        },
        ..default()
    };
    // camera_bundle.orthographic_projection.depth_calculation = DepthCalculation::Distance;;
    commands.spawn(camera_bundle);


}