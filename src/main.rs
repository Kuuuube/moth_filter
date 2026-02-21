use std::{collections::HashMap, fs::File};

use serde::Serialize;

use crate::tsv_structs::*;

mod addin_tsv_hashmaps;
mod tsv_structs;

const MOTH_ORDER: &str = "Lepidoptera";
const BUTTERFLY_SUPERFAMILY: &str = "Papilionoidea";

fn main() {
    let mut taxon_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(File::open("./data/Taxon.tsv").unwrap());
    let taxon_tsv = taxon_tsv_reader.deserialize::<TaxonTSVRaw>();

    let mut vernacular_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(File::open("./data/VernacularName.tsv").unwrap());
    let vernacular = addin_tsv_hashmaps::vernacular_to_hashmap(
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

    let mut bad_entry_count = 0;
    let mut moth_entries: Vec<SpeciesData> = Vec::new();
    let mut moth_synonyms: HashMap<String, String> = HashMap::new();

    for tsv_reader_result in taxon_tsv {
        let Ok(taxon_tsv_data_raw) = tsv_reader_result else {
            bad_entry_count += 1;
            continue;
        };
        if taxon_tsv_data_raw.dwc_taxon_rank != "species"
            || taxon_tsv_data_raw.dwc_order != MOTH_ORDER
            || taxon_tsv_data_raw.dwc_superfamily == BUTTERFLY_SUPERFAMILY
        {
            // not a moth
            continue;
        }
        match taxon_tsv_data_raw.dwc_taxonomic_status {
            TaxonomicStatusRaw::Synonym | TaxonomicStatusRaw::AmbiguousSynonym => {
                moth_synonyms.insert(taxon_tsv_data_raw.dwc_taxon_id, taxon_tsv_data_raw.dwc_accepted_name_usage_id);
                continue;
            },
            TaxonomicStatusRaw::Misapplied => {
                continue;
            }
            _ => ()
        };

        moth_entries.push(SpeciesData {
            catalogue_of_life_taxon_id: taxon_tsv_data_raw.dwc_taxon_id,
            classification: ScientificClassification {
                superfamily: taxon_tsv_data_raw.dwc_superfamily,
                family: taxon_tsv_data_raw.dwc_family,
                subfamily: taxon_tsv_data_raw.dwc_subfamily,
                tribe: taxon_tsv_data_raw.dwc_tribe,
                subtribe: taxon_tsv_data_raw.dwc_subtribe,
                genus: taxon_tsv_data_raw.dwc_genus,
                epithet: taxon_tsv_data_raw.dwc_specific_epithet,
            },
        });
    }

    println!("Found {} moths and {} synonym species", moth_entries.len(), moth_synonyms.len());
    println!("Failed to parse {bad_entry_count} entries");
}

#[derive(Debug, Serialize)]
struct SpeciesData {
    catalogue_of_life_taxon_id: String,
    classification: ScientificClassification,
}

#[derive(Debug, Serialize)]
struct ScientificClassification {
    superfamily: String,
    family: String,
    subfamily: String,
    tribe: String,
    subtribe: String,
    genus: String,
    epithet: String,
}
