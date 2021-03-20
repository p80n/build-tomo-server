use build_tomo_rs;

fn main() {
    env_logger::init();
    build_tomo_rs::rocket().launch();
}
