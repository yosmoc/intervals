use clap::{Parser, Subcommand};
use intervals::client::ApiClient;
use intervals::commands::{
    activity_analysis, activity_interval_editing, activity_upload, athlete_settings,
    chat_and_fitness, create_event, create_manual_activity, csv_and_wellness, custom_items,
    delete_activity, download_activity_file, folder_operations, gear_operations, get_activities,
    get_activity, get_activity_best_efforts, get_activity_map, get_activity_segments,
    get_activity_streams, get_activity_weather_summary, get_athlete, get_athlete_models,
    get_athlete_profile, get_athlete_summary, get_athlete_training_plan, get_delete_event,
    get_interval_stats, get_route, get_update_weather_config, get_weather_forecast, get_wellness,
    get_workout, list_activities, list_activities_around, list_activity_intervals,
    list_activity_messages, list_activity_tags, list_athlete_hr_curves, list_athlete_pace_curves,
    list_athlete_power_curves, list_athlete_routes, list_chats, list_event_workout_tags,
    list_events, list_folders, list_gear, list_sport_settings, list_wellness, list_workouts,
    mark_event_done, misc_endpoints, post_activity_message, search_activities,
    search_activities_full, search_activity_intervals, update_activity,
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
    #[command(about = "Get activity heart rate curve")]
    GetActivityHRCurve {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "Get activity heart rate histogram")]
    GetActivityHRHistogram {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "Get activity heart rate load model")]
    GetActivityHRLoadModel {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "Get activity pace curve")]
    GetActivityPaceCurve {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "Get activity pace histogram")]
    GetActivityPaceHistogram {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "Get activity power curves")]
    GetActivityPowerCurves {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "Get activity power curve")]
    GetActivityPowerCurve {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "Get activity power histogram")]
    GetActivityPowerHistogram {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "Get activity power spike model")]
    GetActivityPowerSpikeModel {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "Get activity power vs heart rate")]
    GetActivityPowerVsHR {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "Get activity gradient-adjusted pace histogram")]
    GetActivityGAPHistogram {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "Get activity time at heart rate")]
    GetActivityTimeAtHR {
        #[arg(help = "Activity ID")]
        activity_id: String,
    },
    #[command(about = "List best heart rate curves for athlete")]
    ListAthleteHRCurvesBest {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
    },
    #[command(about = "List best power curves for athlete")]
    ListAthletePowerCurvesBest {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
    },
    #[command(about = "List best pace curves for athlete")]
    ListAthletePaceCurvesBest {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
    },
    #[command(about = "Update intervals for an activity")]
    UpdateActivityIntervals {
        #[arg(help = "Activity ID")]
        activity_id: String,
        #[arg(long, help = "Intervals as JSON array")]
        intervals: String,
    },
    #[command(about = "Update/create an activity interval")]
    UpdateActivityInterval {
        #[arg(help = "Activity ID")]
        activity_id: String,
        #[arg(help = "Interval ID")]
        interval_id: i64,
        #[arg(long, help = "Interval data as JSON")]
        data: String,
    },
    #[command(about = "Split an activity interval")]
    SplitActivityInterval {
        #[arg(help = "Activity ID")]
        activity_id: String,
        #[arg(long, help = "Interval ID to split")]
        interval_id: i64,
        #[arg(long, help = "Split index")]
        split_index: Option<i64>,
    },
    #[command(about = "Delete intervals from an activity")]
    DeleteActivityIntervals {
        #[arg(help = "Activity ID")]
        activity_id: String,
        #[arg(long, help = "Interval IDs to delete (comma-separated)")]
        interval_ids: String,
    },
    #[command(about = "Update activity streams from JSON")]
    UpdateActivityStreams {
        #[arg(help = "Activity ID")]
        activity_id: String,
        #[arg(long, help = "Stream data as JSON")]
        data: String,
    },
    #[command(about = "Update activity streams from CSV")]
    UpdateActivityStreamsCSV {
        #[arg(help = "Activity ID")]
        activity_id: String,
        #[arg(long, help = "Path to CSV file")]
        csv_file: String,
    },
    #[command(about = "Upload activity file (fit/tcx/gpx/zip)")]
    UploadActivity {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Path to activity file")]
        file: String,
    },
    #[command(about = "Bulk create manual activities")]
    CreateManualActivitiesBulk {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(long, help = "Activities as JSON array")]
        activities: String,
    },
    #[command(about = "Download all FIT files as zip")]
    DownloadActivityFitFiles {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(long, help = "Output zip file path")]
        output: String,
    },
    #[command(about = "Get a specific gear item")]
    GetGear {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Gear ID")]
        gear_id: String,
    },
    #[command(about = "Create new gear or component")]
    CreateGear {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(long, help = "Gear name")]
        name: Option<String>,
        #[arg(long, help = "Initial distance")]
        distance: Option<f64>,
        #[arg(long, help = "Retired status")]
        retired: Option<bool>,
    },
    #[command(about = "Update gear or component")]
    UpdateGear {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Gear ID")]
        gear_id: String,
        #[arg(long, help = "Gear name")]
        name: Option<String>,
        #[arg(long, help = "Distance")]
        distance: Option<f64>,
        #[arg(long, help = "Retired status")]
        retired: Option<bool>,
    },
    #[command(about = "Delete gear or component")]
    DeleteGear {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Gear ID")]
        gear_id: String,
    },
    #[command(about = "Retire and replace gear component")]
    ReplaceGear {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Gear ID")]
        gear_id: String,
        #[arg(long, help = "New gear ID")]
        new_gear_id: Option<String>,
        #[arg(long, help = "Retired date")]
        retired_date: Option<String>,
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
    #[command(about = "Update wellness record for a date")]
    UpdateWellness {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(help = "Date (ISO-8601)")]
        date: String,
        #[arg(long, help = "Weight in kg")]
        weight: Option<f64>,
        #[arg(long, help = "Resting heart rate")]
        resting_hr: Option<i64>,
        #[arg(long, help = "HRV")]
        hrv: Option<f64>,
        #[arg(long, help = "Mood (1-5)")]
        mood: Option<i64>,
        #[arg(long, help = "Fatigue (1-5)")]
        fatigue: Option<i64>,
        #[arg(long, help = "Motivation (1-5)")]
        motivation: Option<i64>,
        #[arg(long, help = "Sleep hours")]
        sleep: Option<f64>,
        #[arg(long, help = "Sleep quality (1-5)")]
        sleep_quality: Option<i64>,
    },
    #[command(about = "Download activities as CSV")]
    DownloadActivitiesCsv {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(help = "Output file path")]
        output: String,
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
    #[command(about = "List activities before and after another activity")]
    ListActivitiesAround {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(help = "Activity ID at the center")]
        activity_id: String,
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
    #[command(about = "Recalculate gear stats")]
    CalcGear {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Gear ID")]
        gear_id: String,
    },
    #[command(about = "Create a gear reminder")]
    CreateGearReminder {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Gear ID")]
        gear_id: String,
        #[arg(long, help = "Distance threshold")]
        distance: Option<f64>,
        #[arg(long, help = "Date threshold")]
        date: Option<String>,
        #[arg(long, help = "Reminder notes")]
        notes: Option<String>,
    },
    #[command(about = "Update a gear reminder")]
    UpdateGearReminder {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Gear ID")]
        gear_id: String,
        #[arg(help = "Reminder ID")]
        reminder_id: i64,
        #[arg(long, help = "Distance threshold")]
        distance: Option<f64>,
        #[arg(long, help = "Date threshold")]
        date: Option<String>,
        #[arg(long, help = "Reminder notes")]
        notes: Option<String>,
        #[arg(long, help = "Mark as done", default_value = "false")]
        done: bool,
    },
    #[command(about = "Delete a gear reminder")]
    DeleteGearReminder {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Gear ID")]
        gear_id: String,
        #[arg(help = "Reminder ID")]
        reminder_id: i64,
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
    #[command(about = "Search activities returning full activity details")]
    SearchActivitiesFull {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(help = "Search query (use # for tag search)")]
        query: String,
        #[arg(long, help = "Maximum number of results")]
        limit: Option<i32>,
    },
    #[command(about = "Find activities with intervals matching duration and intensity")]
    SearchActivityIntervals {
        #[arg(help = "Athlete ID")]
        id: String,
        #[arg(long, help = "Min interval duration in seconds")]
        min_secs: i32,
        #[arg(long, help = "Max interval duration in seconds")]
        max_secs: i32,
        #[arg(long, help = "Min intensity percentage")]
        min_intensity: i32,
        #[arg(long, help = "Max intensity percentage")]
        max_intensity: i32,
        #[arg(long, help = "Interval type (AUTO, POWER, HR, PACE)")]
        interval_type: Option<String>,
        #[arg(long, help = "Min number of matching intervals")]
        min_reps: Option<i32>,
        #[arg(long, help = "Max number of matching intervals")]
        max_reps: Option<i32>,
        #[arg(long, help = "Max results to return")]
        limit: Option<i32>,
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
    #[command(about = "Update a route for an athlete")]
    UpdateRoute {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Route ID")]
        route_id: i64,
        #[arg(long, help = "Route name")]
        name: Option<String>,
        #[arg(long, help = "Route description")]
        description: Option<String>,
        #[arg(long, help = "Mark as commute", default_value = "false")]
        commute: bool,
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
    #[command(about = "Get a chat by id")]
    GetChat {
        #[arg(help = "Chat ID")]
        chat_id: i64,
    },
    #[command(about = "List messages for a chat")]
    ListChatMessages {
        #[arg(help = "Chat ID")]
        chat_id: i64,
        #[arg(long, help = "Return messages before this ID")]
        before_id: Option<i64>,
        #[arg(long, help = "Max messages to return")]
        limit: Option<i32>,
    },
    #[command(about = "Send a chat message")]
    SendChatMessage {
        #[arg(help = "Athlete ID")]
        athlete_id: String,
        #[arg(help = "Message content")]
        content: String,
        #[arg(long, help = "Chat ID to send to")]
        chat_id: Option<i64>,
    },
    #[command(about = "List fitness model events for an athlete")]
    ListFitnessModelEvents {
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
        Commands::GetActivityHRCurve { activity_id } => {
            let curve = activity_analysis::get_activity_hr_curve(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&curve)?);
        }
        Commands::GetActivityHRHistogram { activity_id } => {
            let histogram =
                activity_analysis::get_activity_hr_histogram(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&histogram)?);
        }
        Commands::GetActivityHRLoadModel { activity_id } => {
            let model =
                activity_analysis::get_activity_hr_load_model(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&model)?);
        }
        Commands::GetActivityPaceCurve { activity_id } => {
            let curve = activity_analysis::get_activity_pace_curve(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&curve)?);
        }
        Commands::GetActivityPaceHistogram { activity_id } => {
            let histogram =
                activity_analysis::get_activity_pace_histogram(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&histogram)?);
        }
        Commands::GetActivityPowerCurves { activity_id } => {
            let curves =
                activity_analysis::get_activity_power_curves(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&curves)?);
        }
        Commands::GetActivityPowerCurve { activity_id } => {
            let curve = activity_analysis::get_activity_power_curve(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&curve)?);
        }
        Commands::GetActivityPowerHistogram { activity_id } => {
            let histogram =
                activity_analysis::get_activity_power_histogram(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&histogram)?);
        }
        Commands::GetActivityPowerSpikeModel { activity_id } => {
            let model =
                activity_analysis::get_activity_power_spike_model(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&model)?);
        }
        Commands::GetActivityPowerVsHR { activity_id } => {
            let data = activity_analysis::get_activity_power_vs_hr(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&data)?);
        }
        Commands::GetActivityGAPHistogram { activity_id } => {
            let histogram =
                activity_analysis::get_activity_gap_histogram(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&histogram)?);
        }
        Commands::GetActivityTimeAtHR { activity_id } => {
            let data = activity_analysis::get_activity_time_at_hr(&client, &activity_id).await?;
            println!("{}", serde_json::to_string_pretty(&data)?);
        }
        Commands::ListAthleteHRCurvesBest { athlete_id } => {
            let curves =
                activity_analysis::list_athlete_hr_curves_best(&client, &athlete_id).await?;
            println!("{}", serde_json::to_string_pretty(&curves)?);
        }
        Commands::ListAthletePowerCurvesBest { athlete_id } => {
            let curves =
                activity_analysis::list_athlete_power_curves_best(&client, &athlete_id).await?;
            println!("{}", serde_json::to_string_pretty(&curves)?);
        }
        Commands::ListAthletePaceCurvesBest { athlete_id } => {
            let curves =
                activity_analysis::list_athlete_pace_curves_best(&client, &athlete_id).await?;
            println!("{}", serde_json::to_string_pretty(&curves)?);
        }
        Commands::UpdateActivityIntervals {
            ref activity_id,
            ref intervals,
        } => {
            let input: activity_interval_editing::UpdateIntervalsInput =
                serde_json::from_str(intervals)?;
            let result =
                activity_interval_editing::update_activity_intervals(&client, activity_id, &input)
                    .await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::UpdateActivityInterval {
            ref activity_id,
            interval_id,
            ref data,
        } => {
            let input: activity_interval_editing::UpdateIntervalInput = serde_json::from_str(data)?;
            let result = activity_interval_editing::update_activity_interval(
                &client,
                activity_id,
                interval_id,
                &input,
            )
            .await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::SplitActivityInterval {
            ref activity_id,
            interval_id,
            ref split_index,
        } => {
            let input = activity_interval_editing::SplitIntervalInput {
                activity_id: activity_id.clone(),
                interval_id,
                split_index: *split_index,
            };
            let result =
                activity_interval_editing::split_activity_interval(&client, activity_id, &input)
                    .await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::DeleteActivityIntervals {
            ref activity_id,
            ref interval_ids,
        } => {
            let ids: Vec<i64> = interval_ids
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            let input = activity_interval_editing::DeleteIntervalsInput { interval_ids: ids };
            activity_interval_editing::delete_activity_intervals(&client, activity_id, &input)
                .await?;
            println!("Intervals deleted successfully");
        }
        Commands::UpdateActivityStreams {
            ref activity_id,
            ref data,
        } => {
            let input: activity_interval_editing::StreamUpdate = serde_json::from_str(data)?;
            activity_interval_editing::update_activity_streams(&client, activity_id, &input)
                .await?;
            println!("Streams updated successfully");
        }
        Commands::UpdateActivityStreamsCSV {
            ref activity_id,
            ref csv_file,
        } => {
            let csv_content = std::fs::read_to_string(csv_file)?;
            activity_interval_editing::update_activity_streams_csv(
                &client,
                activity_id,
                &csv_content,
            )
            .await?;
            println!("Streams updated from CSV successfully");
        }
        Commands::UploadActivity {
            ref athlete_id,
            ref file,
        } => {
            let result = activity_upload::upload_activity(&client, athlete_id, file).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::CreateManualActivitiesBulk {
            ref athlete_id,
            ref activities,
        } => {
            let input: activity_upload::BulkManualActivitiesInput =
                serde_json::from_str(activities)?;
            let result =
                activity_upload::create_manual_activities_bulk(&client, athlete_id, &input).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::DownloadActivityFitFiles {
            ref athlete_id,
            ref output,
        } => {
            activity_upload::download_activity_fit_files(&client, athlete_id, output).await?;
            println!("FIT files downloaded to {}", output);
        }
        Commands::GetGear {
            ref athlete_id,
            ref gear_id,
        } => {
            let gear = gear_operations::get_gear(&client, athlete_id, gear_id).await?;
            println!("{}", serde_json::to_string_pretty(&gear)?);
        }
        Commands::CreateGear {
            ref athlete_id,
            ref name,
            ref distance,
            ref retired,
        } => {
            let input = gear_operations::CreateGearInput {
                name: name.clone(),
                distance: *distance,
                retired: *retired,
            };
            let gear = gear_operations::create_gear(&client, athlete_id, &input).await?;
            println!("{}", serde_json::to_string_pretty(&gear)?);
        }
        Commands::UpdateGear {
            ref athlete_id,
            ref gear_id,
            ref name,
            ref distance,
            ref retired,
        } => {
            let input = gear_operations::CreateGearInput {
                name: name.clone(),
                distance: *distance,
                retired: *retired,
            };
            let gear = gear_operations::update_gear(&client, athlete_id, gear_id, &input).await?;
            println!("{}", serde_json::to_string_pretty(&gear)?);
        }
        Commands::DeleteGear {
            ref athlete_id,
            ref gear_id,
        } => {
            gear_operations::delete_gear(&client, athlete_id, gear_id).await?;
            println!("Gear deleted successfully");
        }
        Commands::ReplaceGear {
            ref athlete_id,
            ref gear_id,
            ref new_gear_id,
            ref retired_date,
        } => {
            let input = gear_operations::ReplaceGearInput {
                new_gear_id: new_gear_id.clone(),
                retired_date: retired_date.clone(),
            };
            gear_operations::replace_gear(&client, athlete_id, gear_id, &input).await?;
            println!("Gear replaced successfully");
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
        Commands::UpdateWellness {
            id,
            date,
            weight,
            resting_hr,
            hrv,
            mood,
            fatigue,
            motivation,
            sleep,
            sleep_quality,
        } => {
            let record = csv_and_wellness::WellnessUpdate {
                ctl: None,
                atl: None,
                weight,
                resting_hr,
                hrv,
                mood,
                fatigue,
                motivation,
                sleep,
                sleep_quality,
                soreness: None,
                stress: None,
                fitness: None,
                readiness: None,
            };
            let result = csv_and_wellness::update_wellness(&client, &id, &date, &record).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::DownloadActivitiesCsv { id, output } => {
            csv_and_wellness::download_activities_csv(&client, &id, &output).await?;
            println!("Activities CSV downloaded to {}", output);
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
        Commands::ListActivitiesAround {
            id,
            activity_id,
            route_id,
            limit,
        } => {
            let params = list_activities_around::ListActivitiesAroundParams {
                activity_id,
                route_id,
                limit,
            };
            let activities =
                list_activities_around::list_activities_around(&client, &id, &params).await?;
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
        Commands::CalcGear {
            athlete_id,
            gear_id,
        } => {
            let stats = gear_operations::calc_gear(&client, &athlete_id, &gear_id).await?;
            println!("{}", serde_json::to_string_pretty(&stats)?);
        }
        Commands::CreateGearReminder {
            athlete_id,
            gear_id,
            distance,
            date,
            notes,
        } => {
            let reminder = gear_operations::GearReminder {
                id: None,
                distance,
                date,
                notes,
                done: None,
            };
            let result =
                gear_operations::create_gear_reminder(&client, &athlete_id, &gear_id, &reminder)
                    .await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::UpdateGearReminder {
            athlete_id,
            gear_id,
            reminder_id,
            distance,
            date,
            notes,
            done,
        } => {
            let reminder = gear_operations::GearReminder {
                id: Some(reminder_id),
                distance,
                date,
                notes,
                done: Some(done),
            };
            let result = gear_operations::update_gear_reminder(
                &client,
                &athlete_id,
                &gear_id,
                reminder_id,
                &reminder,
            )
            .await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::DeleteGearReminder {
            athlete_id,
            gear_id,
            reminder_id,
        } => {
            gear_operations::delete_gear_reminder(&client, &athlete_id, &gear_id, reminder_id)
                .await?;
            println!("Gear reminder deleted successfully");
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
        Commands::SearchActivitiesFull { id, query, limit } => {
            let activities =
                search_activities_full::search_activities_full(&client, &id, &query, limit).await?;
            println!("{}", serde_json::to_string_pretty(&activities)?);
        }
        Commands::SearchActivityIntervals {
            id,
            min_secs,
            max_secs,
            min_intensity,
            max_intensity,
            interval_type,
            min_reps,
            max_reps,
            limit,
        } => {
            let params = search_activity_intervals::IntervalSearchParams {
                min_secs,
                max_secs,
                min_intensity,
                max_intensity,
                interval_type,
                min_reps,
                max_reps,
                limit,
            };
            let results =
                search_activity_intervals::search_activity_intervals(&client, &id, &params).await?;
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
        Commands::UpdateRoute {
            athlete_id,
            route_id,
            name,
            description,
            commute,
        } => {
            let route = get_route::AthleteRoute {
                athlete_id: None,
                route_id: Some(route_id),
                name: name.clone(),
                rename_activities: None,
                commute: Some(commute),
                tags: None,
                description: description.clone(),
                replaced_by_route_id: None,
                latlngs: None,
            };
            let result = get_route::update_route(&client, &athlete_id, route_id, &route).await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
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
        Commands::GetChat { chat_id } => {
            let chat = chat_and_fitness::get_chat(&client, chat_id).await?;
            println!("{}", serde_json::to_string_pretty(&chat)?);
        }
        Commands::ListChatMessages {
            chat_id,
            before_id,
            limit,
        } => {
            let params = chat_and_fitness::ListChatMessagesParams {
                chat_id,
                before_id,
                limit,
            };
            let messages = chat_and_fitness::list_chat_messages(&client, &params).await?;
            println!("{}", serde_json::to_string_pretty(&messages)?);
        }
        Commands::SendChatMessage {
            athlete_id,
            content,
            chat_id,
        } => {
            let result =
                chat_and_fitness::send_chat_message(&client, &athlete_id, &content, chat_id)
                    .await?;
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        Commands::ListFitnessModelEvents { id } => {
            let events = chat_and_fitness::list_fitness_model_events(&client, &id).await?;
            println!("{}", serde_json::to_string_pretty(&events)?);
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
