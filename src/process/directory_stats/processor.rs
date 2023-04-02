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
        Ok(load_excel_data::<Self::Item>(self.file.as_path()).unwrap())
    }

    fn generate_result_in_string(&self, data: &Vec<Self::Item>) -> Result<String> {
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
                        .collect::<Vec<String>>()
                        .join(",")
                )
            })
            .collect::<Vec<String>>()
            .join("\n");
        Ok(sql)
    }

    fn write_data(&self, result_str: &str) -> anyhow::Result<()> {
        let tpl = include_str!("../../../fixtures/migration_tpl.txt")
            .replace("{sql}", &result_str)
            .replace("{file_name}", &self.migration_file_name);
        let output_file = "migration_output.php";
        println!("migration sql generates to file: {}", output_file);
        fs::write(output_file, tpl).expect("Unable to write migration file");
        Ok(())
    }

    fn run_post_script(&self) -> Result<()> {
        if !self.raise_pr {
            return Ok(());
        }
        let output = Command::new("sh")
            .args(&["src/bin/directory_stats_post.sh", &self.migration_file_name])
            .output()
            .expect("failed to add migration file to phinx");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
        Ok(())
    }
}
