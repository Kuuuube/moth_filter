use std::fs::File;

use serde::Serialize;

use crate::tsv_structs::*;

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
    let vernacular_tsv = vernacular_tsv_reader.deserialize::<VernacularNameTSVRaw>();
    let mut species_profile_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(File::open("./data/SpeciesProfile.tsv").unwrap());
    let species_profile_tsv = species_profile_tsv_reader.deserialize::<SpeciesProfileTSVRaw>();
    let mut distribution_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(File::open("./data/Distribution.tsv").unwrap());
    let distribution_tsv = distribution_tsv_reader.deserialize::<DistributionTSVRaw>();

    let mut moth_entry_count = 0;
    let mut bad_entry_count = 0;

    for tsv_reader_result in taxon_tsv {
        match tsv_reader_result {
            Ok(taxon_tsv_data_raw) => {
                if taxon_tsv_data_raw.dwc_taxon_rank != "species"
                    || taxon_tsv_data_raw.dwc_order != MOTH_ORDER
                    || taxon_tsv_data_raw.dwc_superfamily == BUTTERFLY_SUPERFAMILY
                {
                    // not a moth
                    continue;
                }
            }
            Err(_err) => {
                bad_entry_count += 1;
                continue;
            }
        }
        moth_entry_count += 1;
    }

    println!("Found {moth_entry_count} moths");
    println!("Failed to parse {bad_entry_count} entries");
}

}

#[derive(Debug, Serialize)]
struct SpeciesData {
    catalogue_of_life_taxon_id: String,
    taxonomic_status: TaxonomicStatus,
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

#[derive(Debug, Serialize)]
enum TaxonomicStatus {
    Accepted,
    ProvisionallyAccepted,
    Synonym(String),
}
