/**
 * Copyright (c) 2023 hapenia
 *
 * This software is released under the MIT License.
 * https://opensource.org/licenses/MIT
 */
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// install godot.
    Install {
        #[arg(short, long)]
        version: String,
    },
    /// Set `version` as current Godot version.
    Set {
        #[arg(short, long)]
        version: String,
    },
    /// Updates the godot package list.
    Update,
    Upgrade,
}
