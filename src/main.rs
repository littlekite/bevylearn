use bevy::prelude::*;
use bevy_inspector_egui::{quick::WorldInspectorPlugin};


fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "绝地求生".to_string(), // ToDo
            canvas: Some("#bevy".to_owned()),
            ..Default::default()
        },
        ..default()
    }));
    app.add_plugin(WorldInspectorPlugin);

    //RPG 颜色转化%255
    app.insert_resource(ClearColor(Color::rgb(1., 1., 0.87)));

    app.add_system(controlplayer);
    app.add_system(players_attack);
    app.add_system(move_bullet);

    app.add_startup_system(step);

    app.register_type::<Stats>();

    app.run();
}

#[derive(Component)]
pub struct Player;

/// Bundle added to a fighter stub, in order to activate it.
#[derive(Bundle)]
pub struct PlayerBundle {
    pub stats: Stats
}



#[derive(Component, Clone, Debug, Reflect)]
pub struct Stats {
    pub max_health: i32,
    pub movement_speed: f32,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            max_health: 100,
            movement_speed: 17000.,
        }
    }
}

// 坦克刷新子弹间隔
pub const TANK_REFRESH_BULLET_INTERVAL: f32 = 2.0;

// 坦克刷新子弹计时器
#[derive(Component, Deref, DerefMut)]
pub struct TankRefreshBulletTimer(pub Timer);





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

    let player_info_bundle = PlayerBundle {
        stats: Stats { max_health: (100), movement_speed: (1500.) }
    };
    commands.spawn(Player).insert(player_bundle).insert(TankRefreshBulletTimer(Timer::from_seconds(
        TANK_REFRESH_BULLET_INTERVAL,
        TimerMode::Once,
    ))).insert(player_info_bundle);


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
            x_position -= 1.0 * 5.0 * 1.0;
        } else if keyboard_input.pressed(KeyCode::D) {
            x_position += 1.0 * 5.0 * 1.0;
        } else {
            return;
        } 
        transform.translation.x = x_position;
    }
}

#[derive(Component)]
pub struct Bullet;

pub fn spawn_bullet(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    translation: Vec3
) {
    let bullet_texture_handle = asset_server.load("bullet.png");
    let bullet_texture_atlas =
        TextureAtlas::from_grid(bullet_texture_handle, Vec2::new(7.0, 8.0), 4, 1, None, None);
    let bullet_texture_atlas_handle = texture_atlases.add(bullet_texture_atlas);

    commands
        .spawn(Bullet)
        .insert(SpriteSheetBundle {
            texture_atlas: bullet_texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(translation.x, translation.y, translation.z),
                ..default()
            },
            ..default()
        });
}

// 炮弹移动
// 撞墙消失
pub fn move_bullet(
    mut _commands: Commands,
    mut transform_query: Query<&mut Transform, With<Bullet>>,
) {
    let bullet_movement = 1.0 * 5. * 1.;
    for mut bullet_transform in &mut transform_query {
        
        bullet_transform.translation.y += bullet_movement
        
    }
}

// 玩家攻击
fn players_attack(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<
    (&Transform, &mut TankRefreshBulletTimer),
        With<Player>,
    >,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (transform, mut refresh_bullet_timer) in &mut player {
        refresh_bullet_timer.tick(time.delta());
        if keyboard_input.just_pressed(KeyCode::W) {
            if refresh_bullet_timer.finished() {
                // TODO startup时加载texture
                spawn_bullet(
                    &mut commands,
                    &asset_server,
                    &mut texture_atlases,
                    transform.translation,
                );
                refresh_bullet_timer.reset();
            }
        }
    }
}