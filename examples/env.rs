use traccia::parse_level_from_env;

fn main() {
    println!("detected log level: {:?}", parse_level_from_env());
}
