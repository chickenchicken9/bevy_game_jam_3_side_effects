use crate::loading::FontAssets;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system(setup_menu.in_schedule(OnEnter(GameState::Menu)))
            .add_system(click_play_button.in_set(OnUpdate(GameState::Menu)))
            .add_system(cleanup_menu.in_schedule(OnExit(GameState::Menu)));
    }
}

#[derive(Resource)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
        }
    }
}

#[derive(Component)]
struct MenuEntity;

fn setup_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    textures: Res<TextureAssets>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(MenuEntity)
    .insert(TextBundle::from_section(
        "Can you run a hospital?\nWatch out, every action you take\nmight have a... side effect.",
        TextStyle {
            font: font_assets.fira_sans.clone(),
            font_size: 30.0,
            color: Color::rgb(0.9, 0.9, 0.9),
        })
        .with_text_alignment(TextAlignment::Right)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(15.0),
                right: Val::Px(25.0),
                ..default()
            },
            ..default()
        })
    );
    commands.spawn(MenuEntity).insert(ImageBundle {
        image: UiImage {
            texture: textures
                .folder
                .get("textures/hospital.png")
                .unwrap()
                .clone(),
            ..default()
        },
        ..default()
    });
    commands
        .spawn(MenuEntity)
        .insert(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: button_colors.normal.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Play",
                TextStyle {
                    font: font_assets.fira_sans.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
        });
}

fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, menu_entities: Query<Entity, With<MenuEntity>>) {
    for e in menu_entities.iter() {
        commands.entity(e).despawn_recursive();
    }
}
