fn main() {
    // #[cfg(target_arch = "wasm32")]
    // console_log::init_with_level(Level::Debug);
    // #[cfg(not(target_arch = "wasm32"))]
    // env_logger::init();

    quick_buck::run();
}
