use clap::{Parser, Subcommand};
use tokio::main;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Login {
        #[arg(long)]
        account_type: String,
    },
    ListMinecraftVersions,
    DownloadMinecraft,
    Launch,
    Configs {
        /// List all configurations.
        #[arg(long, alias = "ls")]
        list: (),
    },
}

#[main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Login { account_type }) => {
            println!("Login with account type: {account_type}")
        }
        Some(Commands::ListMinecraftVersions) => {
            println!("Minecraft versions are listed below")
        }
        Some(Commands::DownloadMinecraft) => {
            println!("Download Minecraft")
        }
        Some(Commands::Launch) => {
            println!("Launch Minecraft")
        }
        Some(Commands::Configs { list }) => {
            println!("All configurations are listed here")
        }
        _ => eprintln!("Invalid command!"),
    }
}
