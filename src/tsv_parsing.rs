use std::{collections::HashMap, fs::File};

use crate::{
    CATALOGUE_OF_LIFE_DATA_DIR, IUCN_DATA_DIR,
    addin_tsv_hashmaps::{self, IUCNData, IUCNDataKey, VernacularHashKey, iucn_hashmaps_combiner},
    tsv_types::*,
};

pub fn parse_tsvs() -> TSVMaps {
    let mut col_vernacular_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(
            File::open(format!("{CATALOGUE_OF_LIFE_DATA_DIR}/VernacularName.tsv")).unwrap(),
        );
    let col_vernacular_tsv = addin_tsv_hashmaps::col_vernacular_to_hashmap(
        col_vernacular_tsv_reader.deserialize::<COLVernacularNameTSVRaw>(),
    );

    let mut col_species_profile_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(
            File::open(format!("{CATALOGUE_OF_LIFE_DATA_DIR}/SpeciesProfile.tsv")).unwrap(),
        );
    let col_species_profile_tsv = addin_tsv_hashmaps::col_species_profile_to_hashmap(
        col_species_profile_tsv_reader.deserialize::<COLSpeciesProfileTSVRaw>(),
    );

    let mut col_distribution_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(File::open(format!("{CATALOGUE_OF_LIFE_DATA_DIR}/Distribution.tsv")).unwrap());
    let col_distribution_tsv = addin_tsv_hashmaps::col_distribution_to_hashmap(
        col_distribution_tsv_reader.deserialize::<COLDistributionTSVRaw>(),
    );

    let mut iucn_taxon_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .has_headers(false)
        .from_reader(File::open(format!("{IUCN_DATA_DIR}/taxon.txt")).unwrap());
    let iucn_taxon_tsv_raw = iucn_taxon_tsv_reader.deserialize::<IUCNTaxonTXTRaw>();

    let mut iucn_distribution_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .has_headers(false)
        .from_reader(File::open(format!("{IUCN_DATA_DIR}/distribution.txt")).unwrap());
    let iucn_distribution_tsv = addin_tsv_hashmaps::iucn_distribution_to_hashmap(
        iucn_distribution_tsv_reader.deserialize::<IUCNDistributionTXTRaw>(),
    );

    let iucn_data = iucn_hashmaps_combiner(iucn_taxon_tsv_raw, iucn_distribution_tsv);

    return TSVMaps {
        col_tsvs: CatalogueOfLifeTSVMaps {
            vernacular_name: col_vernacular_tsv,
            species_profile: col_species_profile_tsv,
            distribution: col_distribution_tsv,
        },
        iucn_data: iucn_data,
    };
}

pub struct TSVMaps {
    pub col_tsvs: CatalogueOfLifeTSVMaps,
    pub iucn_data: HashMap<IUCNDataKey, IUCNData>,
}

pub struct CatalogueOfLifeTSVMaps {
    pub vernacular_name: HashMap<VernacularHashKey, Vec<String>>,
    pub species_profile: HashMap<String, COLSpeciesProfileTSVRaw>,
    pub distribution: HashMap<String, COLDistributionTSVRaw>,
}
