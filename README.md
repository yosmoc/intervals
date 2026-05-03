# intervals

A CLI for the [intervals.icu](https://intervals.icu) API, written in Rust.

## Setup

```bash
export INTERVALS_API_KEY="your-api-key"
```

## Usage

```bash
# Get athlete profile
intervals get-athlete <athlete-id>

# List activities
intervals list-activities <athlete-id> --oldest 2024-01-01 --limit 10

# Create a manual activity
intervals create-manual-activity <athlete-id> \
  --start-date-local "2024-01-01T10:00:00" \
  --activity-type Run \
  --name "Morning Run" \
  --distance 5000 \
  --elapsed-time 1500

# Get activity streams
intervals get-activity-streams <activity-id> --types heartrate,time

# Download FIT file
intervals download-activity-fit-file <activity-id> output.fit
```

## Commands

### Athlete & Profile

| Command | Description |
|---------|-------------|
| `get-athlete <id>` | Get basic athlete profile |
| `get-athlete-profile <id>` | Get detailed athlete profile (city, country, timezone, sex) |
| `get-athlete-summary <id> [--start <date>] [--end <date>]` | Get athlete summary (fitness data, CTL/ATL, training load) |
| `get-athlete-settings <id> <device-class>` | Get settings for phone/tablet/desktop |
| `get-athlete-training-plan <id>` | Get current training plan |
| `apply-plan-changes <id>` | Apply plan changes to calendar |
| `get-athlete-mmp-model <id> <activity-type>` | Get MMP power model for a sport |
| `list-sport-settings <id>` | List sport settings (FTP, LTHR per sport) |
| `list-gear <id>` | List athlete gear |
| `list-custom-items <id>` | List custom items (charts, custom fields) |
| `get-custom-item <athlete-id> <item-id>` | Get a specific custom item |
| `delete-custom-item <athlete-id> <item-id>` | Delete a custom item |

### Activities

| Command | Description |
|---------|-------------|
| `list-activities <id> --oldest <date>` | List activities (required: `--oldest`, optional: `--newest`, `--limit`, `--route-id`) |
| `list-activities-around <id> <activity-id>` | List activities before/after another (`--route-id`, `--limit`) |
| `get-activity <id> <activity-id>` | Get an activity's details |
| `get-activities <id> <ids>` | Fetch multiple activities by id (comma-separated, optional: `--intervals`) |
| `update-activity <activity-id>` | Update activity (`--name`, `--description`, `--activity-type`, `--sport`, `--distance`, `--elapsed-time`) |
| `create-manual-activity <id>` | Create manual activity (`--start-date-local`, `--activity-type`, `--name`, `--description`, `--sport`, `--distance`, `--elapsed-time`) |
| `delete-activity <activity-id>` | Delete an activity |
| `search-activities <id> <query>` | Search activities by name or tag |
| `search-activities-full <id> <query>` | Search activities returning full details (`--limit`) |
| `search-activity-intervals <id>` | Find activities with matching intervals (`--min-secs`, `--max-secs`, `--min-intensity`, `--max-intensity`) |
| `list-activity-messages <activity-id>` | List messages/comments on an activity |
| `post-activity-message <activity-id> <content>` | Post a message to an activity |
| `list-activity-intervals <activity-id>` | List auto-detected intervals within an activity |
| `list-activity-tags <id>` | List all activity tags for an athlete |
| `get-interval-stats <activity-id> <start-index> <end-index>` | Get interval stats for a portion of an activity |
| `get-activity-best-efforts <activity-id> <stream>` | Find best efforts (`--duration`, `--distance`, `--count`) |
| `get-activity-streams <activity-id>` | Get time series data (`--types`, `--include-defaults`) |
| `get-activity-map <activity-id>` | Get activity map data (lat/lng, bounds) |
| `get-activity-weather-summary <activity-id>` | Get weather summary for an activity |
| `get-activity-segments <activity-id>` | Get activity segments |
| `download-activity-file <activity-id> <output>` | Download original activity file |
| `download-activity-fit-file <activity-id> <output>` | Download FIT file (`--power`, `--hr`) |
| `download-activity-gpx-file <activity-id> <output>` | Download GPX file (`--power`, `--hr`) |
| `download-activities-csv <athlete-id> <output>` | Download all activities as CSV |

### Wellness & Weather

| Command | Description |
|---------|-------------|
| `list-wellness <id>` | List wellness records (weight, resting HR, HRV, CTL/ATL) |
| `get-wellness <id> <date>` | Get wellness record for a specific date |
| `update-wellness <id> <date>` | Update wellness record (`--weight`, `--resting-hr`, `--hrv`, `--mood`, `--sleep`) |
| `get-weather-forecast <id>` | Get weather forecast for athlete's location |
| `get-weather-config <id>` | Get weather forecast configuration |
| `update-weather-config <id>` | Update weather forecast configuration (`--forecasts <json>`) |

### Workouts, Events & Calendar

| Command | Description |
|---------|-------------|
| `list-workouts <id>` | List workouts in athlete's library |
| `get-workout <id> <workout-id>` | Get a specific workout |
| `list-folders <id>` | List workout folders and plans |
| `list-folder-shared-with <athlete-id> <folder-id>` | List athletes a folder is shared with |
| `delete-folder <athlete-id> <folder-id>` | Delete a folder and all its workouts |
| `list-events <id>` | List calendar events (`--oldest`, `--newest`, `--category`, `--limit`) |
| `create-event <id>` | Create calendar event (`--start-date-local`, `--event-type`, `--category`, `--name`, `--description`, `--uid`, `--upsert-on-uid`) |
| `get-event <athlete-id> <event-id>` | Get an event (planned workout, note, etc.) |
| `update-event <athlete-id> <event-id>` | Update an event (`--name`, `--description`, `--notes`) |
| `delete-event <athlete-id> <event-id>` | Delete an event from calendar |
| `mark-event-done <athlete-id> <event-id>` | Mark event as done (create manual activity from planned workout) |
| `list-event-tags <id>` | List event tags for an athlete |
| `list-workout-tags <id>` | List workout tags for an athlete |

### Performance Curves & Models

| Command | Description |
|---------|-------------|
| `list-athlete-power-curves <id> <activity-type>` | List power curves for a sport type (e.g., Ride, Run) |
| `list-athlete-hr-curves <id>` | List heart rate curves |
| `list-athlete-pace-curves <id>` | List pace curves |
| `get-power-hr-curve <id> --start <date> --end <date>` | Get power vs heart rate curve |
| `list-pace-distances` | List pace curve distances |

### Routes

| Command | Description |
|---------|-------------|
| `list-athlete-routes <id>` | List saved routes |
| `get-route <athlete-id> <route-id>` | Get a route (`--include-path` for GPS data) |
| `update-route <athlete-id> <route-id>` | Update a route (`--name`, `--description`, `--commute`) |

### Chats

| Command | Description |
|---------|-------------|
| `list-chats <id>` | List chats |
| `get-chat <chat-id>` | Get a chat by id |
| `list-chat-messages <chat-id>` | List messages for a chat (`--before-id`, `--limit`) |

### Other

| Command | Description |
|---------|-------------|
| `disconnect-app` | Disconnect the app from intervals.icu |
| `get-shared-event <event-id>` | Get a shared event (e.g. race) |
| `list-fitness-model-events <id>` | List events that influence fitness calculation |

## Development

```bash
# Run tests
cargo test

# Format code
cargo fmt --all
```
