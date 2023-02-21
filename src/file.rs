use anyhow::{Ok, Result};
use serde::de::DeserializeOwned;
use std::{fmt::Debug, fs::File, path::Path};

use calamine::{open_workbook_auto, Error, RangeDeserializerBuilder, Reader};

pub fn load_data<T>(path: &Path) -> Result<Vec<T>>
where
    T: Sized + DeserializeOwned + Debug,
{
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    match ext {
        "csv" => {
            let file = File::open(path)?;
            let mut rdr = csv::Reader::from_reader(file);
            let mut results: Vec<T> = vec![];
            for item in rdr.deserialize() {
                results.push(item?);
            }
            Ok(results)
        }
        "xlsx" | "xls" => {
            let mut workbook = open_workbook_auto(path)?;
            let range = workbook
                .worksheet_range_at(0)
                .ok_or(Error::Msg("Cannot find 'Sheet1'"))??;

            let iter = RangeDeserializerBuilder::new().from_range::<_, T>(&range)?;

            let results = iter.into_iter().map(|v| v.ok().unwrap()).collect();
            Ok(results)
        }
        _ => Err(Error::Msg("Unsupported file extension").into()),
    }
}
