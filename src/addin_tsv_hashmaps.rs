use std::{collections::HashMap, fs::File};

use crate::tsv_types::*;

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
    return hashmap;
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
    return hashmap;
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
    return hashmap;
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
    return hashmap;
}

pub fn iucn_hashmaps_combiner(
    taxon_tsv: csv::DeserializeRecordsIter<'_, File, IUCNTaxonTXTRaw>,
    /* vernacular: HashMap<String, IUCNVernacularTXTRaw>, */
    distribution: HashMap<String, IUCNDistributionTXTRaw>,
) -> HashMap<IUCNDataKey, IUCNData> {
    let mut hashmap: HashMap<IUCNDataKey, IUCNData> = HashMap::new();
    let mut errors = 0;

    for taxon_entry in taxon_tsv {
        let Ok(taxon_entry) = taxon_entry else {
            errors += 1;
            continue;
        };

        if let Some(taxon_rank) = taxon_entry.taxon_rank
            && (taxon_rank == TaxonRank::Species || taxon_rank == TaxonRank::SubSpecies)
        {
            let Some(genus) = taxon_entry.genus else {
                errors += 1;
                continue;
            };
            let Some(specific) = taxon_entry.specific_epithet else {
                errors += 1;
                continue;
            };

            let distribution_data = distribution
                .get(&taxon_entry.id)
                .expect("IUCN species or subspecies found with no distribution data");

            let iucn_data = IUCNData {
                references: taxon_entry.references,
                threat_status: distribution_data.threat_status.clone(),
            };

            hashmap.insert(
                IUCNDataKey {
                    genus,
                    specific,
                    subspecific: taxon_entry.infraspecific_epithet,
                },
                iucn_data,
            );
        }
    }
    if errors > 0 {
        println!("iucn_hashmaps_combiner bad rows: {errors}");
    }
    return hashmap;
}

#[derive(Debug)]
pub struct IUCNData {
    pub references: String,
    pub threat_status: IUCNThreatStatusRaw,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct IUCNDataKey {
    pub genus: String,
    pub specific: String,
    pub subspecific: Option<String>,
}
