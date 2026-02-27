# Moth Filter

Extract only moth species information from Catalogue of Life's database release.

A list of classifications for butterflies is also provided for easily determining butterfly or moth when the superfamily is not available. But beware filtering solely based on genus or epithet, there are many colliding genus and epithet names between butterflies and moths. All collisions have been removed from the butterfly blacklist.

## Filtering

All entries for species rank, order Lepidoptera and superfamily is not Papilionoidea.

Papilionoidea contains all the butterflies. Moths are everything besides butterflies in Lepidoptera (`Lepidoptera - Papilionoidea = Moths`).

## Data Downloads

https://www.catalogueoflife.org/data/download or https://download.checklistbank.org/col/monthly/

The `Darwin Core Archive` or `dwca` is used here (base or extended `xr` is up to you). Extract and move the contents to `./data`. You should have a bunch of `tsv` files directly in the data directory.

Downloads for specific taxonomic groups can be created with an account.

## Generating Output

After getting the downloads sorted, assuming you already have [Rust](https://rust-lang.org/tools/install/) installed as well, run this:

```bash
cargo run --release
```

## Lib

Minimal docs are available for injesting the output json files provided.

[Auto Generated Docs](https://kuuuube.github.io/moth_filter/moth_filter/)
