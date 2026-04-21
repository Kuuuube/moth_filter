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
    for tsv_reader_result in tsv_iter {
        let Ok(ok) = tsv_reader_result else {
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
    return hashmap;
}

pub fn col_species_profile_to_hashmap(
    tsv_iter: csv::DeserializeRecordsIter<'_, File, COLSpeciesProfileTSVRaw>,
) -> HashMap<std::string::String, COLSpeciesProfileTSVRaw> {
    let mut hashmap: HashMap<String, COLSpeciesProfileTSVRaw> = HashMap::new();
    for tsv_reader_result in tsv_iter {
        let Ok(ok) = tsv_reader_result else {
            continue;
        };
        hashmap.insert(ok.dwc_taxon_id.clone(), ok);
    }
    return hashmap;
}

pub fn col_distribution_to_hashmap(
    tsv_iter: csv::DeserializeRecordsIter<'_, File, COLDistributionTSVRaw>,
) -> HashMap<std::string::String, COLDistributionTSVRaw> {
    let mut hashmap: HashMap<String, COLDistributionTSVRaw> = HashMap::new();
    for tsv_reader_result in tsv_iter {
        let Ok(ok) = tsv_reader_result else {
            continue;
        };
        hashmap.insert(ok.dwc_taxon_id.clone(), ok);
    }
    return hashmap;
}
