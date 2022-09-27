// Structs for Ingressing Json responses from IONOS

use serde::{Deserialize, Serialize};

// Highlevel Zone dump
// Parent - Vec of Zones
pub type ZDump = Vec<Zones>;
// Child(s) - Zone information
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Zones {
    pub name: String,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

// Zone record dump
// Parent - Zone Context
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZConfig {
    pub name: String,
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub records: Recs,
}
// Child(s) - Records and Associated configurations
// Recs Type declaration
pub type Recs = Vec<Record>;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub name: String,
    pub root_name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub content: String,
    pub change_date: String,
    pub ttl: i64,
    pub disabled: bool,
    pub id: String,
}

impl Record {
    // Finds record that contains a wildcard A record
    pub fn is_wildcard(&self) -> bool {
        self.name.find("*") != Option::None && self.type_field == "A"
    }
}
