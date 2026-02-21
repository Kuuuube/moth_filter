# Moth Filter

Extract only moth species information from Catalogue of Life's database release.

## Filtering

All entries for species rank, order Lepidoptera and superfamily is not Papilionoidea.

Papilionoidea contains all the butterflies. Moths are everything besides butterflies in Lepidoptera (`Lepidoptera - Papilionoidea = Moths`).

## Data Downloads

https://www.catalogueoflife.org/data/download or https://download.checklistbank.org/col/monthly/

The `Darwin Core Archive` or `dwca` is used here. Extract and move it `Taxon.tsv` to `./data/Taxon.tsv`.

Downloads for specific taxonomic groups can be created with an account.
