use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};

use crate::{
    auth::{
        team::{TeamCommands, auth_team_commands},
        user::{UserCommands, auth_user_commands},
    },
    ctx::appcontext::AppContext,
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
    Teams {
        #[command(subcommand)]
        command: TeamCommands,
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
            Some(Commands::Teams { command }) => {
                auth_team_commands(ctx.clone(), command.clone()).await
            }
            None => todo!("Command not found"),
            /*
                        Commands::Teams { command } => match command {
                            TeamCommands::Create { name } => create_team(name),
                            TeamCommands::Delete { name } => delete_team(name),
                            TeamCommands::List => list_teams(),
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
