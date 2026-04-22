use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subspecies: Option<Vec<SubSpecies>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_in: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Distribution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threat_status: Option<ThreatStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatStatus {
    LeastConcern,
    ConservationDependent,
    NearThreatened,
    Vulnerable,
    Endangered,
    CriticallyEndangered,
    ExtinctInTheWild,
    Extinct,
}

impl Display for ThreatStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ThreatStatus::LeastConcern => "Least Concern",
                ThreatStatus::ConservationDependent => "Conservation Dependent",
                ThreatStatus::NearThreatened => "Near Threatened",
                ThreatStatus::Vulnerable => "Vulnerable",
                ThreatStatus::Endangered => "Endangered",
                ThreatStatus::CriticallyEndangered => "Critically Endangered",
                ThreatStatus::ExtinctInTheWild => "Extinct in the Wild",
                ThreatStatus::Extinct => "Extinct",
            }
        )
    }
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
    // somehow any of these (even genus and specific name) can be empty for a species
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
    pub specific: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subspecific: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynonymSpecies {
    pub catalogue_of_life_taxon_id: String,
    pub genus: String,
    pub specific: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subspecific: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubSpecies {
    pub catalogue_of_life_taxon_id: String,
    pub genus: String,
    pub specific: String,
    pub subspecific: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ButterflyBlacklist {
    pub families: HashSet<String>,
    pub subfamilies: HashSet<String>,
    pub tribes: HashSet<String>,
    pub subtribes: HashSet<String>,
    pub genera: HashSet<String>,
    pub specifics: HashSet<String>,
    pub subspecifics: HashSet<String>,
}

pub type MothSynonyms = HashMap<String, String>;
