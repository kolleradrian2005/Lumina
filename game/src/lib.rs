#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Trace),
    );
    log::debug!("Starting android app!");
    use lumina_engine::start;
    use winit::event_loop::EventLoopBuilder;
    use winit::platform::android::EventLoopBuilderExtAndroid;
    let event_loop = EventLoopBuilder::new()
        .with_android_app(app)
        .build()
        .unwrap();
    start(event_loop);
}
