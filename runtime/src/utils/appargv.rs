use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};

use crate::{
    ctx::appcontext::AppContext, lib::{auth::user::{auth_user_commands, UserCommands}, organization::{auth_organization_commands, OrganizationCommands}}
};

#[derive(Debug, Parser, Deserialize, Serialize, Clone)]
#[command(author, version, about)]
pub struct AppArgv {
    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Indicates the program was started as a daemon
    #[arg(long = "deamon", short = 'd')]
    pub started_as_deamon: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand, Clone, Serialize, Deserialize)]
pub enum Commands {
    Users {
        #[command(subcommand)]
        command: UserCommands,
    },
    Organizations {
        #[command(subcommand)]
        command: OrganizationCommands,
    },
}

impl AppArgv {
    pub async fn match_commands(&self, ctx: Arc<AppContext>) -> Result<()> {
        if (self.command.is_none()) {
            return Ok(());
        }
        match &self.command {
            Some(Commands::Users { command }) => {
                auth_user_commands(ctx.clone(), command.clone()).await
            }
            Some(Commands::Organizations { command }) => {
                auth_organization_commands(ctx.clone(), command.clone()).await
            }
            None => todo!("Command not found"),
            /*
                        Commands::Organizations { command } => match command {
                            OrganizationCommands::Create { name } => create_organization(name),
                            OrganizationCommands::Delete { name } => delete_organization(name),
                            OrganizationCommands::List => list_organizations(),
                        },
                        Commands::Products { command } => match command {
                            ProductCommands::Add { name } => add_product(name),
                            ProductCommands::Remove { name } => remove_product(name),
                            ProductCommands::List => list_products(),
                        },
            */
        }
    }
}
