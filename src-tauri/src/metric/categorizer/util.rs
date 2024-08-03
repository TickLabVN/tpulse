pub fn load_categorized_dataset(filename: &str) -> Vec<(String, String)> {
    let index_file_path = std::env::current_dir()
        .expect("Should be able to retrieve current path from inverted_index_category")
        .join("assets")
        .join(filename)
        .to_str()
        .expect("Should be able to get index.csv path")
        .to_string();
    read_csv(index_file_path).into_iter().collect()
}

fn read_csv(file_path: String) -> Vec<(String, String)> {
    let raw_csv = std::fs::read_to_string(file_path.clone())
        .expect(format!("Should be able to read {file_path}").as_str());

    let rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(raw_csv.as_bytes());

    rdr.into_records()
        .map(|r| {
            let r = r.expect("Should be able to parse csv record");
            (normalize_str(&r[0]), r[1].to_string())
        })
        .collect()
}

pub fn normalize_str(s: &str) -> String {
    s.chars().filter(|c| c.is_ascii_alphanumeric()).collect()
}
