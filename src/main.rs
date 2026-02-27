use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{Read, Write},
    time::Instant,
};

use crate::{addin_tsv_hashmaps::VernacularHashKey, json_types::*, tsv_types::*};

mod addin_tsv_hashmaps;
mod json_types;
mod tsv_types;

const MOTH_ORDER: &str = "Lepidoptera";
const BUTTERFLY_SUPERFAMILY: &str = "Papilionoidea";

fn main() {
    let start_time = Instant::now();

    let mut taxon_tsv_reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .quoting(false)
        .from_reader(File::open("./data/Taxon.tsv").unwrap());
    let taxon_tsv = taxon_tsv_reader.deserialize::<TaxonTSVRaw>();

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

    let moth_output_file_path = "./output/moth_data.json";
    let moth_output_file_path_zstd = moth_output_file_path.to_owned() + ".zst";
    let moth_output_file = File::create(moth_output_file_path).unwrap();
    let moth_output_file_zstd = File::create(&moth_output_file_path_zstd).unwrap();

    let butterfly_output_file_path = "./output/butterfly_blacklist.json";
    let butterfly_output_file_path_zstd = butterfly_output_file_path.to_owned() + ".zst";
    let butterfly_output_file = File::create(butterfly_output_file_path).unwrap();
    let butterfly_output_file_zstd = File::create(&butterfly_output_file_path_zstd).unwrap();

    let butterfly_collisions_output_file_path = "./output/butterfly_blacklist_collisions.json";
    let butterfly_collisions_output_file = File::create(butterfly_collisions_output_file_path).unwrap();

    let mut bad_entry_count = 0;
    let mut moth_entries: Vec<SpeciesData> = Vec::new();
    let mut synonyms: HashMap<String, Vec<SynonymSpecies>> = HashMap::new();
    let mut moth_ids: HashSet<String> = HashSet::new();
    let mut butterfly_data: ButterflyBlacklist = Default::default();

    for tsv_reader_result in taxon_tsv {
        let Ok(taxon_tsv_data_raw) = tsv_reader_result else {
            bad_entry_count += 1;
            continue;
        };

        // filter out not species before checking for synonyms
        if taxon_tsv_data_raw.dwc_taxon_rank != "species" {
            continue;
        }

        // synonyms have nearly no data and will never be detected as a moth, run before moth check and filter out non moths later
        match taxon_tsv_data_raw.dwc_taxonomic_status {
            TaxonomicStatusRaw::Synonym | TaxonomicStatusRaw::AmbiguousSynonym => {
                let primary_taxon_id = taxon_tsv_data_raw.dwc_accepted_name_usage_id;
                if let Some(genus) = taxon_tsv_data_raw.dwc_generic_name
                    && let Some(epithet) = taxon_tsv_data_raw.dwc_specific_epithet
                {
                    let synonym = SynonymSpecies {
                        catalogue_of_life_taxon_id: taxon_tsv_data_raw.dwc_taxon_id,
                        genus: genus,
                        epithet: epithet,
                    };
                    synonyms
                        .entry(primary_taxon_id)
                        .and_modify(|x| {
                            x.push(synonym.clone());
                        })
                        .or_insert(vec![synonym]);
                }
                continue;
            }
            TaxonomicStatusRaw::Misapplied => {
                continue;
            }
            _ => (),
        };

        // continue if not a moth
        let Some(order) = &taxon_tsv_data_raw.dwc_order else {
            continue;
        };
        if order != MOTH_ORDER {
            continue;
        }
        if let Some(superfamily) = &taxon_tsv_data_raw.dwc_superfamily
            && superfamily == BUTTERFLY_SUPERFAMILY
        {
            if let Some(family) = taxon_tsv_data_raw.dwc_family {
                butterfly_data.families.insert(family.to_lowercase());
            }
            if let Some(subfamily) = taxon_tsv_data_raw.dwc_subfamily {
                butterfly_data.subfamilies.insert(subfamily.to_lowercase());
            }
            if let Some(tribe) = taxon_tsv_data_raw.dwc_tribe {
                butterfly_data.tribes.insert(tribe.to_lowercase());
            }
            if let Some(subtribe) = taxon_tsv_data_raw.dwc_subtribe {
                butterfly_data.subtribes.insert(subtribe.to_lowercase());
            }
            if let Some(genus) = taxon_tsv_data_raw.dwc_genus {
                butterfly_data.genera.insert(genus.to_lowercase());
            }
            if let Some(genus) = taxon_tsv_data_raw.dwc_generic_name {
                butterfly_data.genera.insert(genus.to_lowercase());
            }
            if let Some(epithet) = taxon_tsv_data_raw.dwc_specific_epithet {
                butterfly_data.epithets.insert(epithet.to_lowercase());
            }
            continue;
        }

        moth_ids.insert(taxon_tsv_data_raw.dwc_taxon_id.clone());

        let common_name = vernacular_tsv.get(&VernacularHashKey {
            language_code: "eng".to_string(),
            taxon_id: taxon_tsv_data_raw.dwc_taxon_id.clone(),
        });
        let species_profile = species_profile_tsv
            .get(&taxon_tsv_data_raw.dwc_taxon_id)
            .and_then(|x| {
                Some(SpeciesProfile {
                    extinct: x.gbif_is_extinct,
                    freshwater: x.gbif_is_freshwater,
                    marine: x.gbif_is_marine,
                })
            });

        let distribution = distribution_tsv
            .get(&taxon_tsv_data_raw.dwc_taxon_id)
            .and_then(|x| {
                let threat_status = x.iucn_threat_status.as_ref().and_then(|x| match x {
                    ThreatStatusRaw::LeastConcern => Some(ThreatStatus::LeastConcern),
                    ThreatStatusRaw::Vulnerable => Some(ThreatStatus::Vulnerable),
                    ThreatStatusRaw::Endangered => Some(ThreatStatus::Endangered),
                    ThreatStatusRaw::CriticallyEndangered => {
                        Some(ThreatStatus::CriticallyEndangered)
                    }
                    ThreatStatusRaw::ExtinctInTheWild => Some(ThreatStatus::ExtinctInTheWild),
                    ThreatStatusRaw::Extinct => Some(ThreatStatus::Extinct),
                    ThreatStatusRaw::NotEvaluated => None,
                    ThreatStatusRaw::DataDeficient => None,
                });
                if x.dwc_locality.is_none() && threat_status.is_none() {
                    return None;
                }
                Some(Distribution {
                    locality: x.dwc_locality.clone(),
                    threat_status,
                })
            });

        // some malformed entries dont have a `genus` but have a `generic name` which is synonymous
        let genus_fixed = match taxon_tsv_data_raw.dwc_genus {
            Some(some) => some,
            None => match taxon_tsv_data_raw.dwc_generic_name {
                Some(some) => some,
                None => {
                    bad_entry_count += 1;
                    continue;
                }
            },
        };

        let Some(epithet_checked) = taxon_tsv_data_raw.dwc_specific_epithet else {
            bad_entry_count += 1;
            continue;
        };

        moth_entries.push(SpeciesData {
            catalogue_of_life_taxon_id: taxon_tsv_data_raw.dwc_taxon_id,
            classification: ScientificClassification {
                superfamily: taxon_tsv_data_raw.dwc_superfamily,
                family: taxon_tsv_data_raw.dwc_family,
                subfamily: taxon_tsv_data_raw.dwc_subfamily,
                tribe: taxon_tsv_data_raw.dwc_tribe,
                subtribe: taxon_tsv_data_raw.dwc_subtribe,
                genus: genus_fixed,
                epithet: epithet_checked,
            },
            common_names: common_name.cloned(),
            species_profile: species_profile,
            distribution: distribution,
            synonyms: None,
            published_in: taxon_tsv_data_raw.dwc_name_published_in,
        });
    }

    synonyms.retain(|key, _value| moth_ids.contains(key));
    let moth_synonyms_count: usize = synonyms.iter().map(|x| x.1.len()).sum();

    let mut butterfly_collision_data: ButterflyBlacklist = Default::default();

    for moth_entry in moth_entries.iter_mut() {
        // eliminate any false positives in butterfly blacklist
        // only genera and epithets appear to collide but check over all of them anyways
        if let Some(family) = &moth_entry.classification.family {
            if butterfly_data.families.remove(&family.to_lowercase()) {
                butterfly_collision_data.families.insert(family.clone());
            }
        }
        if let Some(subfamily) = &moth_entry.classification.subfamily {
            if butterfly_data.subfamilies.remove(&subfamily.to_lowercase()) {
                butterfly_collision_data
                    .subfamilies
                    .insert(subfamily.clone());
            }
        }
        if let Some(tribe) = &moth_entry.classification.tribe {
            if butterfly_data.tribes.remove(&tribe.to_lowercase()) {
                butterfly_collision_data.tribes.insert(tribe.clone());
            }
        }
        if let Some(subtribe) = &moth_entry.classification.subtribe {
            if butterfly_data.subtribes.remove(&subtribe.to_lowercase()) {
                butterfly_collision_data.subtribes.insert(subtribe.clone());
            }
        }
        if butterfly_data
            .genera
            .remove(&moth_entry.classification.genus.to_lowercase())
        {
            butterfly_collision_data
                .genera
                .insert(moth_entry.classification.genus.clone());
        }
        if butterfly_data
            .epithets
            .remove(&moth_entry.classification.epithet.to_lowercase())
        {
            butterfly_collision_data
                .epithets
                .insert(moth_entry.classification.epithet.clone());
        }

        // append synonyms
        moth_entry.synonyms = synonyms
            .get(&moth_entry.catalogue_of_life_taxon_id)
            .cloned();
    }

    println!(
        "Found {} moths and {} synonym species",
        moth_entries.len(),
        moth_synonyms_count,
    );
    if bad_entry_count > 0 {
        println!("Failed to parse {bad_entry_count} entries");
    }
    println!(
        "Parsed and constructed data in: {:.6?}",
        start_time.elapsed()
    );

    println!("Writing moth data output to {}", moth_output_file_path);
    if let Err(write_error) = serde_json::to_writer_pretty(moth_output_file, &moth_entries) {
        dbg!(write_error);
    };
    println!(
        "Writing compressed moth data output to {}",
        moth_output_file_path_zstd
    );
    if let Err(err) = write_zstd(moth_output_file_path, &moth_output_file_zstd) {
        eprintln!("{err}");
    };

    println!(
        "Writing butterfly blacklist output to {}",
        butterfly_output_file_path
    );
    if let Err(write_error) = serde_json::to_writer_pretty(butterfly_output_file, &butterfly_data) {
        dbg!(write_error);
    };
    println!(
        "Writing compressed butterfly blacklist output to {}",
        butterfly_output_file_path_zstd
    );
    if let Err(err) = write_zstd(butterfly_output_file_path, &butterfly_output_file_zstd) {
        eprintln!("{err}");
    };

    println!(
        "Writing butterfly blacklist collisions output to {}",
        butterfly_collisions_output_file_path
    );
    if let Err(write_error) = serde_json::to_writer_pretty(butterfly_collisions_output_file, &butterfly_collision_data) {
        dbg!(write_error);
    };
}

fn write_zstd(input_file_path: &str, mut output_file: &File) -> Result<(), Box<dyn Error>> {
    let mut compression_target_data = Vec::new();
    File::open(input_file_path)?.read_to_end(&mut compression_target_data)?;

    let max_compression_level = *zstd::compression_level_range().end();
    let mut compressor = zstd::bulk::Compressor::new(max_compression_level)?;
    output_file.write(&compressor.compress(&compression_target_data)?)?;

    return Ok(());
}
