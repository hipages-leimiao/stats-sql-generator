use anyhow::{Ok, Result};
use serde::de::DeserializeOwned;
use std::path::PathBuf;

use calamine::{open_workbook, Error, RangeDeserializerBuilder, Reader, Xlsx};

pub trait XlsxReader
where
    Self::Item: Sized + DeserializeOwned,
{
    type Item;
    fn load_data(path: PathBuf) -> Result<Vec<Self::Item>> {
        let mut workbook: Xlsx<_> = open_workbook(path)?;
        let range = workbook
            .worksheet_range_at(0)
            .ok_or(Error::Msg("Cannot find 'Sheet1'"))??;

        let iter = RangeDeserializerBuilder::new().from_range::<_, Self::Item>(&range)?;

        let values = iter.into_iter().map(|v| v.ok().unwrap()).collect();
        Ok(values)
    }
}
