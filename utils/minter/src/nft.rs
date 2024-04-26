use std::path::Path;

use dip721_rs::GenericValue;

use crate::metadata::Metadata;

pub struct Nft {
    pub country: String,
    pub city: String,
    pub address: String,
    pub zip_code: String,
    /// The civic number of the address
    pub civic: Option<String>,
    pub floor: Option<i32>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

impl Nft {
    pub fn get_from_metadata(metadata: Metadata, image_dir: &Path) -> anyhow::Result<Self> {
        todo!()
    }

    pub fn into_properties(self) -> Vec<(String, GenericValue)> {
        let mut props = vec![
            (
                "country".to_string(),
                GenericValue::TextContent(self.country),
            ),
            ("city".to_string(), GenericValue::TextContent(self.city)),
            (
                "address".to_string(),
                GenericValue::TextContent(self.address),
            ),
            (
                "zipCode".to_string(),
                GenericValue::TextContent(self.zip_code),
            ),
        ];
        if let Some(civic) = self.civic {
            props.push(("civic".to_string(), GenericValue::TextContent(civic)));
        }
        if let Some(floor) = self.floor {
            props.push(("floor".to_string(), GenericValue::Int32Content(floor)));
        }
        if let Some(latitude) = self.latitude {
            props.push(("latitude".to_string(), GenericValue::FloatContent(latitude)));
        }
        if let Some(longitude) = self.longitude {
            props.push((
                "longitude".to_string(),
                GenericValue::FloatContent(longitude),
            ));
        }
        props
    }
}
