use std::{fs, path::Path};

use m3_builder::{create_theme, deserialize, serialize};
use models::M3Theme;

mod models;

fn parse_args(args: Vec<String>) -> Option<(String, String)> {
    if args[1] == "--help" {
        println!(
            "Options: 
                --source <path to source file> (default \"theme/in.json\")
                --target <path to target file> (default \"theme/out.json\")
            "
        );
        return None;
    }

    let dir = Path::new("theme");
    if !dir.exists() {
        fs::create_dir(dir).expect("Could not create theme directory");
    }

    let mut source = "theme/in.json".to_string();
    let mut target = "theme/out.json".to_string();

    let source_arg = match args.iter().position(|r| r == "--source") {
        None => 0,
        Some(i) => i,
    };
    let target_arg = match args.iter().position(|r| r == "--target") {
        None => 0,
        Some(i) => i,
    };

    if source_arg != 0 {
        source = args[source_arg + 1].to_string();
    }

    if target_arg != 0 {
        target = args[target_arg + 1].to_string();
    }

    Some((source, target))
}

fn main() {
    match parse_args(std::env::args().collect()) {
        None => (),
        Some((source, target)) => {
            let token = deserialize(&source).expect("Deserialize error");

            let theme: M3Theme = create_theme(token);

            match serialize(theme, &target) {
                Ok(_) => println!("Theme successfully created"),
                Err(err) => println!("Error creating theme {}", err),
            }
        }
    }
}
