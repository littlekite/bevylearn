use bevy::prelude::*;
use bevy_inspector_egui::{quick::WorldInspectorPlugin};


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
    app.add_plugin(WorldInspectorPlugin);

    //RPG 颜色转化%255
    app.insert_resource(ClearColor(Color::rgb(1., 1., 0.87)));

    app.add_system(controlplayer);

    app.add_startup_system(step);

    app.run();
}

#[derive(Component)]
pub struct Player;

fn step(
    mut commands:Commands,
    asset_server: Res<AssetServer>,
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

    //生成选手 player

    let player_bundle = SpriteBundle {
        texture:asset_server.load("player.png"),
        transform: Transform {
            translation: Vec3::new(0., -110., 100.0),
            ..default()
        },
        ..default()
    };

    commands.spawn(Player).insert(player_bundle);


}


fn  controlplayer(
    keyboard_input: Res<Input<KeyCode>>,
    mut transform_query: Query<
        &mut Transform,
        With<Player>,
    >,
){
    for mut transform in &mut transform_query {

        let mut x_position = transform.translation.x;
        // 根据速度时间计算新坐标
        if keyboard_input.pressed(KeyCode::A) {
            x_position -= 1.0 * 10.0 * 1.0;
        } else if keyboard_input.pressed(KeyCode::D) {
            x_position += 1.0 * 10.0 * 1.0;
        } else {
            return;
        } 
        transform.translation.x = x_position;
    }
}