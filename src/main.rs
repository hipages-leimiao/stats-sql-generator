use std::{
    cell::RefCell,
    fs,
    io::{self, Write},
    path::Path,
    process::Command,
};

use anyhow::{anyhow, Ok, Result};
use clap::Parser;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use indexmap::IndexMap;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use stats_sql_generator::{
    cli::{Action, Cli, RunArgs, StatType},
    file::load_data,
    get_file_full_path, get_stat_key,
};

fn main() -> Result<()> {
    let cfg = Cli::parse();

    match cfg.action {
        Action::Run(mut args) => {
            if !args.key.is_empty() {
                args.key = get_stat_key(&args.s_type);
            }
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
        .default("fixtures/test.csv".into())
        .interact_text()?;
    let path = get_file_full_path(&file)?;
    if !path.exists() {
        return Err(anyhow!("Path invalid"));
    }
    let parsed = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Is data already parsed?")
        .default(true)
        .interact_text()?;

    let range_items = &vec![
        StatType::All,
        StatType::Quarterly,
        StatType::Monthly,
        StatType::Weekly,
    ];
    let picked_range: usize = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a stats type")
        .items(range_items)
        .default(0)
        .interact()?
        .into();
    let s_type = range_items[picked_range];
    let key = get_stat_key(&s_type);

    let migration_file_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Filename of this migration")
        .default("SeedProfileStatsBatch".into())
        .interact_text()?;
    let raise_pr = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Auto raise phinx migration PR?")
        .default(false)
        .interact_text()?;
    Ok(RunArgs {
        file: path,
        parsed,
        key,
        migration_file_name,
        raise_pr,
        s_type,
    })
}
fn run(args: RunArgs) {
    let vals: Vec<ValueType> = match args.parsed {
        true => load_data(args.file.as_path()).unwrap(),
        false => load_data_and_parse(args.file.as_path()),
    };
    let migrate_file_name = args.migration_file_name.clone();
    let raise_pr = args.raise_pr;
    gen_migration_file(args, vals);
    if raise_pr {
        add_migration_to_phinx(migrate_file_name);
    }
}

fn add_migration_to_phinx(migration_file_name: String) {
    let output = Command::new("sh")
        .args(&["migration.sh", &migration_file_name])
        .output()
        .expect("failed to add migration file to phinx");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}

fn load_data_and_parse(path: &Path) -> Vec<ValueType> {
    let data: Vec<ValueRawType> = load_data(path).unwrap();

    let account_stats: RefCell<IndexMap<&u32, ValueType>> = RefCell::new(IndexMap::new());
    let mut map = account_stats.borrow_mut();
    data.iter().for_each(|v| {
        let parsed = map
            .entry(&v.account_id)
            .or_insert_with(|| ValueType::new(v.account_id));
        let counts = v.sessions.unwrap_or(0);
        if counts > 0 {
            let activity = v.activity.as_ref().unwrap().as_str();
            match activity {
                "profile view" => parsed.profile_view = counts,
                "phone clicks" => parsed.phone_clicks = counts,
                "view gallery" => parsed.view_gallery = counts,
                _ => println!("Warn: unknown activity - {:?}", activity),
            }
        }
    });
    map.values().cloned().collect::<Vec<ValueType>>()
}
fn gen_migration_file(args: RunArgs, vals: Vec<ValueType>) {
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
                            v.account_id, stat_key, v.profile_view, v.phone_clicks, v.view_gallery
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
    activity: Option<String>,
    app: Option<String>,
    sessions: Option<u32>,
}

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
impl ValueType {
    fn new(account_id: u32) -> Self {
        Self {
            account_id,
            ..Default::default()
        }
    }
}
