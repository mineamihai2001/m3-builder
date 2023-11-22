use std::{fs, path::Path};

use models::{Entity, M3Color, M3Theme, Token};
mod models;

impl Entity {
    fn to_m3color(&self) -> M3Color {
        M3Color::new(
            self.name
                .clone()
                .replace("sys.", "")
                .replace("color.", "")
                .replace("ref.", "")
                .replace("md.", ""),
            self.value.clone(),
        )
    }
}

impl M3Color {
    fn new(key: String, value: String) -> Self {
        M3Color { key, value }
    }

    fn to_camel(word: &String, separator: &str) -> String {
        let result = word
            .clone()
            .split(separator)
            .map(|chunk| {
                let first = chunk.chars().nth(0).unwrap().to_ascii_uppercase();
                let rest = &chunk[1..chunk.len()];
                return format!("{}{}", first, rest);
            })
            .collect::<Vec<String>>()
            .join("");

        return format!(
            "{}{}",
            result.chars().nth(0).unwrap().to_ascii_lowercase(),
            &result[1..result.len()]
        );
    }

    fn get_key(&self) -> String {
        let new_key = M3Color::to_camel(&self.key, ".");
        M3Color::to_camel(&new_key, "-")
    }
    fn get_value(&self) -> String {
        self.value.clone()
    }
}

pub fn deserialize(source: &String) -> Result<Token, &str> {
    let path = Path::new(source);
    if !path.exists() {
        return Err("Path does not exists");
    }

    let data: String = fs::read_to_string(path).expect("Could not read source file");
    let deserialized =
        serde_json::from_str(data.as_str()).expect("Could not deserialize source file");

    Ok(deserialized)
}

pub fn serialize(theme: M3Theme, out: &String) -> Result<(), &str> {
    let path = Path::new(out);

    let serialized =
        serde_json::to_string::<M3Theme>(&theme).expect("Could not serialize M3Theme object");
    fs::write(path, serialized).expect("Could not write to out file");

    Ok(())
}

pub fn create_theme(token: Token) -> M3Theme {
    let mut result: M3Theme = M3Theme::new();

    token.entities.iter().for_each(|entity| {
        let color = entity.to_m3color();
        if entity.entity_type == "color" {
            result.insert(color.get_key(), color.get_value());
        }
    });

    return result;
}
