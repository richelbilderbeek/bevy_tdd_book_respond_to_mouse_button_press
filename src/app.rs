use bevy::input::InputPlugin;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn create_app() -> App {
    let mut app = App::new();

    // Only add this plugin in testing.
    // The main app will assume it to be absent
    //
    // The function 'try_add_plugins' 
    // (https://github.com/bevyengine/bevy/discussions/15802#discussioncomment-10898148)
    // will make this if obsolete and increase code coverage.
    // Thanks mgi388 for pointing this out
    if cfg!(test) {
        app.add_plugins(InputPlugin);
    }

    app.add_systems(Startup, add_player);
    app.add_systems(Update, respond_to_mouse_button_press);

    // Do not do update, as this will disallow to do more steps
    // app.update(); //Don't!
    app
}

fn add_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(64.0, 32.0, 1.0),
                ..default()
            },
            ..default()
        },
        Player {},
    ));
}

fn respond_to_mouse_button_press(
    mut query: Query<&mut Transform, With<Player>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    let mut player_position = query.single_mut();
    if input.pressed(MouseButton::Left) {
        // Do something
        player_position.translation.x += 16.0;
    }
}

#[cfg(test)]
fn count_n_players(app: &mut App) -> usize {
    let mut query = app.world_mut().query::<&Player>();
    return query.iter(app.world()).len();
}

#[cfg(test)]
fn get_player_position(app: &mut App) -> Vec2 {
    // Do 'app.update()' before calling this function,
    // else this assert goes off.
    assert_eq!(count_n_players(app), 1);
    let mut query = app.world_mut().query::<(&Transform, &Player)>();
    let (transform, _) = query.single(app.world());
    transform.translation.xy()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_app_has_no_players() {
        let mut app = App::new();
        assert_eq!(count_n_players(&mut app), 0);
    }

    #[test]
    fn test_create_app_has_a_player() {
        let mut app = create_app();
        app.update();
        assert_eq!(count_n_players(&mut app), 1);
    }

    #[test]
    fn test_player_is_at_origin() {
        let mut app = create_app();
        app.update();
        assert_eq!(get_player_position(&mut app), Vec2::new(0.0, 0.0));
    }

    #[test]
    fn test_player_responds_to_mouse_button_press() {
        let mut app = create_app();
        assert!(app.is_plugin_added::<InputPlugin>());
        app.update();

        // Not moved yet
        assert_eq!(Vec2::new(0.0, 0.0), get_player_position(&mut app));

        // Press the left mouse button
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Left);

        app.update();

        // Position must have changed now
        assert_ne!(Vec2::new(0.0, 0.0), get_player_position(&mut app));
    }
}
