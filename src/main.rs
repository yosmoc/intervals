use clap::{Parser, Subcommand};
use intervals::client::ApiClient;
use intervals::commands::{
    athlete_settings, create_event, create_manual_activity, custom_items, delete_activity,
    download_activity_file, folder_operations, get_activities, get_activity,
    get_activity_best_efforts, get_activity_map, get_activity_segments, get_activity_streams,
    get_activity_weather_summary, get_athlete, get_athlete_models, get_athlete_profile,
    get_athlete_summary, get_athlete_training_plan, get_delete_event, get_interval_stats,
    get_route, get_update_weather_config, get_weather_forecast, get_wellness, get_workout,
    list_activities, list_activity_intervals, list_activity_messages, list_activity_tags,
    list_athlete_hr_curves, list_athlete_pace_curves, list_athlete_power_curves,
    list_athlete_routes, list_chats, list_event_workout_tags, list_events, list_folders, list_gear,
    list_sport_settings, list_wellness, list_workouts, mark_event_done, misc_endpoints,
    post_activity_message, search_activities, update_activity,
};

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
    #[command(about = "Get athlete settings for a device class")]
    GetAthleteSettings {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(help = "Device class (phone, tablet, desktop)")]
        device_class: String,
    },
    #[command(about = "Apply plan changes to calendar")]
    ApplyPlanChanges {
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
    #[command(about = "Fetch multiple activities by id")]
    GetActivities {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(help = "Activity IDs (comma-separated)")]
        ids: String,
        #[arg(long, help = "Include interval data", default_value = "false")]
        intervals: bool,
    },
    #[command(about = "Find best efforts in an activity")]
    GetActivityBestEfforts {
        #[arg(help = "Activity ID")]
        activity_id: String,
        #[arg(help = "Stream to search (e.g., watts, speed, heartrate)")]
        stream: String,
        #[arg(long, help = "Duration of each effort in seconds")]
        duration: Option<i64>,
        #[arg(long, help = "Distance of each effort in meters")]
        distance: Option<f64>,
        #[arg(long, help = "Number of efforts to return")]
        count: Option<i64>,
    },
    #[command(about = "Get interval stats for part of an activity")]
    GetIntervalStats {
        #[arg(help = "Activity ID")]
        activity_id: String,
        #[arg(help = "Start index")]
        start_index: i64,
        #[arg(help = "End index")]
        end_index: i64,
    },
    #[command(about = "Get activity streams (time series data)")]
    GetActivityStreams {
        #[arg(help = "Activity ID")]
        activity_id: String,
        #[arg(
            long,
            help = "Stream types to include (e.g., heartrate, watts, cadence)"
        )]
        types: Option<Vec<String>>,
        #[arg(long, help = "Include default streams")]
        include_defaults: bool,
    },
    #[command(about = "Get activity map data")]
    GetActivityMap {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "Get activity weather summary")]
    GetActivityWeatherSummary {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "Get activity segments")]
    GetActivitySegments {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "Get an event (planned workout, note etc.)")]
    GetEvent {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Event ID")]
        event_id: i64,
    },
    #[command(about = "Delete an event from athlete's calendar")]
    DeleteEvent {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Event ID")]
        event_id: i64,
    },
    #[command(about = "Update an event (planned workout, note etc.)")]
    UpdateEvent {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Event ID")]
        event_id: i64,
        #[arg(long, help = "Event name")]
        name: Option<String>,
        #[arg(long, help = "Event description")]
        description: Option<String>,
        #[arg(long, help = "Event notes")]
        notes: Option<String>,
    },
    #[command(about = "Mark event as done (create manual activity from planned workout)")]
    MarkEventDone {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Event ID")]
        event_id: i64,
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
    #[command(about = "Get wellness record for a specific date")]
    GetWellness {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(help = "Date (ISO-8601)")]
        date: String,
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
    #[command(about = "List activity tags for an athlete")]
    ListActivityTags {
        #[arg(help = "Athlete ID")]
        id: String,
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
    #[command(about = "List event tags for an athlete")]
    ListEventTags {
        #[arg(help = "Athlete ID")]
        id: String,
    },
    #[command(about = "List workout tags for an athlete")]
    ListWorkoutTags {
        #[arg(help = "Athlete ID")]
        id: String,
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
    #[command(about = "Download original activity file")]
    DownloadActivityFile {
        #[arg(help = "Activity ID")]
        activity_id: String,
        #[arg(help = "Output file path")]
        output: String,
    },
    #[command(about = "Download FIT file for an activity")]
    DownloadActivityFitFile {
        #[arg(help = "Activity ID")]
        activity_id: String,
        #[arg(help = "Output file path")]
        output: String,
        #[arg(long, help = "Include power data", default_value = "true")]
        power: bool,
        #[arg(long, help = "Include heart rate data", default_value = "true")]
        hr: bool,
    },
    #[command(about = "Download GPX file for an activity")]
    DownloadActivityGpxFile {
        #[arg(help = "Activity ID")]
        activity_id: String,
        #[arg(help = "Output file path")]
        output: String,
        #[arg(long, help = "Include power data", default_value = "true")]
        power: bool,
        #[arg(long, help = "Include heart rate data", default_value = "true")]
        hr: bool,
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
    #[command(about = "List athletes a folder is shared with")]
    ListFolderSharedWith {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Folder ID")]
        folder_id: i64,
    },
    #[command(about = "Delete a folder and all its workouts")]
    DeleteFolder {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Folder ID")]
        folder_id: i64,
    },
    #[command(about = "Create a calendar event (workout, note, race)")]
    CreateEvent {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(long, help = "Start date/time (ISO-8601), required")]
        start_date_local: String,
        #[arg(long, help = "Event type (WORKOUT,NOTE,RACE_A,etc.), required")]
        event_type: String,
        #[arg(long, help = "Category (WORKOUT,NOTE,RACE_A,etc.), required")]
        category: String,
        #[arg(long, help = "Event name")]
        name: Option<String>,
        #[arg(long, help = "Event description or workout definition")]
        description: Option<String>,
        #[arg(long, help = "Unique identifier for upsert")]
        uid: Option<String>,
        #[arg(long, help = "Calendar ID")]
        calendar_id: Option<i32>,
        #[arg(long, help = "Update existing event with matching uid")]
        upsert_on_uid: bool,
    },
    #[command(about = "Get a workout from library")]
    GetWorkout {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(help = "Workout ID")]
        workout_id: i32,
    },
    #[command(about = "List athlete power curves")]
    ListAthletePowerCurves {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(help = "Activity type (Ride, Run, etc.)")]
        activity_type: String,
    },
    #[command(about = "List athlete routes")]
    ListAthleteRoutes {
        #[arg(help = "Athlete ID")]
        id: String,
    },
    #[command(about = "Get a route for an athlete")]
    GetRoute {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Route ID")]
        route_id: i64,
        #[arg(long, help = "Include GPS path", default_value = "false")]
        include_path: bool,
    },
    #[command(about = "Get athlete profile")]
    GetAthleteProfile {
        #[arg(help = "Athlete ID")]
        id: String,
    },
    #[command(about = "Get athlete summary (fitness data)")]
    GetAthleteSummary {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(long, help = "Start date (ISO-8601)")]
        start: Option<String>,
        #[arg(long, help = "End date (ISO-8601)")]
        end: Option<String>,
    },
    #[command(about = "List activity messages")]
    ListActivityMessages {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "Post a message to an activity")]
    PostActivityMessage {
        #[arg(help = "Activity ID")]
        activity_id: String,
        #[arg(help = "Message content")]
        content: String,
    },
    #[command(about = "Get weather forecast")]
    GetWeatherForecast {
        #[arg(help = "Athlete ID")]
        id: String,
    },
    #[command(about = "Get weather forecast configuration")]
    GetWeatherConfig {
        #[arg(help = "Athlete ID")]
        id: String,
    },
    #[command(about = "Update weather forecast configuration")]
    UpdateWeatherConfig {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(long, help = "Forecasts config (JSON)")]
        forecasts: Option<String>,
    },
    #[command(about = "List sport settings")]
    ListSportSettings {
        #[arg(help = "Athlete ID")]
        id: String,
    },
    #[command(about = "List chats")]
    ListChats {
        #[arg(help = "Athlete ID")]
        id: String,
    },
    #[command(about = "List custom items for an athlete")]
    ListCustomItems {
        #[arg(help = "Athlete ID")]
        id: String,
    },
    #[command(about = "Get a custom item")]
    GetCustomItem {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Item ID")]
        item_id: i64,
    },
    #[command(about = "Delete a custom item")]
    DeleteCustomItem {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Item ID")]
        item_id: i64,
    },
    #[command(about = "List athlete HR curves")]
    ListAthleteHrCurves {
        #[arg(help = "Athlete ID")]
        id: String,
    },
    #[command(about = "List athlete pace curves")]
    ListAthletePaceCurves {
        #[arg(help = "Athlete ID")]
        id: String,
    },
    #[command(about = "Get athlete training plan")]
    GetAthleteTrainingPlan {
        #[arg(help = "Athlete ID")]
        id: String,
    },
    #[command(about = "Get athlete MMP model for an activity type")]
    GetAthleteMmpModel {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(help = "Activity type (e.g., Ride, Run)")]
        activity_type: String,
    },
    #[command(about = "Get athlete power vs heart rate curve")]
    GetPowerHrCurve {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(long, help = "Start date (ISO-8601)")]
        start: String,
        #[arg(long, help = "End date (ISO-8601)")]
        end: String,
    },
    #[command(about = "List activity intervals")]
    ListActivityIntervals {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "Disconnect the app")]
    DisconnectApp,
    #[command(about = "Get a shared event")]
    GetSharedEvent {
        #[arg(help = "Event ID")]
        event_id: i64,
    },
    #[command(about = "List pace distances")]
    ListPaceDistances,
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
        Commands::GetAthleteSettings { id, device_class } => {
            let settings =
                athlete_settings::get_athlete_settings(&client, &id, &device_class).await?;
            println!("{}", serde_json::to_string_pretty(&settings)?);
        }
        Commands::ApplyPlanChanges { id } => {
            let result = athlete_settings::apply_plan_changes(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::GetActivity { id, activity_id } => {
            let activity = get_activity::get_activity(&client, &id, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&activity)?);
        }
        Commands::GetActivities { id, ids, intervals } => {
            let ids_list: Vec<String> = ids.split(',').map(|s| s.trim().to_string()).collect();
            let activities =
                get_activities::get_activities(&client, &id, &ids_list, intervals).await?;
            println!("{}", serde_json::to_string_pretty(&activities)?);
        }
        Commands::GetActivityBestEfforts {
            activity_id,
            stream,
            duration,
            distance,
            count,
        } => {
            let params = get_activity_best_efforts::BestEffortsParams {
                stream,
                duration,
                distance,
                count,
                min_value: None,
                exclude_intervals: None,
                start_index: None,
                end_index: None,
            };
            let efforts = get_activity_best_efforts::get_activity_best_efforts(
                &client,
                &activity_id,
                &params,
            )
            .await?;
            println!("{}", serde_json::to_string_pretty(&efforts)?);
        }
        Commands::GetIntervalStats {
            activity_id,
            start_index,
            end_index,
        } => {
            let stats = get_interval_stats::get_interval_stats(
                &client,
                &activity_id,
                start_index,
                end_index,
            )
            .await?;
            println!("{}", serde_json::to_string_pretty(&stats)?);
        }
        Commands::GetActivityStreams {
            activity_id,
            types,
            include_defaults,
        } => {
            let streams = get_activity_streams::get_activity_streams(
                &client,
                &activity_id,
                types.as_deref(),
                include_defaults,
            )
            .await?;
            println!("{}", serde_json::to_string_pretty(&streams)?);
        }
        Commands::GetActivityMap { activity_id } => {
            let map = get_activity_map::get_activity_map(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&map)?);
        }
        Commands::GetActivityWeatherSummary { activity_id } => {
            let summary =
                get_activity_weather_summary::get_activity_weather_summary(&client, &activity_id)
                    .await?;
            println!("{}", serde_json::to_string_pretty(&summary)?);
        }
        Commands::GetActivitySegments { activity_id } => {
            let segments =
                get_activity_segments::get_activity_segments(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&segments)?);
        }
        Commands::GetEvent {
            athlete_id,
            event_id,
        } => {
            let event = get_delete_event::get_event(&client, &athlete_id, event_id).await?;
            println!("{}", serde_json::to_string_pretty(&event)?);
        }
        Commands::DeleteEvent {
            athlete_id,
            event_id,
        } => {
            get_delete_event::delete_event(&client, &athlete_id, event_id).await?;
            println!("Event deleted successfully");
        }
        Commands::UpdateEvent {
            athlete_id,
            event_id,
            name,
            description,
            notes,
        } => {
            let update = get_delete_event::EventEx {
                id: Some(event_id),
                start_date_local: None,
                event_type: None,
                category: None,
                name,
                description,
                uid: None,
                notes,
                workout: None,
            };
            let event =
                get_delete_event::update_event(&client, &athlete_id, event_id, &update).await?;
            println!("{}", serde_json::to_string_pretty(&event)?);
        }
        Commands::MarkEventDone {
            athlete_id,
            event_id,
        } => {
            let activity = mark_event_done::mark_event_done(&client, &athlete_id, event_id).await?;
            println!("{}", serde_json::to_string_pretty(&activity)?);
        }
        Commands::ListWellness { id, oldest, newest } => {
            let params = list_wellness::ListWellnessParams { oldest, newest };
            let records = list_wellness::list_wellness(&client, &id, &params).await?;
            println!("{}", serde_json::to_string_pretty(&records)?);
        }
        Commands::GetWellness { id, date } => {
            let record = get_wellness::get_wellness(&client, &id, &date).await?;
            println!("{}", serde_json::to_string_pretty(&record)?);
        }
        Commands::ListActivities {
            id,
            oldest,
            newest,
            route_id,
            limit,
        } => {
            let params = list_activities::ListActivitiesParams {
                oldest,
                newest,
                route_id,
                limit,
            };
            let activities = list_activities::list_activities(&client, &id, &params).await?;
            println!("{}", serde_json::to_string_pretty(&activities)?);
        }
        Commands::ListActivityTags { id } => {
            let tags = list_activity_tags::list_activity_tags(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&tags)?);
        }
        Commands::ListWorkouts { id } => {
            let workouts = list_workouts::list_workouts(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&workouts)?);
        }
        Commands::ListEvents {
            id,
            oldest,
            newest,
            category,
            limit,
        } => {
            let params = list_events::ListEventsParams {
                oldest,
                newest,
                category,
                limit,
            };
            let events = list_events::list_events(&client, &id, &params).await?;
            println!("{}", serde_json::to_string_pretty(&events)?);
        }
        Commands::ListEventTags { id } => {
            let tags = list_event_workout_tags::list_event_tags(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&tags)?);
        }
        Commands::ListWorkoutTags { id } => {
            let tags = list_event_workout_tags::list_workout_tags(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&tags)?);
        }
        Commands::UpdateActivity {
            activity_id,
            name,
            description,
            activity_type,
            sport,
            distance,
            elapsed_time,
        } => {
            let input = update_activity::UpdateActivityInput {
                name,
                description,
                activity_type,
                sport,
                distance,
                elapsed_time,
            };
            let activity = update_activity::update_activity(&client, &activity_id, &input).await?;
            println!("{}", serde_json::to_string_pretty(&activity)?);
        }
        Commands::CreateManualActivity {
            id,
            start_date_local,
            activity_type,
            name,
            description,
            sport,
            distance,
            elapsed_time,
        } => {
            let input = create_manual_activity::CreateManualActivityInput {
                start_date_local,
                activity_type,
                name,
                description,
                sport,
                distance,
                elapsed_time,
            };
            let activity =
                create_manual_activity::create_manual_activity(&client, &id, &input).await?;
            println!("{}", serde_json::to_string_pretty(&activity)?);
        }
        Commands::DeleteActivity { activity_id } => {
            let result = delete_activity::delete_activity(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::DownloadActivityFile {
            activity_id,
            output,
        } => {
            download_activity_file::download_activity_file(&client, &activity_id, &output).await?;
            println!("File downloaded to {}", output);
        }
        Commands::DownloadActivityFitFile {
            activity_id,
            output,
            power,
            hr,
        } => {
            download_activity_file::download_activity_fit_file(
                &client,
                &activity_id,
                &output,
                power,
                hr,
            )
            .await?;
            println!("FIT file downloaded to {}", output);
        }
        Commands::DownloadActivityGpxFile {
            activity_id,
            output,
            power,
            hr,
        } => {
            download_activity_file::download_activity_gpx_file(
                &client,
                &activity_id,
                &output,
                power,
                hr,
            )
            .await?;
            println!("GPX file downloaded to {}", output);
        }
        Commands::ListGear { id } => {
            let gear = list_gear::list_gear(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&gear)?);
        }
        Commands::SearchActivities {
            id,
            query,
            limit,
            full,
        } => {
            let params = search_activities::SearchActivitiesParams { query, limit, full };
            let results = search_activities::search_activities(&client, &id, &params).await?;
            println!("{}", serde_json::to_string_pretty(&results)?);
        }
        Commands::ListFolders { id } => {
            let folders = list_folders::list_folders(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&folders)?);
        }
        Commands::ListFolderSharedWith {
            athlete_id,
            folder_id,
        } => {
            let shared =
                folder_operations::list_folder_shared_with(&client, &athlete_id, folder_id).await?;
            println!("{}", serde_json::to_string_pretty(&shared)?);
        }
        Commands::DeleteFolder {
            athlete_id,
            folder_id,
        } => {
            folder_operations::delete_folder(&client, &athlete_id, folder_id).await?;
            println!("Folder deleted successfully");
        }
        Commands::CreateEvent {
            id,
            start_date_local,
            event_type,
            category,
            name,
            description,
            uid,
            calendar_id,
            upsert_on_uid,
        } => {
            let input = create_event::CreateEventInput {
                start_date_local,
                event_type,
                category,
                name,
                description,
                uid,
                calendar_id,
            };
            let event = create_event::create_event(&client, &id, &input, upsert_on_uid).await?;
            println!("{}", serde_json::to_string_pretty(&event)?);
        }
        Commands::GetWorkout { id, workout_id } => {
            let workout = get_workout::get_workout(&client, &id, workout_id).await?;
            println!("{}", serde_json::to_string_pretty(&workout)?);
        }
        Commands::ListAthletePowerCurves { id, activity_type } => {
            let curves =
                list_athlete_power_curves::list_athlete_power_curves(&client, &id, &activity_type)
                    .await?;
            println!("{}", serde_json::to_string_pretty(&curves)?);
        }
        Commands::ListAthleteRoutes { id } => {
            let routes = list_athlete_routes::list_athlete_routes(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&routes)?);
        }
        Commands::GetRoute {
            athlete_id,
            route_id,
            include_path,
        } => {
            let route = get_route::get_route(&client, &athlete_id, route_id, include_path).await?;
            println!("{}", serde_json::to_string_pretty(&route)?);
        }
        Commands::GetAthleteProfile { id } => {
            let profile = get_athlete_profile::get_athlete_profile(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&profile)?);
        }
        Commands::GetAthleteSummary { id, start, end } => {
            let params = get_athlete_summary::GetAthleteSummaryParams {
                start,
                end,
                tags: None,
            };
            let summaries = get_athlete_summary::get_athlete_summary(&client, &id, &params).await?;
            println!("{}", serde_json::to_string_pretty(&summaries)?);
        }
        Commands::ListActivityMessages { activity_id } => {
            let messages =
                list_activity_messages::list_activity_messages(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&messages)?);
        }
        Commands::PostActivityMessage {
            activity_id,
            content,
        } => {
            let result =
                post_activity_message::post_activity_message(&client, &activity_id, &content)
                    .await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::GetWeatherForecast { id } => {
            let forecast = get_weather_forecast::get_weather_forecast(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&forecast)?);
        }
        Commands::GetWeatherConfig { id } => {
            let config = get_update_weather_config::get_weather_config(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&config)?);
        }
        Commands::UpdateWeatherConfig { id, forecasts } => {
            let forecasts_parsed = forecasts
                .map(|f| serde_json::from_str(&f))
                .transpose()?
                .unwrap_or_default();
            let config = get_update_weather_config::WeatherConfig {
                forecasts: Some(forecasts_parsed),
            };
            let result =
                get_update_weather_config::update_weather_config(&client, &id, &config).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::ListSportSettings { id } => {
            let settings = list_sport_settings::list_sport_settings(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&settings)?);
        }
        Commands::ListChats { id } => {
            let chats = list_chats::list_chats(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&chats)?);
        }
        Commands::ListCustomItems { id } => {
            let items = custom_items::list_custom_items(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&items)?);
        }
        Commands::GetCustomItem {
            athlete_id,
            item_id,
        } => {
            let item = custom_items::get_custom_item(&client, &athlete_id, item_id).await?;
            println!("{}", serde_json::to_string_pretty(&item)?);
        }
        Commands::DeleteCustomItem {
            athlete_id,
            item_id,
        } => {
            custom_items::delete_custom_item(&client, &athlete_id, item_id).await?;
            println!("Custom item deleted successfully");
        }
        Commands::ListAthleteHrCurves { id } => {
            let curves = list_athlete_hr_curves::list_athlete_hr_curves(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&curves)?);
        }
        Commands::ListAthletePaceCurves { id } => {
            let curves = list_athlete_pace_curves::list_athlete_pace_curves(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&curves)?);
        }
        Commands::GetAthleteTrainingPlan { id } => {
            let plan = get_athlete_training_plan::get_athlete_training_plan(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&plan)?);
        }
        Commands::GetAthleteMmpModel { id, activity_type } => {
            let model =
                get_athlete_models::get_athlete_mmp_model(&client, &id, &activity_type).await?;
            println!("{}", serde_json::to_string_pretty(&model)?);
        }
        Commands::GetPowerHrCurve { id, start, end } => {
            let curve = get_athlete_models::get_power_hr_curve(&client, &id, &start, &end).await?;
            println!("{}", serde_json::to_string_pretty(&curve)?);
        }
        Commands::ListActivityIntervals { activity_id } => {
            let intervals =
                list_activity_intervals::list_activity_intervals(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&intervals)?);
        }
        Commands::DisconnectApp => {
            misc_endpoints::disconnect_app(&client).await?;
            println!("App disconnected successfully");
        }
        Commands::GetSharedEvent { event_id } => {
            let event = misc_endpoints::get_shared_event(&client, event_id).await?;
            println!("{}", serde_json::to_string_pretty(&event)?);
        }
        Commands::ListPaceDistances => {
            let distances = misc_endpoints::list_pace_distances(&client).await?;
            println!("{}", serde_json::to_string_pretty(&distances)?);
        }
    }

    Ok(())
}
