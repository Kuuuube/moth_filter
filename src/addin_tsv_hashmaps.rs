use std::{collections::HashMap, fs::File};

use crate::{MOTH_ORDER, tsv_types::*};

#[derive(Eq, Hash, PartialEq)]
pub struct VernacularHashKey {
    pub language_code: String,
    pub taxon_id: String,
}
pub type VernacularCommonName = Vec<String>;

pub fn col_vernacular_to_hashmap(
    tsv_iter: csv::DeserializeRecordsIter<'_, File, COLVernacularNameTSVRaw>,
) -> HashMap<VernacularHashKey, VernacularCommonName> {
    let mut hashmap: HashMap<VernacularHashKey, VernacularCommonName> = HashMap::new();
    let mut errors = 0;
    for tsv_reader_result in tsv_iter {
        let Ok(ok) = tsv_reader_result else {
            errors += 1;
            continue;
        };
        let key = VernacularHashKey {
            language_code: ok.dcterms_language,
            taxon_id: ok.dwc_taxon_id,
        };
        hashmap
            .entry(key)
            .and_modify(|x| x.push(ok.dwc_vernacular_name.clone()))
            .or_insert(vec![ok.dwc_vernacular_name]);
    }
    if errors > 0 {
        println!("col_vernacular_to_hashmap bad rows: {errors}");
    }
    hashmap
}

pub fn col_species_profile_to_hashmap(
    tsv_iter: csv::DeserializeRecordsIter<'_, File, COLSpeciesProfileTSVRaw>,
) -> HashMap<String, COLSpeciesProfileTSVRaw> {
    let mut hashmap: HashMap<String, COLSpeciesProfileTSVRaw> = HashMap::new();
    let mut errors = 0;
    for tsv_reader_result in tsv_iter {
        let Ok(ok) = tsv_reader_result else {
            errors += 1;
            continue;
        };
        hashmap.insert(ok.dwc_taxon_id.clone(), ok);
    }
    if errors > 0 {
        println!("col_species_profile_to_hashmap bad rows: {errors}");
    }
    hashmap
}

pub fn col_distribution_to_hashmap(
    tsv_iter: csv::DeserializeRecordsIter<'_, File, COLDistributionTSVRaw>,
) -> HashMap<String, COLDistributionTSVRaw> {
    let mut hashmap: HashMap<String, COLDistributionTSVRaw> = HashMap::new();
    let mut errors = 0;
    for tsv_reader_result in tsv_iter {
        let Ok(ok) = tsv_reader_result else {
            errors += 1;
            continue;
        };
        hashmap.insert(ok.dwc_taxon_id.clone(), ok);
    }
    if errors > 0 {
        println!("col_distribution_to_hashmap bad rows: {errors}");
    }
    hashmap
}

pub fn iucn_distribution_to_hashmap(
    tsv_iter: csv::DeserializeRecordsIter<'_, File, IUCNDistributionTXTRaw>,
) -> HashMap<String, IUCNDistributionTXTRaw> {
    let mut hashmap: HashMap<String, IUCNDistributionTXTRaw> = HashMap::new();
    let mut errors = 0;
    for tsv_reader_result in tsv_iter {
        let Ok(ok) = tsv_reader_result else {
            errors += 1;
            continue;
        };
        hashmap.insert(ok.id.clone(), ok);
    }
    if errors > 0 {
        println!("iucn_distribution_to_hashmap bad rows: {errors}");
    }
    hashmap
}

pub fn iucn_hashmaps_combiner(
    taxon_tsv: csv::DeserializeRecordsIter<'_, File, IUCNTaxonTXTRaw>,
    /* vernacular: HashMap<String, IUCNVernacularTXTRaw>, */
    distribution: HashMap<String, IUCNDistributionTXTRaw>,
) -> HashMap<AddinDataKey, IUCNData> {
    let mut hashmap: HashMap<AddinDataKey, IUCNData> = HashMap::new();
    let mut errors = 0;

    for taxon_entry in taxon_tsv {
        let Ok(taxon_entry) = taxon_entry else {
            errors += 1;
            continue;
        };

        if let Some(taxon_rank) = taxon_entry.taxon_rank
            && (taxon_rank == TaxonRank::Species || taxon_rank == TaxonRank::SubSpecies)
        {
            if let Some(order) = taxon_entry.order
                && order != MOTH_ORDER
            {
                continue;
            }

            let genus = taxon_entry
                .genus
                .expect("IUCN species or subspecies found with no genus");

            let specific = taxon_entry
                .specific_epithet
                .expect("IUCN species or subspecies found with no specific name");

            let subspecific = (taxon_rank == TaxonRank::SubSpecies).then(|| {
                taxon_entry
                    .infraspecific_epithet
                    .expect("IUCN subspecies found with no infraspecific name")
            });

            let distribution_data = distribution
                .get(&taxon_entry.id)
                .expect("IUCN species or subspecies found with no distribution data");

            let iucn_data = IUCNData {
                locality: distribution_data.locality.clone(),
                references: taxon_entry.references,
                threat_status: distribution_data.threat_status.clone(),
            };

            hashmap.insert(
                AddinDataKey {
                    genus: genus.to_lowercase(),
                    specific: specific.to_lowercase(),
                    subspecific: subspecific.map(|x| x.to_lowercase()),
                },
                iucn_data,
            );
        }
    }
    if errors > 0 {
        println!("iucn_hashmaps_combiner bad rows: {errors}");
    }
    hashmap
}

