use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciesData {
    pub catalogue_of_life_taxon_id: String,
    pub classification: ScientificClassification,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub species_profile: Option<SpeciesProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distribution: Option<Distribution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<SynonymSpecies>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Distribution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_status: Option<ThreatStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatStatus {
    LeastConcern,
    Vulnerable,
    Endangered,
    CriticallyEndangered,
    ExtinctInTheWild,
    Extinct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeciesProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extinct: Option<bool>,
    // yep, aquatic moths are a thing though this database doesn't have many of them identified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freshwater: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marine: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScientificClassification {
    // somehow any of these (even genus and epithet) can be empty for a species
    #[serde(skip_serializing_if = "Option::is_none")]
    pub superfamily: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subfamily: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tribe: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtribe: Option<String>,
    pub genus: String,
    pub epithet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynonymSpecies {
    pub catalogue_of_life_taxon_id: String,
    pub genus: String,
    pub epithet: String,
}
