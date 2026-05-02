use clap::{Parser, Subcommand};
use intervals_cli::client::ApiClient;
use intervals_cli::commands::{create_manual_activity, delete_activity, get_activity, get_athlete, list_activities, list_events, list_folders, list_gear, list_wellness, list_workouts, search_activities, update_activity};

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
    #[command(about = "List wellness records for a date range")]
    ListWellness {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(long, help = "Oldest date (ISO-8601)")]
        oldest: Option<String>,
        #[arg(long, help = "Newest date (ISO-8601)")]
        newest: Option<String>,
    },
    #[command(about = "List activities for a date range")]
    ListActivities {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(long, help = "Oldest date (ISO-8601), required")]
        oldest: String,
        #[arg(long, help = "Newest date (ISO-8601)")]
        newest: Option<String>,
        #[arg(long, help = "Filter by route ID")]
        route_id: Option<i64>,
        #[arg(long, help = "Maximum number of activities")]
        limit: Option<i32>,
    },
    #[command(about = "List workouts in athlete's library")]
    ListWorkouts {
        #[arg(help = "Athlete ID")]
        id: String,
    },
    #[command(about = "List calendar events (planned workouts, notes)")]
    ListEvents {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(long, help = "Oldest date (ISO-8601)")]
        oldest: Option<String>,
        #[arg(long, help = "Newest date (ISO-8601)")]
        newest: Option<String>,
        #[arg(long, help = "Filter by category (WORKOUT,NOTE,RACE_A,etc.)")]
        category: Option<String>,
        #[arg(long, help = "Maximum number of events")]
        limit: Option<i32>,
    },
    #[command(about = "Update an activity")]
    UpdateActivity {
        #[arg(help = "Activity ID")]
        activity_id: String,
        #[arg(long, help = "Activity name")]
        name: Option<String>,
        #[arg(long, help = "Activity description")]
        description: Option<String>,
        #[arg(long, help = "Activity type")]
        activity_type: Option<String>,
        #[arg(long, help = "Sport")]
        sport: Option<String>,
        #[arg(long, help = "Distance in meters")]
        distance: Option<f64>,
        #[arg(long, help = "Elapsed time in seconds")]
        elapsed_time: Option<i64>,
    },
    #[command(about = "Create a manual activity")]
    CreateManualActivity {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(long, help = "Start date/time (ISO-8601), required")]
        start_date_local: String,
        #[arg(long, help = "Activity type (Ride, Run, etc.), required")]
        activity_type: String,
        #[arg(long, help = "Activity name")]
        name: Option<String>,
        #[arg(long, help = "Activity description")]
        description: Option<String>,
        #[arg(long, help = "Sport")]
        sport: Option<String>,
        #[arg(long, help = "Distance in meters")]
        distance: Option<f64>,
        #[arg(long, help = "Elapsed time in seconds")]
        elapsed_time: Option<i64>,
    },
    #[command(about = "Delete an activity")]
    DeleteActivity {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "List athlete gear")]
    ListGear {
        #[arg(help = "Athlete ID")]
        id: String,
    },
    #[command(about = "Search activities by name or tag")]
    SearchActivities {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(help = "Search query (use # for tag search)")]
        query: String,
        #[arg(long, help = "Maximum number of results")]
        limit: Option<i32>,
        #[arg(long, help = "Return full activity details")]
        full: bool,
    },
    #[command(about = "List workout folders, plans, and workouts")]
    ListFolders {
        #[arg(help = "Athlete ID")]
        id: String,
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
        Commands::ListWellness { id, oldest, newest } => {
            let params = list_wellness::ListWellnessParams { oldest, newest };
            let records = list_wellness::list_wellness(&client, &id, &params).await?;
            println!("{}", serde_json::to_string_pretty(&records)?);
        }
        Commands::ListActivities { id, oldest, newest, route_id, limit } => {
            let params = list_activities::ListActivitiesParams { oldest, newest, route_id, limit };
            let activities = list_activities::list_activities(&client, &id, &params).await?;
            println!("{}", serde_json::to_string_pretty(&activities)?);
        }
        Commands::ListWorkouts { id } => {
            let workouts = list_workouts::list_workouts(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&workouts)?);
        }
        Commands::ListEvents { id, oldest, newest, category, limit } => {
            let params = list_events::ListEventsParams { oldest, newest, category, limit };
            let events = list_events::list_events(&client, &id, &params).await?;
            println!("{}", serde_json::to_string_pretty(&events)?);
        }
        Commands::UpdateActivity { activity_id, name, description, activity_type, sport, distance, elapsed_time } => {
            let input = update_activity::UpdateActivityInput { name, description, activity_type, sport, distance, elapsed_time };
            let activity = update_activity::update_activity(&client, &activity_id, &input).await?;
            println!("{}", serde_json::to_string_pretty(&activity)?);
        }
        Commands::CreateManualActivity { id, start_date_local, activity_type, name, description, sport, distance, elapsed_time } => {
            let input = create_manual_activity::CreateManualActivityInput { start_date_local, activity_type, name, description, sport, distance, elapsed_time };
            let activity = create_manual_activity::create_manual_activity(&client, &id, &input).await?;
            println!("{}", serde_json::to_string_pretty(&activity)?);
        }
        Commands::DeleteActivity { activity_id } => {
            let result = delete_activity::delete_activity(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::ListGear { id } => {
            let gear = list_gear::list_gear(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&gear)?);
        }
        Commands::SearchActivities { id, query, limit, full } => {
            let params = search_activities::SearchActivitiesParams { query, limit, full };
            let results = search_activities::search_activities(&client, &id, &params).await?;
            println!("{}", serde_json::to_string_pretty(&results)?);
        }
        Commands::ListFolders { id } => {
            let folders = list_folders::list_folders(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&folders)?);
        }
    }

    Ok(())
}