#[derive(Debug)]
pub struct IUCNData {
    pub locality: String,
    pub references: String,
    pub threat_status: IUCNThreatStatusRaw,
}

#[allow(unused)]
pub fn worms_species_profile_to_hashmap(
    tsv_iter: csv::DeserializeRecordsIter<'_, File, WORMSSpeciesProfileTXTRaw>,
) -> HashMap<String, WORMSSpeciesProfileTXTRaw> {
    let mut hashmap: HashMap<String, WORMSSpeciesProfileTXTRaw> = HashMap::new();
    let mut errors = 0;
    for tsv_reader_result in tsv_iter {
        let Ok(ok) = tsv_reader_result else {
            errors += 1;
            continue;
        };
        hashmap.insert(ok.taxon_id.clone(), ok);
    }
    if errors > 0 {
        println!("worms_species_profile_to_hashmap bad rows: {errors}");
    }
    hashmap
}

#[allow(unused)]
pub fn worms_hashmaps_combiner(
    taxon_tsv: csv::DeserializeRecordsIter<'_, File, WORMSTaxonTXTRaw>,
    species_profiles: HashMap<String, WORMSSpeciesProfileTXTRaw>,
) -> HashMap<AddinDataKey, WORMSData> {
    let mut hashmap: HashMap<AddinDataKey, WORMSData> = HashMap::new();
    let mut errors = 0;

    for taxon_entry in taxon_tsv {
        let Ok(taxon_entry) = taxon_entry else {
            errors += 1;
            continue;
        };

        if let Some(taxon_rank) = taxon_entry.taxon_rank
            && (taxon_rank == TaxonRank::Species || taxon_rank == TaxonRank::SubSpecies)
        {
            if let Some(order) = taxon_entry.order
                && order != MOTH_ORDER
            {
                continue;
            }

            let Some(genus) = taxon_entry.genus else {
                // in WoRMS, theres some viruses that dont have a genus but are a species or subspecies
                continue;
            };

            let specific = taxon_entry
                .specific_epithet
                .expect("WORMS species or subspecies found with no specific name");

            let subspecific = (taxon_rank == TaxonRank::SubSpecies).then(|| {
                taxon_entry
                    .infraspecific_epithet
                    .expect("WORMS subspecies found with no infraspecific name")
            });

            let species_profile_data = species_profiles
                .get(&taxon_entry.taxon_id)
                .expect("WORMS species or subspecies found with no distribution data");

            let worms_data = WORMSData {
                is_marine: species_profile_data.is_marine,
                is_freshwater: species_profile_data.is_freshwater,
                // is_terrestrial: species_profile_data.is_terrestrial,
                // is_extinct: species_profile_data.is_extinct,
                is_brackish: species_profile_data.is_brackish,
            };

            hashmap.insert(
                AddinDataKey {
                    genus: genus.to_lowercase(),
                    specific: specific.to_lowercase(),
                    subspecific: subspecific.map(|x| x.to_lowercase()),
                },
                worms_data,
            );
        }
    }
    if errors > 0 {
        println!("worms_hashmaps_combiner bad rows: {errors}");
    }
    hashmap
}

#[derive(Debug)]
#[allow(unused)]
pub struct WORMSData {
    pub is_marine: Option<bool>,
    pub is_freshwater: Option<bool>,
    // pub is_terrestrial: Option<bool>,
    // pub is_extinct: Option<bool>,
    pub is_brackish: Option<bool>,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct AddinDataKey {
    pub genus: String,
    pub specific: String,
    pub subspecific: Option<String>,
}
