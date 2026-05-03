use clap::{Parser, Subcommand};
use intervals::client::ApiClient;
use intervals::commands::{
    create_event, create_manual_activity, delete_activity, download_activity_file, get_activity,
    get_activity_best_efforts, get_activity_map, get_activity_segments, get_activity_streams,
    get_activity_weather_summary, get_athlete, get_athlete_profile, get_athlete_summary,
    get_athlete_training_plan, get_delete_event, get_route, get_weather_forecast, get_workout,
    list_activities, list_activity_intervals, list_activity_messages, list_athlete_hr_curves,
    list_athlete_pace_curves, list_athlete_power_curves, list_athlete_routes, list_chats,
    list_events, list_folders, list_gear, list_sport_settings, list_wellness, list_workouts,
    mark_event_done, post_activity_message, search_activities, update_activity,
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
    #[command(about = "Get an activity")]
    GetActivity {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(help = "Activity ID")]
        activity_id: String,
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
    #[command(about = "List activity intervals")]
    ListActivityIntervals {
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
        Commands::ListSportSettings { id } => {
            let settings = list_sport_settings::list_sport_settings(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&settings)?);
        }
        Commands::ListChats { id } => {
            let chats = list_chats::list_chats(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&chats)?);
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
        Commands::ListActivityIntervals { activity_id } => {
            let intervals =
                list_activity_intervals::list_activity_intervals(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&intervals)?);
        }
    }

    Ok(())
}
