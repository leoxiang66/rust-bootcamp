#[derive(Clone)]
struct Config {
    enable_logging: bool,
}

fn main() {
    let config = Config {enable_logging: true};
    one(config.clone());
    two(config);

    loop {
        let vec = Vec::new();
        let mut index = 0;

        if index == 5 {
            drop(vec);
            break;
        }

        drop(vec);
    }
}

fn one (config: Config) {

}

fn two (config: Config) {

}