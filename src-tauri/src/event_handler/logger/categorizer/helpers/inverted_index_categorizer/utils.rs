pub fn read_csv(file_path: String) -> Vec<(String, String)> {
    let raw_csv = std::fs::read_to_string(file_path.clone())
        .expect(format!("Should be able to read {file_path}").as_str());

    let rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .from_reader(raw_csv.as_bytes());

    rdr.into_records()
        .map(|r| {
            let r = r.expect("Should be able to parse csv record");
            (r[0].to_string(), r[1].to_string())
        })
        .collect()
}
