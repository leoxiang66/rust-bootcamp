// #[derive(Copy, Clone)]
struct Config {
    enable_logging: bool,
}

fn main() {
    let config = Config {enable_logging: true};
    one(&config);
    two(&config);
}

fn one (config: &Config) {

}

fn two (config: &Config) {

}