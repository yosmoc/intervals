use clap::{Parser, Subcommand};
use intervals_cli::client::ApiClient;
use intervals_cli::commands::{get_activity, get_athlete};

const DEFAULT_BASE_URL: &str = "https://intervals.icu";

#[derive(Parser)]
#[command(name = "intervals-cli")]
#[command(about = "CLI for Intervals.icu API")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, default_value = DEFAULT_BASE_URL, env = "INTERVALS_BASE_URL")]
    base_url: String,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Get athlete profile")]
    GetAthlete {
        #[arg(help = "Athlete ID")]
        id: String,
    },
    #[command(about = "Get an activity")]
    GetActivity {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let client = ApiClient::from_env(cli.base_url)?;

    match cli.command {
        Commands::GetAthlete { id } => {
            let athlete = get_athlete::get_athlete(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&athlete)?);
        }
        Commands::GetActivity { id, activity_id } => {
            let activity = get_activity::get_activity(&client, &id, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&activity)?);
        }
    }

    Ok(())
}
