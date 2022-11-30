use std::{time::Duration, fs::File, io::BufReader, path::Path, sync::Arc};
use enigo::Enigo;
use notify::Watcher;
use obws::Client;
use tokio::{time::sleep, sync::Mutex};
use winit::{event_loop::{EventLoop}, window::{WindowBuilder, Window}};

mod model;
use crate::model::Config;

fn main() {

    // load config
    let config_path = "res/config.json";
    let config = load_config(config_path);
    if config.is_none() {
        panic!("Failed to load config file: {}", config_path);
    }
    let config = Arc::new(Mutex::new(config.unwrap()));

    // watch for config changes
    let config_clone = config.clone();
    let mut watcher = notify::recommended_watcher(move |res| {
        let config_clone = config_clone.clone();
        match res {
            Ok(_) => {
                let mut config_lock = config_clone.blocking_lock();
                if let Some(new_config) = load_config(config_path) {
                    *config_lock = new_config;
                    println!("Config reloaded");
                }
            },
            Err(e) => println!("watch error: {:?}", e)
        }
    }).unwrap();
    watcher.watch(Path::new(config_path), notify::RecursiveMode::NonRecursive).unwrap();
    
    // initialize winit
    let runtime = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_visible(false);

    // display initial values for the lulz
    println!("All detected monitors:");
    for monitor in window.available_monitors() {
        println!("name: {:?}, position: {:?}, size: {:?}", monitor.name(), monitor.position(), monitor.size());
    }
    
    // every 10 ms, check for mouse movement
    // tokio and winit hate each other so I opted to run the tokio runtime manually vs async main
    runtime.block_on(async move {
        let config_lock = config.lock().await;
        let password: Option<&str> = Some(&config_lock.obs_password);
        let client = Client::connect("localhost", 4455, password).await.unwrap();
        drop(config_lock);

        let mut cached_monitor_name = get_cursor_monitor(&window).await;
        println!("Mouse currently in monitor: {:?}", cached_monitor_name);
        loop {
            sleep(Duration::from_millis(10)).await;
            let current_monitor_name = get_cursor_monitor(&window).await;
            if current_monitor_name != cached_monitor_name {
                println!("Monitor changed from {:?} to {:?}", cached_monitor_name, current_monitor_name);
                cached_monitor_name = current_monitor_name;

                let config_lock = config.lock().await;
                if !config_lock.enabled {
                    continue;
                }
                for mapping in config_lock.mappings.iter() {
                    if Some(mapping.monitor_name.clone()) == cached_monitor_name {
                        println!("Switching to scene: {:?}", mapping.obs_scene_name);
                        client.scenes().set_current_program_scene(&mapping.obs_scene_name).await.unwrap();
                        break;
                    }
                }
            }
        }
    });
}

fn load_config(path: &str) -> Option<Config> {
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        if let Ok(config) = serde_json::from_reader(reader) {
            return Some(config);
        }
    }
    None
}

async fn get_cursor_monitor(window: &Window) -> Option<String> {
    let cursor_location: (i32, i32) = Enigo::mouse_location();
    for monitor in window.available_monitors() {

        let monitor_position = monitor.position();
        let monitor_size = monitor.size();
        
        if cursor_location.0 < monitor_position.x + monitor_size.width as i32 && cursor_location.0 > monitor_position.x && cursor_location.1 < monitor_position.y + monitor_size.height as i32 && cursor_location.1 > monitor_position.y {
            if let Some(name) = monitor.name() {
                return Some(name);
            } else {
                return None;
            }
        }
    }
    None
}
