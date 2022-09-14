use bevy::{
    prelude::*,
};

pub fn keyboard_input(key_evr: Res<Input<KeyCode>>, mut cameras: Query<&mut Transform, With<Camera>>) {
    let mut camera = cameras.single_mut();
    // for mut camera in cameras.iter_mut() {
        
        if key_evr.pressed(KeyCode::W){
            camera.translation = camera.translation + camera.forward() * 0.1;
        }
        if key_evr.pressed(KeyCode::S){
            camera.translation = camera.translation + camera.back() * 0.1;
        }
        if key_evr.pressed(KeyCode::A){
            camera.translation = camera.translation + camera.left() * 0.1;
        }
        if key_evr.pressed(KeyCode::D){
            camera.translation = camera.translation + camera.right() * 0.1;
        }

        if key_evr.pressed(KeyCode::Q){
            camera.translation = camera.translation + camera.down() * 0.1;
        }
        if key_evr.pressed(KeyCode::E){
            camera.translation = camera.translation + camera.up() * 0.1;
        }

        if key_evr.pressed(KeyCode::Left){
            camera.rotate_local_y(0.01);
        }
        if key_evr.pressed(KeyCode::Right){
            camera.rotate_local_y(-0.01);
        }
        if key_evr.pressed(KeyCode::Up){
            camera.rotate_local_x(0.01);
        }
        if key_evr.pressed(KeyCode::Down){
            camera.rotate_local_x(-0.01);
        }
    // }
}