use std::{cell::RefCell, collections::HashMap, fs, path::PathBuf};

use anyhow::{anyhow, Ok, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use stats_sql_generator::{
    cli::{Action, Cli, RunArgs},
    xlsx::XlsxReader,
};

fn main() -> Result<()> {
    let cfg = Cli::parse();

    match cfg.action {
        Action::Run(args) => {
            run(args);
        }
        Action::Parse => {
            let arg = parse()?;
            run(arg)
        }
    }
    Ok(())
}
fn parse() -> Result<RunArgs> {
    let file: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Path of stats xlsx")
        .interact_text()?;
    let path = PathBuf::from(file);
    if !path.exists() {
        return Err(anyhow!("Path invalid"));
    }
    let key: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Time range for this batch of stats migration (eg: 1 September 2022 - 31 January 2023)",
        )
        .interact_text()?;
    let migration_file_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Filename of this migration (eg: SeedProfileStatsBatch*)")
        .interact_text()?;
    Ok(RunArgs {
        file: path,
        key,
        migration_file_name,
    })
}
fn run(args: RunArgs) {
    let data = ProfileStats::load_data(args.file).unwrap();
    let account_stats: RefCell<HashMap<&u32, ValueType>> = RefCell::new(HashMap::new());
    let mut map = account_stats.borrow_mut();
    data.iter().for_each(|v| {
        let parsed = map
            .entry(&v.account_id)
            .or_insert_with(|| ValueType::new(v.account_id));
        match v.activity.as_str() {
            "profile view" => parsed.profile_views = v.sessions,
            "phone clicks" => parsed.contact_number_impressions = v.sessions,
            "view gallery" => parsed.gallery_impressions = v.sessions,
            _ => println!("{:?}", v),
        }
    });
    let vals = map.values().cloned().collect::<Vec<ValueType>>();
    let sql = gen_sql(&vals, args.key);
    let tpl = include_str!("../fixtures/migration_tpl.txt")
        .replace("{sql}", &sql)
        .replace("{file_name}", &args.migration_file_name);
    let output_file = "migration_output.php";
    println!("migration sql generates to file: {}", output_file);
    fs::write(output_file, tpl).expect("Unable to write migration file");
}

fn gen_sql(values: &Vec<ValueType>, stat_key: String) -> String {
    let chunk_size = 10000;
    let sql_prefix: &str = "insert ignore into directory_tradie_statistics (account_id,stats_key,profile_views,contact_number_impressions,gallery_impressions) values ";
    values
        .par_chunks(chunk_size)
        .map(|chunk| {
            format!(
                "{}{};",
                sql_prefix,
                chunk
                    .iter()
                    .map(|v| {
                        format!(
                            "({},'{}',{},{},{})",
                            v.account_id,
                            stat_key,
                            v.profile_views,
                            v.contact_number_impressions,
                            v.gallery_impressions
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(",")
            )
        })
        .collect::<Vec<String>>()
        .join("\n")
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ValueRawType {
    account_id: u32,
    account_type: Option<String>,
    client_type: Option<String>,
    account_created_date_dim_key: Option<String>,
    activity: String,
    app: Option<usize>,
    sessions: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ValueType {
    account_id: u32,
    profile_views: u32,
    contact_number_impressions: u32,
    gallery_impressions: u32,
}
impl ValueType {
    fn new(account_id: u32) -> Self {
        Self {
            account_id,
            ..Default::default()
        }
    }
}
pub struct ProfileStats;
impl XlsxReader for ProfileStats {
    type Item = ValueRawType;
}
