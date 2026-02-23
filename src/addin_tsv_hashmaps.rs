use std::{collections::HashMap, fs::File};

use crate::tsv_types::*;

#[derive(Eq, Hash, PartialEq)]
pub struct VernacularHashKey {
    pub language_code: String,
    pub taxon_id: String,
}
pub type VernacularCommonName = Vec<String>;

pub fn vernacular_to_hashmap(
    tsv_iter: csv::DeserializeRecordsIter<'_, File, VernacularNameTSVRaw>,
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

pub fn species_profile_to_hashmap(
    tsv_iter: csv::DeserializeRecordsIter<'_, File, SpeciesProfileTSVRaw>,
) -> HashMap<std::string::String, SpeciesProfileTSVRaw> {
    let mut hashmap: HashMap<String, SpeciesProfileTSVRaw> = HashMap::new();
    for tsv_reader_result in tsv_iter {
        let Ok(ok) = tsv_reader_result else {
            continue;
        };
        hashmap.insert(ok.dwc_taxon_id.clone(), ok);
    }
    return hashmap;
}

pub fn distribution_to_hashmap(
    tsv_iter: csv::DeserializeRecordsIter<'_, File, DistributionTSVRaw>,
) -> HashMap<std::string::String, DistributionTSVRaw> {
    let mut hashmap: HashMap<String, DistributionTSVRaw> = HashMap::new();
    for tsv_reader_result in tsv_iter {
        let Ok(ok) = tsv_reader_result else {
            continue;
        };
        hashmap.insert(ok.dwc_taxon_id.clone(), ok);
    }
    return hashmap;
}
