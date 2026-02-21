use std::{fs::File, io::{BufRead, BufReader, Write}};

const MOTH_ORDER: &str = "Lepidoptera";
const BUTTERFLY_SUPERFAMILY: &str = "Papilionoidea";

fn main() {
    let file = File::open("./data/Taxon.tsv").unwrap();
    let reader = BufReader::new(file);

    let mut output_file = File::create("./output/output.tsv").unwrap();

    for line in reader.lines() {
        let mut line_unwrap = line.unwrap();
        let line_split: Vec<_> = line_unwrap.split("\t").collect();

        if line_split[7] != "species" || line_split[23] != MOTH_ORDER || line_split[24] == BUTTERFLY_SUPERFAMILY {
            continue;
        }

        line_unwrap += "\n";
        output_file.write(line_unwrap.as_bytes()).unwrap();
    }
}
