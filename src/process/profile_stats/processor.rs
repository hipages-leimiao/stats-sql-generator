use anyhow::{Ok, Result};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
    process::Command,
};

use crate::{file::load_excel_data, Processor};

use super::cli::RunArgs;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ValueType {
    account_id: u32,
    account_type: Option<String>,
    client_type: Option<String>,
    account_created_date_dim_key: Option<String>,
    profile_view: u32,
    phone_clicks: u32,
    view_gallery: u32,
}

impl Processor for RunArgs {
    type Item = ValueType;

    fn load_data(&self) -> anyhow::Result<Vec<Self::Item>> {
        let mut data = load_excel_data::<Self::Item>(self.file.as_path()).unwrap();
        // filter with target account ids
        if let Some(account_ids) = get_target_ids() {
            data = data
                .into_iter()
                .filter_map(|v| {
                    if !account_ids.contains(&v.account_id) {
                        return None;
                    }
                    Some(v)
                })
                .collect::<Vec<Self::Item>>();
        };
        Ok(data)
    }

    fn generate_result_in_string(&self, data: &[Self::Item]) -> Result<String> {
        let s_type = self.s_type.as_str();
        let chunk_size = 10000;
        let sql_prefix: &str = "replace into directory_tradie_statistics (account_id,stats_key,stats_type,profile_views,contact_number_impressions,gallery_impressions) values ";
        let sql = data
            .par_chunks(chunk_size)
            .map(|chunk| {
                format!(
                    "{}{};",
                    sql_prefix,
                    chunk
                        .iter()
                        .map(|v| {
                            format!(
                                "({},'{}','{}',{},{},{})",
                                v.account_id,
                                self.key,
                                s_type,
                                v.profile_view,
                                v.phone_clicks,
                                v.view_gallery
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(",")
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        Ok(sql)
    }

    fn write_data(&self, result_str: &str) -> anyhow::Result<()> {
        let tpl = fs::read_to_string("fixtures/migration_tpl.txt")?
            .replace("{sql}", result_str)
            .replace("{file_name}", &self.migration_file_name);
        let output_file = "migration_output.php";
        fs::write(output_file, tpl).expect("Unable to write migration file");
        println!("migration sql has been generated to file: {}", output_file);
        Ok(())
    }

    fn run_post_script(&self) -> Result<()> {
        if !self.raise_pr {
            return Ok(());
        }
        let script = "src/bin/profile_stats_post.sh";
        let output = Command::new("sh")
            .arg(script)
            .arg(&self.migration_file_name)
            .output()?;

        io::stdout().write_all(&output.stdout)?;
        io::stderr().write_all(&output.stderr)?;
        Ok(())
    }
}

pub fn get_target_ids() -> Option<Vec<u32>> {
    let contents = fs::read_to_string("account_id.txt").unwrap_or("".to_string());
    let account_id: Vec<u32> = contents.lines().filter_map(|v| v.parse().ok()).collect();
    match !account_id.is_empty() {
        true => Some(account_id),
        false => None,
    }
}
