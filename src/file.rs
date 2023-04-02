use anyhow::{Ok, Result};
use csv::ReaderBuilder;
use dirs;
use serde::de::DeserializeOwned;
use std::{
    fmt::Debug,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use calamine::{open_workbook_auto, Error, RangeDeserializerBuilder, Reader};

pub fn get_file_full_path(path: &str) -> Result<PathBuf> {
    let mut full_path = PathBuf::new();
    if path.starts_with("~/") {
        let home_dir = dirs::home_dir().unwrap();
        full_path.push(home_dir);
        full_path.push(path.trim_start_matches("~/"));
    } else {
        full_path.push(path);
    }
    Ok(full_path)
}

pub fn load_excel_data<T>(path: &Path) -> Result<Vec<T>>
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

pub fn load_csv_line_stream<T: DeserializeOwned>(
    file_path: &str,
) -> impl Iterator<Item = Result<T, csv::Error>> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let r = ReaderBuilder::new().has_headers(true).from_reader(reader);

    r.into_deserialize()
}
