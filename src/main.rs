use std::{
    collections::{HashMap, HashSet},
    fs::File,
    time::Instant,
};

use serde::Serialize;

use crate::{addin_tsv_hashmaps::VernacularHashKey, tsv_structs::*};

mod addin_tsv_hashmaps;
mod tsv_structs;

const MOTH_ORDER: &str = "Lepidoptera";
const BUTTERFLY_SUPERFAMILY: &str = "Papilionoidea";

fn main() {
    let start_time = Instant::now();

    let mut taxon_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(File::open("./data/Taxon.tsv").unwrap());
    let taxon_tsv = taxon_tsv_reader.deserialize::<TaxonTSVRaw>();

    let mut vernacular_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(File::open("./data/VernacularName.tsv").unwrap());
    let vernacular_tsv = addin_tsv_hashmaps::vernacular_to_hashmap(
        vernacular_tsv_reader.deserialize::<VernacularNameTSVRaw>(),
    );

    let mut species_profile_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(File::open("./data/SpeciesProfile.tsv").unwrap());
    let species_profile_tsv = addin_tsv_hashmaps::species_profile_to_hashmap(
        species_profile_tsv_reader.deserialize::<SpeciesProfileTSVRaw>(),
    );

    let mut distribution_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(File::open("./data/Distribution.tsv").unwrap());
    let distribution_tsv = addin_tsv_hashmaps::distribution_to_hashmap(
        distribution_tsv_reader.deserialize::<DistributionTSVRaw>(),
    );

    let output_file_path = "./output/moth_data.json";
    let output_file = File::create(output_file_path).unwrap();

    let mut bad_entry_count = 0;
    let mut moth_entries: Vec<SpeciesData> = Vec::new();
    let mut synonyms: HashMap<String, Vec<SynonymSpecies>> = HashMap::new();
    let mut moth_ids: HashSet<String> = HashSet::new();

    for tsv_reader_result in taxon_tsv {
        let Ok(taxon_tsv_data_raw) = tsv_reader_result else {
            bad_entry_count += 1;
            continue;
        };

        // filter out not species before checking for synonyms
        if taxon_tsv_data_raw.dwc_taxon_rank != "species" {
            continue;
        }

        // synonyms have nearly no data and will never be detected as a moth, run before moth check and filter out non moths later
        match taxon_tsv_data_raw.dwc_taxonomic_status {
            TaxonomicStatusRaw::Synonym | TaxonomicStatusRaw::AmbiguousSynonym => {
                let primary_taxon_id = taxon_tsv_data_raw.dwc_accepted_name_usage_id;
                if let Some(genus) = taxon_tsv_data_raw.dwc_generic_name
                    && let Some(epithet) = taxon_tsv_data_raw.dwc_specific_epithet
                {
                    let synonym = SynonymSpecies {
                        catalogue_of_life_taxon_id: taxon_tsv_data_raw.dwc_taxon_id,
                        genus: genus,
                        epithet: epithet,
                    };
                    synonyms
                        .entry(primary_taxon_id)
                        .and_modify(|x| {
                            x.push(synonym.clone());
                        })
                        .or_insert(vec![synonym]);
                }
                continue;
            }
            TaxonomicStatusRaw::Misapplied => {
                continue;
            }
            _ => (),
        };

        // continue if not a moth
        let Some(order) = &taxon_tsv_data_raw.dwc_order else {
            continue;
        };
        if order != MOTH_ORDER {
            continue;
        }
        if let Some(superfamily) = &taxon_tsv_data_raw.dwc_superfamily
            && superfamily == BUTTERFLY_SUPERFAMILY
        {
            continue;
        }

        moth_ids.insert(taxon_tsv_data_raw.dwc_taxon_id.clone());

        let common_name = vernacular_tsv.get(&VernacularHashKey {
            language_code: "eng".to_string(),
            taxon_id: taxon_tsv_data_raw.dwc_taxon_id.clone(),
        });
        let species_profile = species_profile_tsv
            .get(&taxon_tsv_data_raw.dwc_taxon_id)
            .and_then(|x| {
                Some(SpeciesProfile {
                    extinct: x.gbif_is_extinct,
                    freshwater: x.gbif_is_freshwater,
                    marine: x.gbif_is_marine,
                })
            });

        let distribution = distribution_tsv
            .get(&taxon_tsv_data_raw.dwc_taxon_id)
            .and_then(|x| {
                let threat_status = x.iucn_threat_status.as_ref().and_then(|x| match x {
                    ThreatStatusRaw::LeastConcern => Some(ThreatStatus::LeastConcern),
                    ThreatStatusRaw::Vulnerable => Some(ThreatStatus::Vulnerable),
                    ThreatStatusRaw::Endangered => Some(ThreatStatus::Endangered),
                    ThreatStatusRaw::CriticallyEndangered => {
                        Some(ThreatStatus::CriticallyEndangered)
                    }
                    ThreatStatusRaw::ExtinctInTheWild => Some(ThreatStatus::ExtinctInTheWild),
                    ThreatStatusRaw::Extinct => Some(ThreatStatus::Extinct),
                    ThreatStatusRaw::NotEvaluated => None,
                    ThreatStatusRaw::DataDeficient => None,
                });
                if x.dwc_locality.is_none() && threat_status.is_none() {
                    return None;
                }
                Some(Distribution {
                    locality: x.dwc_locality.clone(),
                    threat_status,
                })
            });

        // some malformed entries dont have a `genus` but have a `generic name` which is synonymous
        let genus_fixed = match taxon_tsv_data_raw.dwc_genus {
            Some(some) => some,
            None => match taxon_tsv_data_raw.dwc_generic_name {
                Some(some) => some,
                None => {
                    bad_entry_count += 1;
                    continue;
                }
            },
        };

        let Some(epithet_checked) = taxon_tsv_data_raw.dwc_specific_epithet else {
            bad_entry_count += 1;
            continue;
        };

        moth_entries.push(SpeciesData {
            catalogue_of_life_taxon_id: taxon_tsv_data_raw.dwc_taxon_id,
            classification: ScientificClassification {
                superfamily: taxon_tsv_data_raw.dwc_superfamily,
                family: taxon_tsv_data_raw.dwc_family,
                subfamily: taxon_tsv_data_raw.dwc_subfamily,
                tribe: taxon_tsv_data_raw.dwc_tribe,
                subtribe: taxon_tsv_data_raw.dwc_subtribe,
                genus: genus_fixed,
                epithet: epithet_checked,
            },
            common_names: common_name.cloned(),
            species_profile: species_profile,
            distribution: distribution,
            synonyms: None,
        });
    }

    synonyms.retain(|key, _value| moth_ids.contains(key));
    let moth_synonyms_count: usize = synonyms.iter().map(|x| x.1.len()).sum();

    for moth_entry in moth_entries.iter_mut() {
        moth_entry.synonyms = synonyms
            .get(&moth_entry.catalogue_of_life_taxon_id)
            .cloned();
    }

    println!(
        "Found {} moths and {} synonym species",
        moth_entries.len(),
        moth_synonyms_count,
    );
    if bad_entry_count > 0 {
        println!("Failed to parse {bad_entry_count} entries");
    }
    println!("Parsed in: {:.6?}", start_time.elapsed());

    println!("Writing output to {}", output_file_path);
    if let Err(write_error) = serde_json::to_writer_pretty(output_file, &moth_entries) {
        dbg!(write_error);
    };
}

