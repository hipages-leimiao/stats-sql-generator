use anyhow::Result;
use clap::Parser;
use stats_sql_generator::{
    process::profile_stats::cli::{Action, RunArgs},
    Cli, CliArgs, Processor,
};
fn main() -> Result<()> {
    let cfg = Cli::<Action>::parse();

    match cfg.action {
        Action::Run(mut args) => {
            args.parse_args()?.run()?;
        }
        Action::Parse => RunArgs::default().parse_args_interactively()?.run()?,
    }
    Ok(())
}
