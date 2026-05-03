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
```

## Commands

### Athlete

- `get-athlete <id>` - Get athlete profile
- `get-athlete-profile <id>` - Get detailed athlete profile
- `get-athlete-training-plan <id>` - Get athlete training plan
- `list-athlete-power-curves <id> <activity_type>` - List athlete power curves
- `list-athlete-hr-curves <id>` - List athlete HR curves
- `list-athlete-pace-curves <id>` - List athlete pace curves
- `list-athlete-routes <id>` - List athlete routes
- `list-sport-settings <id>` - List sport settings
- `list-gear <id>` - List athlete gear
- `get-weather-forecast <id>` - Get weather forecast

### Activities

- `list-activities <id> --oldest <date> --limit <n>` - List activities
- `get-activity <id> <activityId>` - Get an activity
- `update-activity <activityId>` - Update an activity
- `create-manual-activity <id>` - Create a manual activity
- `delete-activity <activityId>` - Delete an activity
- `search-activities <id> <query>` - Search activities
- `list-activity-messages <activityId>` - List activity messages
- `list-activity-intervals <activityId>` - List activity intervals

### Wellness

- `list-wellness <id>` - List wellness records

### Workouts & Events

- `list-workouts <id>` - List workouts
- `get-workout <id> <workoutId>` - Get a workout
- `list-folders <id>` - List workout folders
- `list-events <id>` - List calendar events
- `create-event <id>` - Create a calendar event

### Chats

- `list-chats <id>` - List chats

## Development

```bash
# Run tests
cargo test

# Format code
cargo fmt
```
