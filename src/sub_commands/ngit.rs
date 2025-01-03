#![cfg_attr(not(test), warn(clippy::pedantic))]
#![cfg_attr(not(test), warn(clippy::expect_used))]
use clap::{Args, Parser};
//use ngit::sub_commands::*;
use ngit::Cli;
use ngit::Commands;
use nostr_sdk::prelude::*;

#[derive(Args)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct NgitSubCommand {
    #[command(subcommand)]
    command: Commands,
    /// nsec or hex private key
    #[arg(short, long, global = true)]
    nsec: Option<String>,
    /// password to decrypt nsec
    #[arg(short, long, global = true)]
    password: Option<String>,
    /// disable spinner animations
    #[arg(long, action)]
    disable_cli_spinners: bool,
}

pub async fn ngit(sub_command_args: &NgitSubCommand) -> Result<()> {
    let cli = Cli::parse();
    println!("{:?}", cli);
    match &sub_command_args.command {
        Commands::Login(args) => ngit::sub_commands::login::launch(&cli, &args).await?,
        Commands::Init(args) => ngit::sub_commands::init::launch(&cli, &&args).await?,
        Commands::Send(args) => ngit::sub_commands::send::launch(&cli, &args).await?,
        Commands::List => ngit::sub_commands::list::launch().await?,
        Commands::Pull => ngit::sub_commands::pull::launch().await?,
        Commands::Push(args) => ngit::sub_commands::push::launch(&cli, &args).await?,
    }
    Ok(())
}

//#[tokio::main]
//async fn main() -> Result<()> {
//    let cli = Cli::parse();
//    match &cli.command {
//        Commands::Login(args) => sub_commands::login::launch(&cli, args).await,
//        Commands::Init(args) => sub_commands::init::launch(&cli, args).await,
//        Commands::Send(args) => sub_commands::send::launch(&cli, args).await,
//        Commands::List => sub_commands::list::launch().await,
//        Commands::Pull => sub_commands::pull::launch().await,
//        Commands::Push(args) => sub_commands::push::launch(&cli, args).await,
//    }
//}
