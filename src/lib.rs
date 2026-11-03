//! SigmaOS-activemq - SigmaOS Component
//!
//! Cloud and infrastructure component for SigmaOS.

use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub enabled: bool,
    pub cluster_mode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self { enabled: true, cluster_mode: false }
    }
}

pub struct activemq {
    config: Config,
}

impl activemq {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn initialize(&mut self) -> Result<()> {
        info!("Initializing SigmaOS-activemq");
        if self.config.cluster_mode {
            info!("Cluster mode enabled");
        }
        Ok(())
    }

    pub fn deploy(&mut self) -> Result<()> {
        info!("Deploying SigmaOS-activemq");
        Ok(())
    }
}
