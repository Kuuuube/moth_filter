use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

const MOTH_ORDER: &str = "Lepidoptera";
const BUTTERFLY_SUPERFAMILY: &str = "Papilionoidea";

fn main() {
    let taxon_tsv_file = File::open("./data/Taxon.tsv").unwrap();

    for taxon_tsv_raw_data in TaxonTSV::new(taxon_tsv_file) {
        if taxon_tsv_raw_data.dwc_taxon_rank != "species"
            || taxon_tsv_raw_data.dwc_order != MOTH_ORDER
            || taxon_tsv_raw_data.dwc_superfamily == BUTTERFLY_SUPERFAMILY
        {
            continue;
        }
    }
}

struct TaxonTSV {
    tsv_lines: std::io::Lines<BufReader<File>>,
}

impl TaxonTSV {
    fn new(file: File) -> Self {
        TaxonTSV {
            tsv_lines: BufReader::new(file).lines(),
        }
    }
}

impl Iterator for TaxonTSV {
    type Item = TaxonTSVRaw;

    fn next(&mut self) -> Option<Self::Item> {
        let tsv_line = self.tsv_lines.next()?.ok()?;
        let split_tsv_line: Vec<_> = tsv_line.split("\t").collect();
        Some(TaxonTSVRaw {
            dwc_taxon_id: split_tsv_line.get(0)?.to_string(),
            dwc_parent_name_usage_id: split_tsv_line.get(1)?.to_string(),
            dwc_accepted_name_usage_id: split_tsv_line.get(2)?.to_string(),
            dwc_original_name_usage_id: split_tsv_line.get(3)?.to_string(),
            dwc_scientific_name_id: split_tsv_line.get(4)?.to_string(),
            dwc_dataset_id: split_tsv_line.get(5)?.to_string(),
            dwc_taxonomic_status: split_tsv_line.get(6)?.to_string(),
            dwc_taxon_rank: split_tsv_line.get(7)?.to_string(),
            dwc_scientific_name: split_tsv_line.get(8)?.to_string(),
            dwc_scientific_name_authorship: split_tsv_line.get(9)?.to_string(),
            col_notho: split_tsv_line.get(10)?.to_string(),
            dwc_generic_name: split_tsv_line.get(11)?.to_string(),
            dwc_infrageneric_epithet: split_tsv_line.get(12)?.to_string(),
            dwc_specific_epithet: split_tsv_line.get(13)?.to_string(),
            dwc_infraspecific_epithet: split_tsv_line.get(14)?.to_string(),
            dwc_cultivar_epithet: split_tsv_line.get(15)?.to_string(),
            dwc_name_according_to: split_tsv_line.get(16)?.to_string(),
            dwc_name_published_in: split_tsv_line.get(17)?.to_string(),
            dwc_nomenclatural_code: split_tsv_line.get(18)?.to_string(),
            dwc_nomenclatural_status: split_tsv_line.get(19)?.to_string(),
            dwc_kingdom: split_tsv_line.get(20)?.to_string(),
            dwc_phylum: split_tsv_line.get(21)?.to_string(),
            dwc_class: split_tsv_line.get(22)?.to_string(),
            dwc_order: split_tsv_line.get(23)?.to_string(),
            dwc_superfamily: split_tsv_line.get(24)?.to_string(),
            dwc_family: split_tsv_line.get(25)?.to_string(),
            dwc_subfamily: split_tsv_line.get(26)?.to_string(),
            dwc_tribe: split_tsv_line.get(27)?.to_string(),
            dwc_subtribe: split_tsv_line.get(28)?.to_string(),
            dwc_genus: split_tsv_line.get(29)?.to_string(),
            dwc_subgenus: split_tsv_line.get(30)?.to_string(),
            dwc_taxon_remarks: split_tsv_line.get(31)?.to_string(),
            dcterms_references: split_tsv_line.get(32)?.to_string(),
            clb_merged: split_tsv_line.get(33)?.to_string(),
        })
    }
}

#[allow(unused)]
struct TaxonTSVRaw {
    dwc_taxon_id: String,
    dwc_parent_name_usage_id: String,
    dwc_accepted_name_usage_id: String,
    dwc_original_name_usage_id: String,
    dwc_scientific_name_id: String,
    dwc_dataset_id: String,
    dwc_taxonomic_status: String,
    dwc_taxon_rank: String,
    dwc_scientific_name: String,
    dwc_scientific_name_authorship: String,
    col_notho: String,
    dwc_generic_name: String,
    dwc_infrageneric_epithet: String,
    dwc_specific_epithet: String,
    dwc_infraspecific_epithet: String,
    dwc_cultivar_epithet: String,
    dwc_name_according_to: String,
    dwc_name_published_in: String,
    dwc_nomenclatural_code: String,
    dwc_nomenclatural_status: String,
    dwc_kingdom: String,
    dwc_phylum: String,
    dwc_class: String,
    dwc_order: String,
    dwc_superfamily: String,
    dwc_family: String,
    dwc_subfamily: String,
    dwc_tribe: String,
    dwc_subtribe: String,
    dwc_genus: String,
    dwc_subgenus: String,
    dwc_taxon_remarks: String,
    dcterms_references: String,
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
