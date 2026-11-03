use anyhow::Result;
use clap::Parser;
use activemq::activemq;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    cluster: bool,
}

fn main() -> Result<()> {
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
