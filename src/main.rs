use std::env;
use log::{info, warn, error, debug};
use anyhow::Result;
use clap::Parser;
use activemq::activemq;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    cluster: bool,
}

fn main() -> Result<()> { -> Result<()> {
    let cli = Cli::parse();
    let config = activemq::Config { 
        cluster_mode: cli.cluster, 
        ..Default::default() 
    };
    let mut component = activemq::new(config);
    
    component.initialize()?;
    component.deploy()?;
    
    println!("SigmaOS-activemq deployed");
    Ok(())
}

#[derive(Debug, Clone)]
struct Config {
    verbose: bool,
    dry_run: bool,
}

impl Config {
    fn from_env() -> Self {
        Config {
            verbose: env::var("VERBOSE").is_ok(),
            dry_run: env::var("DRY_RUN").is_ok(),
        }
    }
}
