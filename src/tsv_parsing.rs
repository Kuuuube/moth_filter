use std::{collections::HashMap, fs::File};

use crate::{
    CATALOGUE_OF_LIFE_DATA_DIR, IUCN_DATA_DIR,
    addin_tsv_hashmaps::{
        self, AddinDataKey, IUCNData, VernacularHashKey, iucn_hashmaps_combiner,
    },
    tsv_types::*,
};

pub fn parse_tsvs() -> TSVMaps {
    println!("Parsing Catalogue of Life vernacular");
    let mut col_vernacular_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(
            File::open(format!("{CATALOGUE_OF_LIFE_DATA_DIR}/VernacularName.tsv")).unwrap(),
        );
    let col_vernacular_tsv = addin_tsv_hashmaps::col_vernacular_to_hashmap(
        col_vernacular_tsv_reader.deserialize::<COLVernacularNameTSVRaw>(),
    );

    println!("Parsing Catalogue of Life species profiles");
    let mut col_species_profile_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(
            File::open(format!("{CATALOGUE_OF_LIFE_DATA_DIR}/SpeciesProfile.tsv")).unwrap(),
        );
    let col_species_profile_tsv = addin_tsv_hashmaps::col_species_profile_to_hashmap(
        col_species_profile_tsv_reader.deserialize::<COLSpeciesProfileTSVRaw>(),
    );

    println!("Parsing Catalogue of Life distribution");
    let mut col_distribution_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(File::open(format!("{CATALOGUE_OF_LIFE_DATA_DIR}/Distribution.tsv")).unwrap());
    let col_distribution_tsv = addin_tsv_hashmaps::col_distribution_to_hashmap(
        col_distribution_tsv_reader.deserialize::<COLDistributionTSVRaw>(),
    );

    println!("Parsing IUCN Redlist taxon");
    let mut iucn_taxon_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .has_headers(false)
        .from_reader(File::open(format!("{IUCN_DATA_DIR}/taxon.txt")).unwrap());
    let iucn_taxon_tsv_raw = iucn_taxon_tsv_reader.deserialize::<IUCNTaxonTXTRaw>();

    println!("Parsing IUCN Redlist distribution");
    let mut iucn_distribution_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .has_headers(false)
        .from_reader(File::open(format!("{IUCN_DATA_DIR}/distribution.txt")).unwrap());
    let iucn_distribution_tsv = addin_tsv_hashmaps::iucn_distribution_to_hashmap(
        iucn_distribution_tsv_reader.deserialize::<IUCNDistributionTXTRaw>(),
    );

    println!("Combining IUCN Redlist data");
    let iucn_data = iucn_hashmaps_combiner(iucn_taxon_tsv_raw, iucn_distribution_tsv);

    // println!("Parsing WoRMS taxon");
    // let mut worms_taxon_tsv_reader = csv::ReaderBuilder::new()
    //     .delimiter(b'\t')
    //     .quoting(false)
    //     .from_reader(File::open(format!("{WORMS_DATA_DIR}/taxon.txt")).unwrap());
    // let worms_taxon_tsv_raw = worms_taxon_tsv_reader.deserialize::<WORMSTaxonTXTRaw>();

    // println!("Parsing WoRMS species profiles");
    // let mut worms_species_profile_tsv_reader = csv::ReaderBuilder::new()
    //     .delimiter(b'\t')
    //     .quoting(false)
    //     .from_reader(File::open(format!("{WORMS_DATA_DIR}/speciesprofile.txt")).unwrap());
    // let worms_species_profile_tsv = addin_tsv_hashmaps::worms_species_profile_to_hashmap(
    //     worms_species_profile_tsv_reader.deserialize::<WORMSSpeciesProfileTXTRaw>(),
    // );

    // println!("Combining WoRMS data");
    // let worms_data = worms_hashmaps_combiner(worms_taxon_tsv_raw, worms_species_profile_tsv);

    TSVMaps {
        col_tsvs: CatalogueOfLifeTSVMaps {
            vernacular_name: col_vernacular_tsv,
            species_profile: col_species_profile_tsv,
            distribution: col_distribution_tsv,
        },
        iucn_data,
        // worms_data,
    }
}

pub struct TSVMaps {
    pub col_tsvs: CatalogueOfLifeTSVMaps,
    pub iucn_data: HashMap<AddinDataKey, IUCNData>,
    // pub worms_data: HashMap<AddinDataKey, WORMSData>,
}

pub struct CatalogueOfLifeTSVMaps {
    pub vernacular_name: HashMap<VernacularHashKey, Vec<String>>,
    pub species_profile: HashMap<String, COLSpeciesProfileTSVRaw>,
    pub distribution: HashMap<String, COLDistributionTSVRaw>,
}
