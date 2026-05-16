---
name: intervals-cli
description: >
  Use the `intervals` CLI to interact with the intervals.icu (Intervals.icu) API for
  training data, activities, workouts, power/HR/pace curves, wellness, events, routes,
  chats, and gear. Trigger whenever the user mentions intervals.icu, Intervals.icu,
  training data, activities, workouts, power curves, heart rate curves, pace curves,
  wellness records, calendar events, training plans, or wants to create/update/delete
  activities or events. Also trigger for queries about FTP, CTL/ATL (fitness/fatigue),
  training load, or any request to fetch or modify training data from intervals.icu.
  The user's athlete ID is stored in the repo's AGENTS.md or can be provided explicitly.
---

# Intervals CLI Skill

This skill helps you use the `intervals` CLI tool to interact with the [intervals.icu](https://intervals.icu) API.

## Prerequisites

- The `intervals` binary must be built or available in PATH
- `INTERVALS_API_KEY` environment variable must be set
- The user's athlete ID — check `AGENTS.md` in the repo root, or ask the user

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
| `intervals update-training-plan <athlete-id> --data <json>` | Update athlete training plan |
| `intervals update-athlete-plans --data <json>` | Update training plans for multiple athletes |
| `intervals apply-plan-changes <athlete-id>` | Apply plan changes to calendar |
| `intervals get-athlete-mmp-model <athlete-id> <activity-type>` | Get MMP power model for a sport |
| `intervals list-sport-settings <athlete-id>` | Get sport settings (FTP, LTHR per sport) |
| `intervals get-sport-setting <athlete-id> <id>` | Get sport setting by ID |
| `intervals create-sport-setting <athlete-id>` | Create sport setting (options: `--types`, `--ftp`, `--lthr`, `--name`) |
| `intervals update-sport-setting <athlete-id> <id>` | Update sport setting (options: `--types`, `--ftp`, `--lthr`, `--name`) |
| `intervals delete-sport-setting <athlete-id> <id>` | Delete sport setting |
| `intervals apply-sport-setting <athlete-id> <id>` | Apply setting to matching activities |
| `intervals list-sport-setting-matching <athlete-id> <id>` | List activities matching setting |
| `intervals list-sport-setting-pace-distances <athlete-id> <id>` | List pace distances for sport |
| `intervals list-gear <athlete-id>` | List athlete gear (bikes, shoes, etc.) |
| `intervals get-gear <athlete-id> <gear-id>` | Get a specific gear item |
| `intervals create-gear <athlete-id>` | Create new gear (options: `--name`, `--distance`, `--retired`) |
| `intervals update-gear <athlete-id> <gear-id>` | Update gear (options: `--name`, `--distance`, `--retired`) |
| `intervals delete-gear <athlete-id> <gear-id>` | Delete gear |
| `intervals replace-gear <athlete-id> <gear-id>` | Retire and replace gear (options: `--new-gear-id`, `--retired-date`) |
| `intervals list-custom-items <athlete-id>` | List custom items (charts, custom fields) |
| `intervals get-custom-item <athlete-id> <item-id>` | Get a specific custom item |
| `intervals delete-custom-item <athlete-id> <item-id>` | Delete a custom item |
| `intervals create-custom-item <athlete-id> --data <json>` | Create a custom item |
| `intervals update-custom-item <athlete-id> <item-id> --data <json>` | Update a custom item |
| `intervals update-custom-item-indexes <athlete-id> --data <json>` | Re-order custom items |
| `intervals upload-custom-item-image <athlete-id> <item-id> <file>` | Upload image for custom item |

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
| `intervals search-activity-intervals <athlete-id>` | Find activities with matching intervals (options: `--min-secs`, `--max-secs`, `--min-intensity`, `--max-intensity`, `--interval-type`) |
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
| `intervals get-activity-hr-curve <activity-id>` | Get activity heart rate curve |
| `intervals get-activity-hr-histogram <activity-id>` | Get activity heart rate histogram |
| `intervals get-activity-hr-load-model <activity-id>` | Get activity heart rate load model |
| `intervals get-activity-pace-curve <activity-id>` | Get activity pace curve |
| `intervals get-activity-pace-histogram <activity-id>` | Get activity pace histogram |
| `intervals get-activity-power-curves <activity-id>` | Get activity power curves |
| `intervals get-activity-power-curve <activity-id>` | Get activity power curve |
| `intervals get-activity-power-histogram <activity-id>` | Get activity power histogram |
| `intervals get-activity-power-spike-model <activity-id>` | Get activity power spike model |
| `intervals get-activity-power-vs-hr <activity-id>` | Get activity power vs heart rate |
| `intervals get-activity-gap-histogram <activity-id>` | Get activity gradient-adjusted pace histogram |
| `intervals get-activity-time-at-hr <activity-id>` | Get activity time at heart rate |
| `intervals update-activity-intervals <activity-id> --intervals <json>` | Update intervals for an activity |
| `intervals update-activity-interval <activity-id> <interval-id> --data <json>` | Update/create an interval |
| `intervals split-activity-interval <activity-id> --interval-id <id>` | Split an interval |
| `intervals delete-activity-intervals <activity-id> --interval-ids <ids>` | Delete intervals |
| `intervals update-activity-streams <activity-id> --data <json>` | Update streams from JSON |
| `intervals update-activity-streams-csv <activity-id> --csv-file <path>` | Update streams from CSV |
| `intervals download-activity-file <activity-id> <output>` | Download original activity file |
| `intervals download-activity-fit-file <activity-id> <output>` | Download FIT file (options: `--power`, `--hr`) |
| `intervals download-activity-gpx-file <activity-id> <output>` | Download GPX file (options: `--power`, `--hr`) |
| `intervals download-activities-csv <athlete-id> <output>` | Download all activities as CSV |
| `intervals upload-activity <athlete-id> <file>` | Upload activity file (fit/tcx/gpx/zip) |
| `intervals create-manual-activities-bulk <athlete-id> --activities <json>` | Bulk create manual activities |
| `intervals download-activity-fit-files <athlete-id> --output <path>` | Download all FIT files as zip |
| `intervals download-gear-csv <athlete-id> <output>` | Download gear as CSV |
| `intervals download-events-csv <athlete-id> <output>` | Download events as CSV |
| `intervals download-wellness-csv <athlete-id> <output>` | Download wellness as CSV |
| `intervals list-activity-streams-ext <activity-id>` | List activity streams |

### Performance Curves

| Command | Description |
|---------|-------------|
| `intervals list-athlete-power-curves <athlete-id> <activity-type>` | List power curves for a sport type (e.g., Ride, Run) |
| `intervals list-athlete-hr-curves <athlete-id>` | List heart rate curves |
| `intervals list-athlete-pace-curves <athlete-id>` | List pace curves |
| `intervals get-power-hr-curve <athlete-id> --start <date> --end <date>` | Get power vs heart rate curve |
| `intervals list-pace-distances` | List pace curve distances |
| `intervals list-athlete-hr-curves-best <athlete-id>` | List best heart rate curves |
| `intervals list-athlete-power-curves-best <athlete-id>` | List best power curves |
| `intervals list-athlete-pace-curves-best <athlete-id>` | List best pace curves |

### Wellness & Weather

| Command | Description |
|---------|-------------|
| `intervals list-wellness <athlete-id>` | List wellness records (weight, resting HR, HRV, mood, CTL/ATL) |
| `intervals get-wellness <athlete-id> <date>` | Get wellness record for a specific date |
| `intervals update-wellness <athlete-id> <date>` | Update wellness record (options: `--weight`, `--resting-hr`, `--hrv`, `--mood`, `--sleep`) |
| `intervals update-wellness-bulk <athlete-id> --records <json>` | Bulk update wellness records |
| `intervals upload-wellness-csv <athlete-id> <file>` | Upload wellness records from CSV |
| `intervals get-weather-forecast <athlete-id>` | Get weather forecast for athlete's location |
| `intervals get-weather-config <athlete-id>` | Get weather forecast configuration |
| `intervals update-weather-config <athlete-id>` | Update weather forecast configuration (`--forecasts <json>`) |

### Workouts, Events & Calendar

| Command | Description |
|---------|-------------|
| `intervals list-workouts <athlete-id>` | List workouts in athlete's library |
| `intervals get-workout <athlete-id> <workout-id>` | Get a specific workout |
| `intervals create-workout <athlete-id>` | Create workout (options: `--name`, `--workout-type`, `--folder-id`) |
| `intervals update-workout <athlete-id> <workout-id>` | Update workout (options: `--name`, `--description`, `--workout-type`) |
| `intervals delete-workout <athlete-id> <workout-id>` | Delete a workout |
| `intervals create-workouts-bulk <athlete-id> --workouts <json>` | Create multiple workouts |
| `intervals duplicate-workouts <athlete-id> --workout-ids <ids>` | Duplicate workouts |
| `intervals download-workouts-zip <athlete-id> <output>` | Download workouts as zip (options: `--oldest`, `--newest`, `--ext`) |
| `intervals download-workout <athlete-id> <workout-id> <output>` | Download a workout (option: `--ext`) |
| `intervals download-workout-ext <workout-id> <output>` | Download a workout without athlete context (option: `--ext`) |
| `intervals import-workout <athlete-id> <folder-id> <file>` | Import workout from file |
| `intervals list-folders <athlete-id>` | List workout folders and plans |
| `intervals list-folder-shared-with <athlete-id> <folder-id>` | List athletes a folder is shared with |
| `intervals delete-folder <athlete-id> <folder-id>` | Delete a folder and all its workouts |
| `intervals create-folder <athlete-id>` | Create folder (options: `--name`, `--description`, `--folder-type`) |
| `intervals update-folder <athlete-id> <folder-id>` | Update folder (options: `--name`, `--description`, `--folder-type`) |
| `intervals update-folder-shared-with <athlete-id> <folder-id> --athlete-ids <ids>` | Update folder sharing |
| `intervals update-folder-workouts <athlete-id> <folder-id> --workout-ids <ids>` | Update plan workouts |
| `intervals list-events <athlete-id>` | List calendar events (options: `--oldest`, `--newest`, `--category`, `--limit`) |
| `intervals create-event <athlete-id> --start-date-local <datetime> --event-type <type> --category <category>` | Create calendar event (options: `--name`, `--description`, `--uid`, `--upsert-on-uid`) |
| `intervals get-event <athlete-id> <event-id>` | Get an event (planned workout, note, etc.) |
| `intervals update-event <athlete-id> <event-id>` | Update an event (options: `--name`, `--description`, `--notes`) |
| `intervals delete-event <athlete-id> <event-id>` | Delete an event from calendar |
| `intervals mark-event-done <athlete-id> <event-id>` | Mark event as done (create manual activity from planned workout) |
| `intervals delete-events-range <athlete-id> --start-date <date> --end-date <date>` | Delete events in date range |
| `intervals update-events-range <athlete-id> --start-date <date> --end-date <date>` | Update events in date range (options: `--name`, `--description`, `--category`) |
| `intervals delete-events-bulk <athlete-id>` | Bulk delete events (options: `--ids`, `--external-ids`) |
| `intervals create-events-bulk <athlete-id> --events <json>` | Create multiple events |
| `intervals duplicate-events <athlete-id> --event-ids <ids>` | Duplicate events (options: `--num-copies`, `--weeks-between`) |
| `intervals apply-plan-to-events <athlete-id> --folder-id <id>` | Apply training plan to events |
| `intervals download-event-workout <athlete-id> <event-id> <output>` | Download event workout (option: `--ext`) |
| `intervals list-event-tags <athlete-id>` | List event tags for an athlete |
| `intervals list-workout-tags <athlete-id>` | List workout tags for an athlete |

### Routes

| Command | Description |
|---------|-------------|
| `intervals list-athlete-routes <athlete-id>` | List saved routes |
| `intervals get-route <athlete-id> <route-id>` | Get a route (option: `--include-path` for GPS data) |
| `intervals update-route <athlete-id> <route-id>` | Update a route (options: `--name`, `--description`, `--commute`) |
| `intervals compare-routes <athlete-id> <route-id> <other-id>` | Compare two routes for similarity |

### Chats

| Command | Description |
|---------|-------------|
| `intervals list-chats <athlete-id>` | List chats |
| `intervals get-chat <chat-id>` | Get a chat by id |
| `intervals list-chat-messages <chat-id>` | List messages for a chat (options: `--before-id`, `--limit`) |
| `intervals send-chat-message <athlete-id> <content>` | Send a chat message (option: `--chat-id`) |
| `intervals update-chat-message <chat-id> <msg-id> <content>` | Update a chat message |
| `intervals delete-chat-message <chat-id> <msg-id>` | Delete a chat message |
| `intervals mark-chat-messages-seen <chat-id> <msg-id>` | Mark messages as seen |

### Other

| Command | Description |
|---------|-------------|
| `intervals disconnect-app` | Disconnect the app from intervals.icu |
| `intervals get-shared-event <event-id>` | Get a shared event (e.g. race) |
| `intervals list-fitness-model-events <athlete-id>` | List events that influence fitness calculation |

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
