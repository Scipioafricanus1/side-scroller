use crate::prelude::*;

pub fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    window.set_maximized(true);

    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.5;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;
    commands.spawn((camera, MainCamera));
    
    commands.spawn( LdtkWorldBundle {
        ldtk_handle: asset_server.load( "tile_based_game.ldtk"),
        ..Default::default()
    });
}

pub fn window_resize_system(mut resize_events: EventReader<WindowResized>) {
    for event in resize_events.read() {
        println!("width = {} height = {}", event.width, event.height);
    }
}