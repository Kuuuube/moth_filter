use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct TaxonTSVRaw {
    #[serde(rename = "dwc:taxonID")]
    pub dwc_taxon_id: String,
    #[serde(rename = "dwc:parentNameUsageID")]
    pub dwc_parent_name_usage_id: String,
    #[serde(rename = "dwc:acceptedNameUsageID")]
    pub dwc_accepted_name_usage_id: String,
    #[serde(rename = "dwc:originalNameUsageID")]
    pub dwc_original_name_usage_id: String,
    #[serde(rename = "dwc:scientificNameID")]
    pub dwc_scientific_name_id: String,
    #[serde(rename = "dwc:datasetID")]
    pub dwc_dataset_id: String,
    #[serde(rename = "dwc:taxonomicStatus")]
    pub dwc_taxonomic_status: TaxonomicStatusRaw,
    #[serde(rename = "dwc:taxonRank")]
    pub dwc_taxon_rank: String,
    #[serde(rename = "dwc:scientificName")]
    pub dwc_scientific_name: String,
    #[serde(rename = "dwc:scientificNameAuthorship")]
    pub dwc_scientific_name_authorship: String,
    #[serde(rename = "col:notho")]
    pub col_notho: String,
    #[serde(rename = "dwc:genericName")]
    pub dwc_generic_name: String,
    #[serde(rename = "dwc:infragenericEpithet")]
    pub dwc_infrageneric_epithet: String,
    #[serde(rename = "dwc:specificEpithet")]
    pub dwc_specific_epithet: String,
    #[serde(rename = "dwc:infraspecificEpithet")]
    pub dwc_infraspecific_epithet: String,
    #[serde(rename = "dwc:cultivarEpithet")]
    pub dwc_cultivar_epithet: String,
    #[serde(rename = "dwc:nameAccordingTo")]
    pub dwc_name_according_to: String,
    #[serde(rename = "dwc:namePublishedIn")]
    pub dwc_name_published_in: String,
    #[serde(rename = "dwc:nomenclaturalCode")]
    pub dwc_nomenclatural_code: String,
    #[serde(rename = "dwc:nomenclaturalStatus")]
    pub dwc_nomenclatural_status: String,
    #[serde(rename = "dwc:kingdom")]
    pub dwc_kingdom: String,
    #[serde(rename = "dwc:phylum")]
    pub dwc_phylum: String,
    #[serde(rename = "dwc:class")]
    pub dwc_class: String,
    #[serde(rename = "dwc:order")]
    pub dwc_order: String,
    #[serde(rename = "dwc:superfamily")]
    pub dwc_superfamily: String,
    #[serde(rename = "dwc:family")]
    pub dwc_family: String,
    #[serde(rename = "dwc:subfamily")]
    pub dwc_subfamily: String,
    #[serde(rename = "dwc:tribe")]
    pub dwc_tribe: String,
    #[serde(rename = "dwc:subtribe")]
    pub dwc_subtribe: String,
    #[serde(rename = "dwc:genus")]
    pub dwc_genus: String,
    #[serde(rename = "dwc:subgenus")]
    pub dwc_subgenus: String,
    #[serde(rename = "dwc:taxonRemarks")]
    pub dwc_taxon_remarks: String,
    #[serde(rename = "dcterms:references")]
    pub dcterms_references: String,
    #[serde(rename = "clb:merged")]
    pub clb_merged: String,
}

#[derive(Debug, Deserialize)]
pub enum TaxonomicStatusRaw {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "provisionally accepted")]
    ProvisionallyAccepted,
    #[serde(rename = "synonym")]
    Synonym,
    #[serde(rename = "ambiguous synonym")]
    AmbiguousSynonym,
    #[serde(rename = "misapplied")]
    Misapplied,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct VernacularNameTSVRaw {
    #[serde(rename = "dwc:taxonID")]
    pub dwc_taxon_id: String,
    #[serde(rename = "dcterms:language")]
    pub dcterms_language: String,
    #[serde(rename = "dwc:vernacularName")]
    pub dwc_vernacular_name: String,
    #[serde(rename = "clb:merged")]
    pub clb_merged: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct SpeciesProfileTSVRaw {
    #[serde(rename = "dwc:taxonID")]
    pub dwc_taxon_id: String,
    #[serde(rename = "gbif:isExtinct")]
    pub gbif_is_extinct: String,
    #[serde(rename = "gbif:isMarine")]
    pub gbif_is_marine: String,
    #[serde(rename = "gbif:isFreshwater")]
    pub gbif_is_freshwater: String,
    #[serde(rename = "gbif:isTerrestrial")]
    pub gbif_is_terrestrial: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct DistributionTSVRaw {
    #[serde(rename = "dwc:taxonID")]
    pub dwc_taxon_id: String,
    #[serde(rename = "dwc:establishmentMeans")]
    pub dwc_establishment_means: String,
    #[serde(rename = "dwc:degreeOfEstablishment")]
    pub dwc_degree_of_establishment: String,
    #[serde(rename = "iucn:threatStatus")]
    pub iucn_threat_status: String,
    #[serde(rename = "dwc:pathway")]
    pub dwc_pathway: String,
    #[serde(rename = "dwc:lifeStage")]
    pub dwc_life_stage: String,
    #[serde(rename = "dwc:occurrenceStatus")]
    pub dwc_occurrence_status: String,
    #[serde(rename = "dwc:locationID")]
    pub dwc_location_id: String,
    #[serde(rename = "dwc:locality")]
    pub dwc_locality: String,
    #[serde(rename = "dwc:countryCode")]
    pub dwc_country_code: String,
    #[serde(rename = "dcterms:source")]
    pub dcterms_source: String,
    #[serde(rename = "clb:merged")]
    pub clb_merged: String,
}
