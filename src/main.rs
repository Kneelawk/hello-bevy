fn main() {
    // Setup the non-WASM logger.
    dotenv::dotenv().ok();
    env_logger::init();

    hello_bevy::run();
}
