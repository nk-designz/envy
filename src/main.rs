use std::env;
use std::fs::File;
use std::io::Write;

const PREFIX: &str = "ENVY_";

fn main() {
    let prop_path = env::var("ENVY_PROPERTIES_PATH").unwrap();
    println!("Envy path:\n\t{}\nFound variables:", prop_path);
    let mut file = File::create(&prop_path).unwrap();
    for (key, var) in env::vars() {
        if key.contains(PREFIX) && key != "ENVY_PROPERTIES_PATH" {
            let prop_key = key
                .trim_start_matches(PREFIX)
                .to_lowercase()
                .replace("_", ".");
            println!("\t{}", prop_key);
            writeln!(&mut file, "{0}={1}", prop_key, var).unwrap();
        }
    }
}
