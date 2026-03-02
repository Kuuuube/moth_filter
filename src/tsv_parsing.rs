use std::{collections::HashMap, fs::File};

use crate::{
    addin_tsv_hashmaps::{self, VernacularHashKey},
    tsv_types::*,
};

pub fn parse_tsvs() -> TSVMaps {
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

    return TSVMaps {
        vernacular_name: vernacular_tsv,
        species_profile: species_profile_tsv,
        distribution: distribution_tsv,
    };
}

pub struct TSVMaps {
    pub vernacular_name: HashMap<VernacularHashKey, Vec<String>>,
    pub species_profile: HashMap<String, SpeciesProfileTSVRaw>,
    pub distribution: HashMap<String, DistributionTSVRaw>,
}
