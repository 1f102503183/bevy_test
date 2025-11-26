use bevy::{asset::LoadedFolder, image::ImageSampler, prelude::*};

pub struct PlayerPlugin;

// Main Player component
#[derive(Component)]
pub struct Id {
    id: u32,
}
#[derive(Component)]
pub struct HitPoint {
    hp: i32,
}
#[derive(Component)]
struct Movespeed {
    speed: f32,
}
#[derive(Component)]
pub struct Player;

// 謎の挨拶タイマー
#[derive(Resource)]
struct GreetTimer(Timer);

// animation resorce
#[derive(Component)]
struct AnimationTimer(Timer);

#[derive(Resource, Default)]
struct MoveAnimation(Handle<LoadedFolder>);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum AppState {
    #[default]
    Setup,
    Finished,
}

// player statuse
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum PlayerState {
    #[default]
    Stand,
    Walking,
}

// open folder
fn load_textures(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MoveAnimation(
        asset_server.load_folder("sprite/player/walk"),
    ));
}

// スプライトを読み込んでいる
fn check_textures(
    mut next_state: ResMut<NextState<AppState>>,
    move_animation: Res<MoveAnimation>,
    mut events: MessageReader<AssetEvent<LoadedFolder>>,
) {
    for events in events.read() {
        if events.is_loaded_with_dependencies(&move_animation.0) {
            next_state.set(AppState::Finished);
        }
    }
}

//　テクスチャアトラスを作る関数
fn create_texture_atlas(
    folder: &LoadedFolder,
    padding: Option<UVec2>,
    sampling: Option<ImageSampler>,
    textures: &mut ResMut<Assets<Image>>,
) -> (TextureAtlasLayout, TextureAtlasSources, Handle<Image>) {
    // Build a texture atlas using the individual sprites
    let mut texture_atlas_builder = TextureAtlasBuilder::default();

    texture_atlas_builder.padding(padding.unwrap_or_default());
    for handle in folder.handles.iter() {
        let id = handle.id().typed_unchecked::<Image>();
        let Some(texture) = textures.get(id) else {
            warn!(
                "{} did not resolve to an `Image` asset.",
                handle.path().unwrap()
            );
            continue;
        };

        texture_atlas_builder.add_texture(Some(id), texture);
    }

    let (texture_atlas_layout, texture_atlas_sources, texture) =
        texture_atlas_builder.build().unwrap();
    let texture = textures.add(texture);

    // Update the sampling settings of the texture atlas
    let image = textures.get_mut(&texture).unwrap();
    image.sampler = sampling.unwrap_or_default();

    (texture_atlas_layout, texture_atlas_sources, texture)
}

fn setup_player(
    mut commands: Commands,
    animation_handle: Res<MoveAnimation>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    loaded_folder: Res<Assets<LoadedFolder>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let loaded_folder = loaded_folder.get(&animation_handle.0).unwrap();

    let (texture_atlas_nearest_padded, nearest_padded_sources, nearest_padded_texture) =
        create_texture_atlas(
            loaded_folder,
            Some(UVec2::new(6, 6)),
            Some(ImageSampler::nearest()),
            &mut textures,
        );
    let atlas_layout_handle = texture_atlases.add(texture_atlas_nearest_padded);

    // (2) アトラスから "stand.png" のインデックスを取得
    // "walk1.png" のハンドルを AssetServer から取得
    let walk1_handle: Handle<Image> = asset_server
        .get_handle("sprite/player/walk/walk1.png")
        .unwrap();
    //　ソース情報を使って、うえのwalk1.pngがアトラスの何番目か調べる
    let walk1_index = nearest_padded_sources
        .handle(atlas_layout_handle.clone(), &walk1_handle)
        .expect("can't find walk1.png");

    commands.spawn(Camera2d); // Bundleとかくとエラーになる
    commands.spawn((
        Id { id: 0 },
        HitPoint { hp: 100 },
        Movespeed { speed: 1.5 },
        Sprite {
            custom_size: Some(Vec2::new(150.0, 150.0)),
            image: nearest_padded_texture.clone(),
            texture_atlas: Some(walk1_index),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Player,
    ));
}

fn animation(
    time: Res<Time>,
    texture_atlas_layouts: Res<Assets<TextureAtlasLayout>>,
    player_state: Res<State<PlayerState>>,
    mut query: Query<(&mut Sprite, &mut AnimationTimer), With<Player>>,
) {
    for (mut sprite, mut animation_timer) in query.iter_mut() {
        animation_timer.0.tick(time.delta());
        if animation_timer.0.just_finished() {
            match player_state.get() {
                PlayerState::Walking => {
                    //.as_mut() でOptionの中身を可変参照で取り出す
                    if let Some(atlas) = sprite.texture_atlas.as_mut() {
                        //アトラスのレイアウト情報を取得、もしなかったらスキップ
                        let Some(layout) = texture_atlas_layouts.get(&atlas.layout) else {
                            continue;
                        };
                        //総フレーム数を取得
                        let frames = layout.len();
                        // アトラスのインデックスを更新
                        atlas.index = (atlas.index + 1) % frames;
                    }
                }
                PlayerState::Stand => (),
            }
        }
    }
}

fn check_entety(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<(&Id, &HitPoint, &Transform)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for (id, hp, transform) in &query {
            println!(
                "entety ID : {} health : {} position is : {},{},{}",
                id.id,
                hp.hp,
                transform.translation.x,
                transform.translation.y,
                transform.translation.z
            );
        }
    }
}

fn move_player(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Movespeed), With<Player>>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut player_state: ResMut<NextState<PlayerState>>,
) {
    for (mut transform, movespeed) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        if key_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        } else if key_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        } else if key_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        } else if key_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if direction != Vec3::ZERO {
            player_state.set(PlayerState::Walking);
            transform.translation +=
                direction.normalize() * movespeed.speed * time.delta_secs() * 100.0;
        } else {
            player_state.set(PlayerState::Stand);
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .init_state::<PlayerState>()
            .insert_resource(GreetTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
            .add_systems(OnEnter(AppState::Setup), load_textures) // OnEnterでAppStateがSetupの時動くようになる
            // 2. AppState::Setup の間、毎フレーム読み込み完了をチェック
            .add_systems(Update, check_textures.run_if(in_state(AppState::Setup)))
            // OnEnter はその状態になったとき一度だけ
            .add_systems(OnEnter(AppState::Finished), setup_player)
            .add_systems(
                Update,
                (move_player, check_entety, animation).run_if(in_state(AppState::Finished)),
            );
    }
}
