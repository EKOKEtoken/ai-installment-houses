use std::collections::HashMap;
use std::path::Path;

use candid::Principal;
use csv::ReaderBuilder;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Metadata {
    pub id: u64,
    pub country: String,
    pub city: String,
    pub address: String,
    pub zip_code: String,
    pub civic: Option<String>,
    pub floor: Option<i32>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub initial_owner: Option<Principal>,
}

impl Metadata {
    pub fn load_database(p: &Path) -> anyhow::Result<HashMap<u64, Metadata>> {
        let mut rdr = ReaderBuilder::new().has_headers(true).from_path(p)?;
        let mut metadata = HashMap::new();
        for result in rdr.deserialize() {
            let record: Metadata = result?;
            metadata.insert(record.id, record);
        }
        Ok(metadata)
    }
}

#[cfg(test)]
mod test {

    use std::fs::File;
    use std::io::Write as _;

    use pretty_assertions::assert_eq;
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_should_parse_metadata() {
        // write dummy csv
        let csv = r#"id,country,city,address,zip_code,civic,floor,latitude,longitude,initial_owner
1,italy,milano,via cavalcanti 9,33100,9,2,45.4169784,9.0514524,bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe
2,italy,milano,via cavalcanti 45,33100,,,,,
"#;
        let csv_file = NamedTempFile::new().unwrap();
        // write
        let mut f = File::create(&csv_file).unwrap();
        write!(&mut f, "{}", csv).unwrap();
        f.flush().unwrap();

        // parse csv
        let metadata = Metadata::load_database(csv_file.path()).unwrap();
        assert_eq!(metadata.len(), 2);
        let first = metadata.get(&1_u64).unwrap();
        assert_eq!(first.id, 1);
        assert_eq!(first.country, "italy");
        assert_eq!(first.city, "milano");
        assert_eq!(first.address, "via cavalcanti 9");
        assert_eq!(first.zip_code, "33100");
        assert_eq!(first.civic, Some("9".to_string()));
        assert_eq!(first.floor, Some(2));
        assert_eq!(first.latitude, Some(45.4169784));
        assert_eq!(first.longitude, Some(9.0514524));
        assert_eq!(
            first.initial_owner,
            Some(
                Principal::from_text(
                    "bs5l3-6b3zu-dpqyj-p2x4a-jyg4k-goneb-afof2-y5d62-skt67-3756q-dqe"
                )
                .unwrap()
            )
        );

        let second = metadata.get(&2u64).unwrap();
        assert_eq!(second.id, 2);
        assert_eq!(second.country, "italy");
        assert_eq!(second.city, "milano");
        assert_eq!(second.address, "via cavalcanti 45");
        assert_eq!(second.zip_code, "33100");
        assert!(second.civic.is_none());
        assert!(second.floor.is_none());
        assert!(second.latitude.is_none());
        assert!(second.longitude.is_none());
        assert!(second.initial_owner.is_none());
    }
}
