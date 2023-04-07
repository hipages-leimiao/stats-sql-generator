use anyhow::Result;
use clap::{command, Parser, Subcommand};
mod date;
pub mod file;
pub mod process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli<T: Subcommand> {
    #[clap(subcommand)]
    pub action: T,
}

pub trait Processor
where
    Self: Sized,
{
    type Item;

    fn load_data(&self) -> Result<Vec<Self::Item>>;

    fn generate_result_in_string(&self, data: &[Self::Item]) -> Result<String>;

    fn write_data(&self, result_str: &str) -> Result<()>;
    fn run_post_script(&self) -> Result<()> {
        Ok(())
    }
    fn run(&self) -> Result<()> {
        let data = self.load_data()?;
        let result = self.generate_result_in_string(&data)?;
        self.write_data(&result)?;
        self.run_post_script()?;
        Ok(())
    }
}

pub trait CliArgs
where
    Self: Sized,
{
    fn parse_args(&mut self) -> Result<Self>;
    fn parse_args_interactively(&mut self) -> Result<Self>;
}
