// src/main.rs

use bevy::prelude::*;
use bevy::render::renderer::RenderAdapterInfo;

fn main() {
    env_logger::init(); 
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, print_adapter_info)
        .add_systems(Startup, hello)
        .run();
    //test
}

fn hello() {
    println!("Hello, Bevy!");
}

fn print_adapter_info(adapter_info: Option<Res<RenderAdapterInfo>>) {
    if let Some(info) = adapter_info {
        println!("GPU: {}", info.name);
        println!("Backend: {:?}", info.backend);
    } else {
        println!("GPU情報が取得できませんでした/テストです");
    }
}

