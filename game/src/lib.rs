pub mod camera;
pub mod fish;
pub mod game;
pub mod player;
pub mod postprocess;
pub mod scene;
pub mod sea_trash;

pub use game::initialize;

#[cfg(target_os = "android")]
pub use game::initialize_android;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Trace),
    );
    log::debug!("Starting android app!");
    initialize_android(app);
}
