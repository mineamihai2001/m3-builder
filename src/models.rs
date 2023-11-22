use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub class: String,
    #[serde(rename(deserialize = "type"))]
    pub entity_type: String,
    pub id: String,
    pub name: String,
    pub value: String,
    pub description: String,
    pub category_id: String,
    pub last_updated_by: String,
    pub last_updated: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub dsp_spec_version: String,
    pub last_updated_by: String,
    pub entities: Vec<Entity>,
}

#[allow(dead_code)]
pub struct M3Color {
    pub key: String,
    pub value: String,
}

pub type M3Theme = BTreeMap<String, String>;
