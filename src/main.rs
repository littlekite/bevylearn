use bevy::prelude::*;
use bevy_inspector_egui::{quick::WorldInspectorPlugin};
use rand::prelude::*;
use bevy_rapier2d::prelude::*;

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
    /*
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window{
            resolution: WindowResolution::new(800.0,600.0),
            ..default()
        }),
        ..default()
    }));
     */
    app.add_plugin(WorldInspectorPlugin);

    //RPG 颜色转化%255
    app.insert_resource(ClearColor(Color::rgb(1., 1., 0.87)));

    app.add_system(controlplayer);
    app.add_system(players_attack);
    app.add_system(move_bullet);
    app.add_system(move_bullet_enemy);
    app.add_system(update_uiboard);//更新UI

    app.add_system(swap_suiji_bullet);//更新UI

    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    app.add_plugin(RapierDebugRenderPlugin::default());


    app.add_system(check_collide);

    app.add_startup_system(step);

    app.register_type::<Stats>();

    app.run();
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

/// Bundle added to a fighter stub, in order to activate it.
#[derive(Bundle)]
pub struct PlayerBundle {
    pub stats: Stats
}



#[derive(Component, Clone, Debug, Reflect)]
pub struct Stats {
    pub max_health: i32,
    pub movement_speed: f32,
    pub bullet_num: i32
}


// 坦克刷新子弹间隔
pub const TANK_REFRESH_BULLET_INTERVAL: f32 = 0.05;

// 坦克刷新子弹计时器
#[derive(Component, Deref, DerefMut)]
pub struct TankRefreshBulletTimer(pub Timer);

//敌人随机位置的子弹计时器
#[derive(Component, Deref, DerefMut)]
pub struct EnemyRefreshBulletTimer(pub Timer);

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
            translation: Vec3::new(0., -95., 100.0),
            ..default()
        },
        ..default()
    };

    let player_info_bundle = PlayerBundle {
        stats: Stats { max_health: 500, movement_speed: 15., bullet_num: 200 }
    };
    commands.spawn(Player).insert(player_bundle).insert(
        TankRefreshBulletTimer(Timer::from_seconds(
        TANK_REFRESH_BULLET_INTERVAL,
        TimerMode::Once,
    )))
    .insert(player_info_bundle).insert(Collider::ball(20.0));

    commands.spawn(Enemy).insert(
        EnemyRefreshBulletTimer(Timer::from_seconds(
        0.1,
        TimerMode::Repeating,
    )));


    //UI生命
    // Scoreboard
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "生命: ",
                TextStyle {
                    font: asset_server.load("fonts/qingfengfuan.ttf"),
                    font_size: 32.,
                    color: Color::BLACK,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/qingfengfuan.ttf"),
                font_size: 32.,
                color: Color::BLACK,
            }),
            TextSection::new(
                "子弹: ",
                TextStyle {
                    font: asset_server.load("fonts/qingfengfuan.ttf"),
                    font_size: 32.,
                    color: Color::BLACK,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/qingfengfuan.ttf"),
                font_size: 32.,
                color: Color::BLACK,
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(550.0),
                left: Val::Px(300.0),
                ..default()
            },
            ..default()
        }),
    );

}

fn update_uiboard(
    mut query: Query<&mut Text>,
    mut transform_query: Query<
    &mut Stats,
    With<Player>,
    >,
) {
    let mut text = query.single_mut();
    let stats = transform_query.single_mut();
    text.sections[1].value = stats.max_health.to_string();
    text.sections[3].value = stats.bullet_num.to_string();
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
        }).insert(Collider::ball(3.0));
}


#[derive(Component)]
pub struct Bulletenemy;

pub fn spawn_bullet_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    translation: Vec3
) {
    let bullet_texture_handle = asset_server.load("bulletenemy.png");
    let bullet_texture_atlas =
        TextureAtlas::from_grid(bullet_texture_handle, Vec2::new(7.0, 8.0), 4, 1, None, None);
    let bullet_texture_atlas_handle = texture_atlases.add(bullet_texture_atlas);

    commands
        .spawn(Bulletenemy)
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
        }).insert(Collider::ball(3.0));
}

// 炮弹移动
// 撞墙消失
pub fn move_bullet_enemy(
    mut _commands: Commands,
    mut transform_enemy_query: Query<&mut Transform, With<Bulletenemy>>
) {
    let bullet_movement = 1.0 * 5. * 1.;


    for mut bullet_enemy in &mut transform_enemy_query {
        
        bullet_enemy.translation.y -= bullet_movement 
    }
}

// 炮弹移动
// 撞墙消失
pub fn move_bullet(
    mut _commands: Commands,
    mut transform_query: Query<&mut Transform, With<Bullet>>,
) {
    let bullet_movement = 1.0 * 5. * 1.;
    for mut bullet_transform_query in &mut transform_query {
        
        bullet_transform_query.translation.y += bullet_movement 
    }
}


// 玩家攻击
fn players_attack(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<
    (&Transform, &mut TankRefreshBulletTimer, &mut Stats),
        With<Player>,
    >,
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    for (transform, mut refresh_bullet_timer,mut stats) in &mut player {
        refresh_bullet_timer.tick(time.delta());
        if keyboard_input.just_pressed(KeyCode::W) {
            if refresh_bullet_timer.finished() {
                // TODO startup时加载texture
                //子弹数量减少
                stats.bullet_num = stats.bullet_num - 1;
                if stats.bullet_num > 0 {
                    spawn_bullet(
                        &mut commands,
                        &asset_server,
                        &mut texture_atlases,
                        transform.translation,
                    );
                 }
                refresh_bullet_timer.reset();
            }
        }
    }
}

//随机位置生产子弹 并向下移动
fn swap_suiji_bullet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
    mut enemy: Query<
    &mut EnemyRefreshBulletTimer,
        With<Enemy>,
    >,
){
    //隔一段时间  随机出现在某位置 发射颗子弹
    for mut enemy_timer in &mut enemy{
        enemy_timer.tick(time.delta());
            if enemy_timer.finished() {
                let mut rng = thread_rng();
                let n: f32 = rng.gen_range(-140.0..140.);
                spawn_bullet_enemy(
                    &mut commands,
                    &asset_server,
                    &mut texture_atlases,
                    Vec3 { x: n, y:130., z: 100.},
                );
                enemy_timer.reset();
            }
    }
}


fn check_collide(
    mut commands: Commands,
    mut query1: Query<(Entity,
        &mut Transform,
        &mut Stats
        ),
        With<Player>,
    >,
    mut query2: Query<(
        Entity,
        &mut Transform),
        (With<Bulletenemy>,Without<Player>),
    >,
){
    for (player_ent,tranform,mut stats) in &mut query1{
          for (enemy_bullet_ent, enemy_transform) in &query2{
                let distance = tranform.translation.truncate().distance(enemy_transform.translation.truncate());
                if distance < 50. {
                    println!("检测出碰撞,距离{}",distance);
                    commands.entity(enemy_bullet_ent).despawn_recursive();
                    if stats.max_health > 0 {
                        stats.max_health = stats.max_health - 5;
                    } 
                }
          }
    }
}