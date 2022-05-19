mod configuration;

use configuration::Settings;

fn main() {
    let settings = Settings::new();
    println!("{:#?}", settings);
}
