use crate::{file::get_file_full_path, CliArgs};
use anyhow::{anyhow, Ok, Result};
use dialoguer::{theme::ColorfulTheme, Input, Select};

use super::cli::{get_stat_key, RunArgs, StatType};

impl CliArgs for RunArgs {
    fn parse_args(&mut self) -> Result<Self> {
        if !self.key.is_empty() {
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
        let range_items = &vec![
            StatType::Default,
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
            key,
            migration_file_name,
            raise_pr,
            s_type,
        })
    }
}
