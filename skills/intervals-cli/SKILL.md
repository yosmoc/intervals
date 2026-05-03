---
name: intervals-cli
description: >
  Use the `intervals` CLI to interact with the intervals.mcu (Intervals.icu) API for
  training data, activities, workouts, power/HR/pace curves, wellness, events, routes,
  chats, and gear. Trigger whenever the user mentions intervals.mcu, Intervals.icu,
  training data, activities, workouts, power curves, heart rate curves, pace curves,
  wellness records, calendar events, training plans, or wants to create/update/delete
  activities or events. Also trigger for queries about FTP, CTL/ATL (fitness/fatigue),
  training load, or any request to fetch or modify training data from intervals.mcu.
  The user's athlete ID is stored in the repo's AGENTS.md or can be provided explicitly.
---

# Intervals CLI Skill

This skill helps you use the `intervals` CLI tool to interact with the [intervals.mcu](https://intervals.mcu) API.

## Prerequisites

- The `intervals` binary must be built or available in PATH
- `INTERVALS_API_KEY` environment variable must be set
- The user's athlete ID (e.g., `i545497`) — check `AGENTS.md` in the repo root, or ask the user

## Command Reference

All commands output JSON by default.

### Athlete & Profile

| Command | Description |
|---------|-------------|
| `intervals get-athlete <athlete-id>` | Get basic athlete profile |
| `intervals get-athlete-profile <athlete-id>` | Get detailed athlete profile (city, country, timezone, sex) |
| `intervals get-athlete-summary <athlete-id> [--start <date>] [--end <date>]` | Get athlete summary (fitness data, CTL/ATL, training load) |
| `intervals get-athlete-settings <athlete-id> <device-class>` | Get settings for phone/tablet/desktop |
| `intervals get-athlete-training-plan <athlete-id>` | Get current training plan |
| `intervals apply-plan-changes <athlete-id>` | Apply plan changes to calendar |
| `intervals get-athlete-mmp-model <athlete-id> <activity-type>` | Get MMP power model for a sport |
| `intervals list-sport-settings <athlete-id>` | Get sport settings (FTP, LTHR per sport) |
| `intervals list-gear <athlete-id>` | List athlete gear (bikes, shoes, etc.) |
| `intervals list-custom-items <athlete-id>` | List custom items (charts, custom fields) |
| `intervals get-custom-item <athlete-id> <item-id>` | Get a specific custom item |
| `intervals delete-custom-item <athlete-id> <item-id>` | Delete a custom item |

### Activities

| Command | Description |
|---------|-------------|
| `intervals list-activities <athlete-id> --oldest <date>` | List activities (required: `--oldest`, optional: `--newest`, `--limit`, `--route-id`) |
| `intervals list-activities-around <athlete-id> <activity-id>` | List activities before/after another (options: `--route-id`, `--limit`) |
| `intervals get-activity <athlete-id> <activity-id>` | Get a single activity's details |
| `intervals get-activities <athlete-id> <ids>` | Fetch multiple activities by id (comma-separated, option: `--intervals`) |
| `intervals update-activity <activity-id> [options]` | Update activity (options: `--name`, `--description`, `--activity-type`, `--sport`, `--distance`, `--elapsed-time`) |
| `intervals create-manual-activity <athlete-id> --start-date-local <datetime> --activity-type <type>` | Create manual activity (options: `--name`, `--description`, `--sport`, `--distance`, `--elapsed-time`) |
| `intervals delete-activity <activity-id>` | Delete an activity |
| `intervals search-activities <athlete-id> <query>` | Search activities by name or tag |
| `intervals search-activities-full <athlete-id> <query>` | Search activities returning full details (option: `--limit`) |
| `intervals list-activity-messages <activity-id>` | List messages/comments on an activity |
| `intervals post-activity-message <activity-id> <content>` | Post a message/comment to an activity |
| `intervals list-activity-intervals <activity-id>` | List auto-detected intervals within an activity |
| `intervals list-activity-tags <athlete-id>` | List all activity tags for an athlete |
| `intervals get-interval-stats <activity-id> <start-index> <end-index>` | Get interval stats for a portion of an activity |
| `intervals get-activity-best-efforts <activity-id> <stream>` | Find best efforts (options: `--duration`, `--distance`, `--count`) |
| `intervals get-activity-streams <activity-id>` | Get time series data (options: `--types`, `--include-defaults`) |
| `intervals get-activity-map <activity-id>` | Get activity map data (lat/lng, bounds) |
| `intervals get-activity-weather-summary <activity-id>` | Get weather summary for an activity |
| `intervals get-activity-segments <activity-id>` | Get activity segments |
| `intervals download-activity-file <activity-id> <output>` | Download original activity file |
| `intervals download-activity-fit-file <activity-id> <output>` | Download FIT file (options: `--power`, `--hr`) |
| `intervals download-activity-gpx-file <activity-id> <output>` | Download GPX file (options: `--power`, `--hr`) |

### Performance Curves

| Command | Description |
|---------|-------------|
| `intervals list-athlete-power-curves <athlete-id> <activity-type>` | List power curves for a sport type (e.g., Ride, Run) |
| `intervals list-athlete-hr-curves <athlete-id>` | List heart rate curves |
| `intervals list-athlete-pace-curves <athlete-id>` | List pace curves |
| `intervals get-power-hr-curve <athlete-id> --start <date> --end <date>` | Get power vs heart rate curve |
| `intervals list-pace-distances` | List pace curve distances |

### Wellness & Weather

| Command | Description |
|---------|-------------|
| `intervals list-wellness <athlete-id>` | List wellness records (weight, resting HR, HRV, mood, CTL/ATL) |
| `intervals get-wellness <athlete-id> <date>` | Get wellness record for a specific date |
| `intervals get-weather-forecast <athlete-id>` | Get weather forecast for athlete's location |
| `intervals get-weather-config <athlete-id>` | Get weather forecast configuration |
| `intervals update-weather-config <athlete-id>` | Update weather forecast configuration (`--forecasts <json>`) |

### Workouts, Events & Calendar

| Command | Description |
|---------|-------------|
| `intervals list-workouts <athlete-id>` | List workouts in athlete's library |
| `intervals get-workout <athlete-id> <workout-id>` | Get a specific workout |
| `intervals list-folders <athlete-id>` | List workout folders and plans |
| `intervals list-folder-shared-with <athlete-id> <folder-id>` | List athletes a folder is shared with |
| `intervals delete-folder <athlete-id> <folder-id>` | Delete a folder and all its workouts |
| `intervals list-events <athlete-id>` | List calendar events (options: `--oldest`, `--newest`, `--category`, `--limit`) |
| `intervals create-event <athlete-id> --start-date-local <datetime> --event-type <type> --category <category>` | Create calendar event (options: `--name`, `--description`, `--uid`, `--upsert-on-uid`) |
| `intervals get-event <athlete-id> <event-id>` | Get an event (planned workout, note, etc.) |
| `intervals update-event <athlete-id> <event-id>` | Update an event (options: `--name`, `--description`, `--notes`) |
| `intervals delete-event <athlete-id> <event-id>` | Delete an event from calendar |
| `intervals mark-event-done <athlete-id> <event-id>` | Mark event as done (create manual activity from planned workout) |
| `intervals list-event-tags <athlete-id>` | List event tags for an athlete |
| `intervals list-workout-tags <athlete-id>` | List workout tags for an athlete |

### Routes

| Command | Description |
|---------|-------------|
| `intervals list-athlete-routes <athlete-id>` | List saved routes |
| `intervals get-route <athlete-id> <route-id>` | Get a route (option: `--include-path` for GPS data) |

### Chats

| Command | Description |
|---------|-------------|
| `intervals list-chats <athlete-id>` | List chats |

### Other

| Command | Description |
|---------|-------------|
| `intervals disconnect-app` | Disconnect the app from intervals.icu |
| `intervals get-shared-event <event-id>` | Get a shared event (e.g. race) |

## Examples

### List recent activities

```bash
intervals list-activities <athlete-id> --oldest 2024-01-01 --limit 10
```

### Create a manual run

```bash
intervals create-manual-activity <athlete-id> \
  --start-date-local "2024-06-15T07:00:00" \
  --activity-type Run \
  --name "Morning Run" \
  --distance 5000 \
  --elapsed-time 1500
```

### Get power curves for cycling

```bash
intervals list-athlete-power-curves <athlete-id> Ride
```

### Update an activity name

```bash
intervals update-activity <activity-id> --name "New Name"
```

### Get activity heart rate streams

```bash
intervals get-activity-streams <activity-id> --types heartrate,time
```

### Download a FIT file

```bash
intervals download-activity-fit-file <activity-id> activity.fit
```

### Get fitness summary for a date range

```bash
intervals get-athlete-summary <athlete-id> --start 2024-01-01 --end 2024-01-31
```

### List upcoming calendar events

```bash
intervals list-events <athlete-id> --oldest 2024-06-01 --newest 2024-06-30
```

### Mark a planned workout as done

```bash
intervals mark-event-done <athlete-id> <event-id>
```

## Common Activity Types

- `Ride`, `VirtualRide`, `MountainBikeRide`, `GravelRide`
- `Run`, `TrailRun`, `Walk`, `Hike`
- `Swim`, `OpenWaterSwim`
- `NordicSki`, `AlpineSki`
- `Workout` (indoor trainer)

## Tips

- Dates use ISO-8601 format (e.g., `2024-01-15` or `2024-01-15T07:00:00`)
- Distances are in meters, times are in seconds
- The `--oldest` flag is required for `list-activities`
- Power curves require an activity type (`Ride`, `Run`, etc.)
- Use `--upsert-on-uid` with `create-event` to avoid duplicates when the event has a known `--uid`
- Activity streams support types: `time`, `heartrate`, `watts`, `cadence`, `speed`, `distance`, `altitude`, `latlng`, `temperature`
- `get-activity-best-efforts` stream can be `watts`, `heartrate`, `speed`, `cadence`
