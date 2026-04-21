use std::{collections::HashMap, fs::File};

use crate::{
    CATALOGUE_OF_LIFE_DATA_DIR,
    addin_tsv_hashmaps::{self, VernacularHashKey},
    tsv_types::*,
};

pub fn parse_tsvs() -> TSVMaps {
    let mut col_vernacular_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(
            File::open(format!("{CATALOGUE_OF_LIFE_DATA_DIR}/VernacularName.tsv")).unwrap(),
        );
    let col_vernacular_tsv = addin_tsv_hashmaps::vernacular_to_hashmap(
        col_vernacular_tsv_reader.deserialize::<VernacularNameTSVRaw>(),
    );

    let mut col_species_profile_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(
            File::open(format!("{CATALOGUE_OF_LIFE_DATA_DIR}/SpeciesProfile.tsv")).unwrap(),
        );
    let col_species_profile_tsv = addin_tsv_hashmaps::species_profile_to_hashmap(
        col_species_profile_tsv_reader.deserialize::<SpeciesProfileTSVRaw>(),
    );

    let mut col_distribution_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(File::open(format!("{CATALOGUE_OF_LIFE_DATA_DIR}/Distribution.tsv")).unwrap());
    let col_distribution_tsv = addin_tsv_hashmaps::distribution_to_hashmap(
        col_distribution_tsv_reader.deserialize::<DistributionTSVRaw>(),
    );

    return TSVMaps {
        col_tsvs: CatalogueOfLifeTSVMaps {
            vernacular_name: col_vernacular_tsv,
            species_profile: col_species_profile_tsv,
            distribution: col_distribution_tsv,
        },
    };
}

pub struct TSVMaps {
    pub col_tsvs: CatalogueOfLifeTSVMaps,
}

pub struct CatalogueOfLifeTSVMaps {
    pub vernacular_name: HashMap<VernacularHashKey, Vec<String>>,
    pub species_profile: HashMap<String, SpeciesProfileTSVRaw>,
    pub distribution: HashMap<String, DistributionTSVRaw>,
}