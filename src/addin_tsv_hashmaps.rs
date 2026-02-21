use std::{collections::HashMap, fs::File};

use crate::tsv_structs::*;

pub fn vernacular_to_hashmap(
    tsv_iter: csv::DeserializeRecordsIter<'_, File, VernacularNameTSVRaw>,
) -> HashMap<String, VernacularNameTSVRaw> {
    let mut hashmap: HashMap<String, VernacularNameTSVRaw> = HashMap::new();
    for tsv_reader_result in tsv_iter {
        let Ok(ok) = tsv_reader_result else {
            continue;
        };
        // currently only accept english names
        if ok.dcterms_language != "eng" {
            continue;
        }
        hashmap.insert(ok.dwc_taxon_id.clone(), ok);
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