#[derive(Debug, Serialize)]
struct SpeciesData {
    catalogue_of_life_taxon_id: String,
    classification: ScientificClassification,
    #[serde(skip_serializing_if = "Option::is_none")]
    common_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    species_profile: Option<SpeciesProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    distribution: Option<Distribution>,
    #[serde(skip_serializing_if = "Option::is_none")]
    synonyms: Option<Vec<SynonymSpecies>>,
}

#[derive(Debug, Serialize)]
struct Distribution {
    #[serde(skip_serializing_if = "Option::is_none")]
    locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    threat_status: Option<ThreatStatus>,
}

#[derive(Debug, Serialize)]
pub enum ThreatStatus {
    LeastConcern,
    Vulnerable,
    Endangered,
    CriticallyEndangered,
    ExtinctInTheWild,
    Extinct,
}

#[derive(Debug, Serialize)]
struct SpeciesProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    extinct: Option<bool>,
    // yep, aquatic moths are a thing though this database doesn't have many of them identified
    #[serde(skip_serializing_if = "Option::is_none")]
    freshwater: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marine: Option<bool>,
}

#[derive(Debug, Serialize)]
struct ScientificClassification {
    // somehow any of these (even genus and epithet) can be empty for a species
    #[serde(skip_serializing_if = "Option::is_none")]
    superfamily: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subfamily: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tribe: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtribe: Option<String>,
    genus: String,
    epithet: String,
}

#[derive(Debug, Clone, Serialize)]
struct SynonymSpecies {
    catalogue_of_life_taxon_id: String,
    genus: String,
    epithet: String,
}
