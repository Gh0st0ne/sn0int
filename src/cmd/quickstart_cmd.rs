use errors::*;

use args::Install;
use api::Client;
// use colored::Colorize;
use cmd::mod_cmd;
use registry;
use shell::Readline;
use structopt::StructOpt;
use sn0int_common::ModuleID;


#[derive(Debug, StructOpt)]
pub struct Args {
}

pub fn run(rl: &mut Readline, args: &[String]) -> Result<()> {
    let _args = Args::from_iter_safe(args)?;
    let config = rl.config().clone();

    let client = Client::new(&config)?;

    for module in client.quickstart()? {
        info!("Installing {:?}", module);
        registry::run_install(&Install {
            module: ModuleID {
                author: module.author,
                name: module.name,
            },
            version: None,
        }, &config)?;
    }

    // trigger reload
    mod_cmd::run(rl, &[String::from("mod"), String::from("reload")])?;

    Ok(())
}
