use serde::Deserialize;

use crate::json_types::ThreatStatus;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct COLTaxonTSVRaw {
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
    pub dwc_taxonomic_status: COLTaxonomicStatusRaw,
    #[serde(rename = "dwc:taxonRank")]
    pub dwc_taxon_rank: Option<TaxonRank>,
    #[serde(rename = "dwc:scientificName")]
    pub dwc_scientific_name: String,
    #[serde(rename = "dwc:scientificNameAuthorship")]
    pub dwc_scientific_name_authorship: String,
    #[serde(rename = "col:notho")]
    pub col_notho: String,
    #[serde(rename = "dwc:genericName")]
    pub dwc_generic_name: Option<String>,
    #[serde(rename = "dwc:infragenericEpithet")]
    pub dwc_infrageneric_epithet: String,
    #[serde(rename = "dwc:specificEpithet")]
    pub dwc_specific_epithet: Option<String>,
    #[serde(rename = "dwc:infraspecificEpithet")]
    pub dwc_infraspecific_epithet: Option<String>,
    #[serde(rename = "dwc:cultivarEpithet")]
    pub dwc_cultivar_epithet: String,
    #[serde(rename = "dwc:nameAccordingTo")]
    pub dwc_name_according_to: String,
    #[serde(rename = "dwc:namePublishedIn")]
    pub dwc_name_published_in: Option<String>,
    #[serde(rename = "dwc:nomenclaturalCode")]
    pub dwc_nomenclatural_code: String,
    #[serde(rename = "dwc:nomenclaturalStatus")]
    pub dwc_nomenclatural_status: String,
    #[serde(rename = "dwc:kingdom")]
    pub dwc_kingdom: Option<String>,
    #[serde(rename = "dwc:phylum")]
    pub dwc_phylum: Option<String>,
    #[serde(rename = "dwc:class")]
    pub dwc_class: Option<String>,
    #[serde(rename = "dwc:order")]
    pub dwc_order: Option<String>,
    #[serde(rename = "dwc:superfamily")]
    pub dwc_superfamily: Option<String>,
    #[serde(rename = "dwc:family")]
    pub dwc_family: Option<String>,
    #[serde(rename = "dwc:subfamily")]
    pub dwc_subfamily: Option<String>,
    #[serde(rename = "dwc:tribe")]
    pub dwc_tribe: Option<String>,
    #[serde(rename = "dwc:subtribe")]
    pub dwc_subtribe: Option<String>,
    #[serde(rename = "dwc:genus")]
    pub dwc_genus: Option<String>,
    #[serde(rename = "dwc:subgenus")]
    pub dwc_subgenus: Option<String>,
    #[serde(rename = "dwc:taxonRemarks")]
    pub dwc_taxon_remarks: String,
    #[serde(rename = "dcterms:references")]
    pub dcterms_references: String,
    #[serde(rename = "clb:merged")]
    pub clb_merged: String,
}

