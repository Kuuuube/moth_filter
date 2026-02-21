use std::fs::File;

use serde::Deserialize;

const MOTH_ORDER: &str = "Lepidoptera";
const BUTTERFLY_SUPERFAMILY: &str = "Papilionoidea";

fn main() {
    let taxon_tsv_file = File::open("./data/Taxon.tsv").unwrap();
    let mut tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(taxon_tsv_file);

    let mut moth_entry_count = 0;
    let mut bad_entry_count = 0;

    for tsv_reader_result in tsv_reader.deserialize::<TaxonTSVRaw>() {
        if let Ok(taxon_tsv_data_raw) = tsv_reader_result {
            if taxon_tsv_data_raw.dwc_taxon_rank != "species"
                || taxon_tsv_data_raw.dwc_order != MOTH_ORDER
                || taxon_tsv_data_raw.dwc_superfamily == BUTTERFLY_SUPERFAMILY
            {
                continue;
            }
        } else {
            bad_entry_count += 1;
            continue;
        }
        moth_entry_count += 1;
    }

    println!("Found {moth_entry_count} moths");
    println!("Failed to parse {bad_entry_count} entries");
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct TaxonTSVRaw {
    #[serde(rename = "dwc:taxonID")]
    dwc_taxon_id: String,
    #[serde(rename = "dwc:parentNameUsageID")]
    dwc_parent_name_usage_id: String,
    #[serde(rename = "dwc:acceptedNameUsageID")]
    dwc_accepted_name_usage_id: String,
    #[serde(rename = "dwc:originalNameUsageID")]
    dwc_original_name_usage_id: String,
    #[serde(rename = "dwc:scientificNameID")]
    dwc_scientific_name_id: String,
    #[serde(rename = "dwc:datasetID")]
    dwc_dataset_id: String,
    #[serde(rename = "dwc:taxonomicStatus")]
    dwc_taxonomic_status: String,
    #[serde(rename = "dwc:taxonRank")]
    dwc_taxon_rank: String,
    #[serde(rename = "dwc:scientificName")]
    dwc_scientific_name: String,
    #[serde(rename = "dwc:scientificNameAuthorship")]
    dwc_scientific_name_authorship: String,
    #[serde(rename = "col:notho")]
    col_notho: String,
    #[serde(rename = "dwc:genericName")]
    dwc_generic_name: String,
    #[serde(rename = "dwc:infragenericEpithet")]
    dwc_infrageneric_epithet: String,
    #[serde(rename = "dwc:specificEpithet")]
    dwc_specific_epithet: String,
    #[serde(rename = "dwc:infraspecificEpithet")]
    dwc_infraspecific_epithet: String,
    #[serde(rename = "dwc:cultivarEpithet")]
    dwc_cultivar_epithet: String,
    #[serde(rename = "dwc:nameAccordingTo")]
    dwc_name_according_to: String,
    #[serde(rename = "dwc:namePublishedIn")]
    dwc_name_published_in: String,
    #[serde(rename = "dwc:nomenclaturalCode")]
    dwc_nomenclatural_code: String,
    #[serde(rename = "dwc:nomenclaturalStatus")]
    dwc_nomenclatural_status: String,
    #[serde(rename = "dwc:kingdom")]
    dwc_kingdom: String,
    #[serde(rename = "dwc:phylum")]
    dwc_phylum: String,
    #[serde(rename = "dwc:class")]
    dwc_class: String,
    #[serde(rename = "dwc:order")]
    dwc_order: String,
    #[serde(rename = "dwc:superfamily")]
    dwc_superfamily: String,
    #[serde(rename = "dwc:family")]
    dwc_family: String,
    #[serde(rename = "dwc:subfamily")]
    dwc_subfamily: String,
    #[serde(rename = "dwc:tribe")]
    dwc_tribe: String,
    #[serde(rename = "dwc:subtribe")]
    dwc_subtribe: String,
    #[serde(rename = "dwc:genus")]
    dwc_genus: String,
    #[serde(rename = "dwc:subgenus")]
    dwc_subgenus: String,
    #[serde(rename = "dwc:taxonRemarks")]
    dwc_taxon_remarks: String,
    #[serde(rename = "dcterms:references")]
    dcterms_references: String,
    #[serde(rename = "clb:merged")]
    clb_merged: String,
}

struct SpeciesData {
    col_taxon_id: String,
    taxonomic_status: String,
    classification: ScientificClassification,
}

struct ScientificClassification {
    superfamily: String,
    family: String,
    subfamily: String,
    tribe: String,
    subtribe: String,
    genus: String,
    epithet: String,
}
