use crate::{file::get_file_full_path, CliArgs};
use anyhow::{anyhow, Ok, Result};
use dialoguer::{theme::ColorfulTheme, Input, Select};

use super::{
    cli::{get_default_migration_name, get_stat_key, RunArgs, StatType},
    processor::get_target_ids,
};

impl CliArgs for RunArgs {
    fn parse_args(&mut self) -> Result<Self> {
        if self.key.is_empty() {
            self.key = get_stat_key(&self.s_type);
        }
        Ok(self.clone())
    }
    fn parse_args_interactively(&mut self) -> Result<Self> {
        let file: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Path of stats xlsx")
            .default("fixtures/test.csv".into())
            .interact_text()?;
        let path = get_file_full_path(&file)?;
        if !path.exists() {
            return Err(anyhow!("Path invalid"));
        }
        let range_items = &vec![StatType::Quarterly, StatType::Monthly, StatType::Weekly];
        let picked_range = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a stats type")
            .items(range_items)
            .default(0)
            .interact()?;
        let s_type = range_items[picked_range];
        let key = get_stat_key(&s_type);

        let migration_file_name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Filename of this migration")
            .default(get_default_migration_name())
            .interact_text()?;
        let target_ids_count = get_target_ids().map_or(0, |v| v.len());

        let do_filter = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Auto filter base on account_id.txt? (count: {target_ids_count})"
            ))
            .default(false)
            .interact_text()?;
        let raise_pr = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Auto raise phinx migration PR?")
            .default(false)
            .interact_text()?;
        Ok(RunArgs {
            file: path,
            key,
            migration_file_name,
            raise_pr,
            do_filter,
            s_type,
        })
    }
}
