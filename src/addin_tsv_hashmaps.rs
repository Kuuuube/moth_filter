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

pub fn iucn_taxon_to_hashmap_id_key(
    tsv_iter: csv::DeserializeRecordsIter<'_, File, IUCNTaxonTXTRaw>,
) -> HashMap<String, String> {
    let mut hashmap: HashMap<String, String> = HashMap::new();
    let mut errors = 0;
    for tsv_reader_result in tsv_iter {
        let Ok(ok) = tsv_reader_result else {
            errors += 1;
            continue;
        };
        let Some(base_scientific_name) = iucn_scientific_name_to_base(&ok.scientific_name) else {
            println!("IUCN bad scientific name");
            continue;
        };
        hashmap.insert(ok.id.clone(), base_scientific_name);
    }
    if errors > 0 {
        println!("iucn_taxon_to_hashmap_id_key bad rows: {errors}");
    }
    return hashmap;
}

pub fn iucn_taxon_to_hashmap(
    tsv_iter: csv::DeserializeRecordsIter<'_, File, IUCNTaxonTXTRaw>,
) -> HashMap<String, IUCNTaxonTXTRaw> {
    let mut hashmap: HashMap<String, IUCNTaxonTXTRaw> = HashMap::new();
    let mut errors = 0;
    for tsv_reader_result in tsv_iter {
        let Ok(ok) = tsv_reader_result else {
            errors += 1;
            continue;
        };
        let Some(base_scientific_name) = iucn_scientific_name_to_base(&ok.scientific_name) else {
            println!("IUCN bad scientific name");
            continue;
        };
        hashmap.insert(base_scientific_name, ok);
    }
    if errors > 0 {
        println!("iucn_taxon_to_hashmap bad rows: {errors}");
    }
    return hashmap;
}

// pub fn iucn_vernacular_to_hashmap(
//     tsv_iter: csv::DeserializeRecordsIter<'_, File, IUCNVernacularTXTRaw>,
// ) -> HashMap<String, IUCNVernacularTXTRaw> {
//     let mut hashmap: HashMap<String, IUCNVernacularTXTRaw> = HashMap::new();
//     let mut errors = 0;
//     for tsv_reader_result in tsv_iter {
//         let Ok(ok) = tsv_reader_result else {
//             errors += 1;
//             continue;
//         };
//         hashmap.insert(ok.id.clone(), ok);
//     }
//     if errors > 0 {
//         println!("iucn_vernacular_to_hashmap bad rows: {errors}");
//     }
//     return hashmap;
// }

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

fn iucn_scientific_name_to_base(scientific_name: &str) -> Option<String> {
    let mut scientific_name_split = scientific_name.split_ascii_whitespace();
    let genus = scientific_name_split.next();
    let Some(genus) = genus else {
        println!("Bad IUCN genus");
        return None;
    };
    let specific_epithet = scientific_name_split.next();
    let Some(specific_epithet) = specific_epithet else {
        println!("Bad IUCN specific_epithet");
        return None;
    };
    return Some(format!("{} {}", genus, specific_epithet));
}