#[derive(Debug, Deserialize)]
pub enum COLTaxonomicStatusRaw {
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
pub struct COLVernacularNameTSVRaw {
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
pub struct COLSpeciesProfileTSVRaw {
    #[serde(rename = "dwc:taxonID")]
    pub dwc_taxon_id: String,
    #[serde(rename = "gbif:isExtinct")]
    pub gbif_is_extinct: Option<bool>,
    #[serde(rename = "gbif:isMarine")]
    pub gbif_is_marine: Option<bool>,
    #[serde(rename = "gbif:isFreshwater")]
    pub gbif_is_freshwater: Option<bool>,
    #[serde(rename = "gbif:isTerrestrial")]
    pub gbif_is_terrestrial: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct COLDistributionTSVRaw {
    #[serde(rename = "dwc:taxonID")]
    pub dwc_taxon_id: String,
    #[serde(rename = "dwc:establishmentMeans")]
    pub dwc_establishment_means: String,
    #[serde(rename = "dwc:degreeOfEstablishment")]
    pub dwc_degree_of_establishment: String,
    #[serde(rename = "iucn:threatStatus")]
    pub iucn_threat_status: Option<IUCNThreatStatusRaw>,
    #[serde(rename = "dwc:pathway")]
    pub dwc_pathway: String,
    #[serde(rename = "dwc:lifeStage")]
    pub dwc_life_stage: String,
    #[serde(rename = "dwc:occurrenceStatus")]
    pub dwc_occurrence_status: String,
    #[serde(rename = "dwc:locationID")]
    pub dwc_location_id: String,
    #[serde(rename = "dwc:locality")]
    pub dwc_locality: Option<String>,
    #[serde(rename = "dwc:countryCode")]
    pub dwc_country_code: String,
    #[serde(rename = "dcterms:source")]
    pub dcterms_source: String,
    #[serde(rename = "clb:merged")]
    pub clb_merged: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct IUCNTaxonTXTRaw {
    pub id: String,
    #[serde(rename = "scientificName")]
    pub scientific_name: String,
    pub kingdom: Option<String>,
    pub phylum: Option<String>,
    pub class: Option<String>,
    pub order: Option<String>,
    pub family: Option<String>,
    pub genus: Option<String>,
    #[serde(rename = "specificEpithet")]
    pub specific_epithet: Option<String>,
    #[serde(rename = "scientificNameAuthorship")]
    pub scientific_name_authorship: String,
    #[serde(rename = "taxonRank")]
    pub taxon_rank: Option<TaxonRank>,
    #[serde(rename = "infraspecificEpithet")]
    pub infraspecific_epithet: Option<String>,
    #[serde(rename = "taxonomicStatus")]
    pub taxonomic_status: String,
    #[serde(rename = "acceptedNameUsageID")]
    pub accepted_name_usage_id: String,
    #[serde(rename = "bibliographicCitation")]
    pub bibliographic_citation: String,
    pub references: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct IUCNVernacularTXTRaw {
    pub id: String,
    #[serde(rename = "isPreferredName")]
    pub is_preferred_name: String,
    #[serde(rename = "vernacularName")]
    pub vernacular_name: String,
    pub language: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct IUCNDistributionTXTRaw {
    pub id: String,
    #[serde(rename = "establishmentMeans")]
    pub establishment_means: String,
    pub source: String,
    #[serde(rename = "countryCode")]
    pub country_code: String,
    pub locality: String,
    #[serde(rename = "threatStatus")]
    pub threat_status: IUCNThreatStatusRaw,
    #[serde(rename = "occurrenceStatus")]
    pub occurrence_status: String,
}

#[derive(Debug, Deserialize)]
pub enum IUCNThreatStatusRaw {
    #[serde(rename = "Least Concern")]
    #[serde(alias = "least concern")]
    LeastConcern,
    #[serde(rename = "Conservation Dependent")]
    #[serde(alias = "conservation dependent")]
    ConservationDependent,
    #[serde(rename = "Near Threatened")]
    #[serde(alias = "near threatened")]
    NearThreatened,
    #[serde(rename = "Vulnerable")]
    #[serde(alias = "vulnerable")]
    Vulnerable,
    #[serde(rename = "Endangered")]
    #[serde(alias = "endangered")]
    Endangered,
    #[serde(rename = "Critically Endangered")]
    #[serde(alias = "critically endangered")]
    CriticallyEndangered,
    #[serde(rename = "Extinct in the Wild")]
    #[serde(alias = "extinct in the wild")]
    ExtinctInTheWild,
    #[serde(rename = "Extinct")]
    #[serde(alias = "extinct")]
    Extinct,
    #[serde(rename = "Not Evaluated")]
    #[serde(alias = "not evaluated")]
    NotEvaluated,
    #[serde(rename = "Data Deficient")]
    #[serde(alias = "data deficient")]
    DataDeficient,
}

impl IUCNThreatStatusRaw {
    pub fn into_threatstatus(&self) -> Option<ThreatStatus> {
        match self {
            IUCNThreatStatusRaw::LeastConcern => Some(ThreatStatus::LeastConcern),
            IUCNThreatStatusRaw::ConservationDependent => Some(ThreatStatus::ConservationDependent),
            IUCNThreatStatusRaw::NearThreatened => Some(ThreatStatus::NearThreatened),
            IUCNThreatStatusRaw::Vulnerable => Some(ThreatStatus::Vulnerable),
            IUCNThreatStatusRaw::Endangered => Some(ThreatStatus::Endangered),
            IUCNThreatStatusRaw::CriticallyEndangered => Some(ThreatStatus::CriticallyEndangered),
            IUCNThreatStatusRaw::ExtinctInTheWild => Some(ThreatStatus::ExtinctInTheWild),
            IUCNThreatStatusRaw::Extinct => Some(ThreatStatus::Extinct),
            IUCNThreatStatusRaw::NotEvaluated => None,
            IUCNThreatStatusRaw::DataDeficient => None,
        }
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum TaxonRank {
    #[serde(rename = "domain")]
    Domain,
    #[serde(rename = "realm")]
    Realm,
    #[serde(rename = "kingdom")]
    Kingdom,
    #[serde(rename = "subkingdom")]
    SubKingdom,
    #[serde(rename = "infrakingdom")]
    InfraKingdom,
    #[serde(rename = "phylum")]
    Phylum,
    #[serde(rename = "subphylum")]
    SubPhylum,
    #[serde(rename = "infraphylum")]
    InfraPhylum,
    #[serde(rename = "parvphylum")]
    ParvPhylum,
    #[serde(rename = "gigaclass")]
    GigaClass,
    #[serde(rename = "megaclass")]
    MegaClass,
    #[serde(rename = "superclass")]
    SuperClass,
    #[serde(rename = "class")]
    Class,
    #[serde(rename = "superorder")]
    SuperOrder,
    #[serde(rename = "subclass")]
    SubClass,
    #[serde(rename = "infraclass")]
    InfraClass,
    #[serde(rename = "subterclass")]
    SubterClass,
    #[serde(rename = "order")]
    Order,
    #[serde(rename = "suborder")]
    SubOrder,
    #[serde(rename = "infraorder")]
    InfraOrder,
    #[serde(rename = "parvorder")]
    ParvOrder,
    #[serde(rename = "nanorder")]
    NanOrder,
    #[serde(rename = "superfamily")]
    SuperFamily,
    #[serde(rename = "family")]
    Family,
    #[serde(rename = "epifamily")]
    EpiFamily,
    #[serde(rename = "subfamily")]
    SubFamily,
    #[serde(rename = "infrafamily")]
    InfraFamily,
    #[serde(rename = "supertribe")]
    SuperTribe,
    #[serde(rename = "tribe")]
    Tribe,
    #[serde(rename = "subtribe")]
    SubTribe,
    #[serde(rename = "infratribe")]
    InfraTribe,
    #[serde(rename = "genus")]
    #[serde(alias = "infrageneric name")]
    Genus,
    #[serde(rename = "infragenus")]
    InfraGenus,
    #[serde(rename = "subgenus")]
    SubGenus,
    #[serde(rename = "section")]
    #[serde(alias = "section botany")]
    #[serde(alias = "section zoology")]
    Section,
    #[serde(rename = "subsection")]
    #[serde(alias = "subsection botany")]
    #[serde(alias = "subsection zoology")]
    SubSection,
    #[serde(rename = "series")]
    Series,
    #[serde(rename = "subseries")]
    SubSeries,
    #[serde(rename = "species aggregate")]
    SuperSpecies,
    #[serde(rename = "species")]
    Species,
    #[serde(rename = "subspecies")]
    #[serde(alias = "subspecies (plantae)")]
    SubSpecies,
    #[serde(rename = "infraspecific name")]
    InfraSpecificName,
    #[serde(rename = "infrasubspecific name")]
    InfraSubSpecificEpithet,
    #[serde(rename = "natio")]
    Natio, // i have no clue what this is but it's below subspecies (https://www.marinespecies.org/copepoda/aphia.php?p=taxdetails&id=363902)
    #[serde(rename = "forma specialis")]
    FormaSpecialis, // not necessarily above or below subspecies
    #[serde(rename = "variety")]
    Variety,
    #[serde(rename = "subvariety")]
    SubVariety,
    #[serde(rename = "form")]
    Form,
    #[serde(rename = "morph")]
    Morph,
    #[serde(rename = "subform")]
    SubForm,
    #[serde(rename = "aberration")]
    Aberration,
    #[serde(rename = "race")]
    #[serde(alias = "proles")]
    Race,
    #[serde(rename = "mutation")]
    #[serde(alias = "mutatio")]
    Mutation,
    #[serde(rename = "unrankedsynonym")]
    #[serde(alias = "lusus")]
    UnrankedSynonym,
    #[serde(rename = "unranked")]
    #[serde(alias = "other")]
    Unranked,
}
