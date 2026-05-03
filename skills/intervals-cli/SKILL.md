---
name: intervals-cli
description: >
  Use the `intervals` CLI to interact with the intervals.mcu (Intervals.icu) API for
  training data, activities, workouts, power/HR/pace curves, wellness, events, and gear.
  Trigger whenever the user mentions intervals.mcu, Intervals.icu, training data, activities,
  workouts, power curves, heart rate curves, pace curves, wellness records, calendar events,
  training plans, or wants to create/update/delete activities or events. Also trigger for
  queries about FTP, CTL/ATL (fitness/fatigue), training load, or any request to fetch or
  modify training data from intervals.mcu. The user's athlete ID is stored in the repo's
  AGENTS.md or can be provided explicitly.
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
| `intervals get-athlete-profile <athlete-id>` | Get detailed athlete profile |
| `intervals get-athlete-training-plan <athlete-id>` | Get current training plan |
| `intervals list-sport-settings <athlete-id>` | Get sport settings (FTP, LTHR per sport) |
| `intervals list-gear <athlete-id>` | List athlete gear (bikes, shoes, etc.) |

### Activities

| Command | Description |
|---------|-------------|
| `intervals list-activities <athlete-id> --oldest <date>` | List activities (required: `--oldest`, optional: `--newest`, `--limit`, `--route-id`) |
| `intervals get-activity <athlete-id> <activity-id>` | Get a single activity's details |
| `intervals update-activity <activity-id> [options]` | Update activity (options: `--name`, `--description`, `--activity-type`, `--sport`, `--distance`, `--elapsed-time`) |
| `intervals create-manual-activity <athlete-id> --start-date-local <datetime> --activity-type <type>` | Create manual activity (options: `--name`, `--description`, `--sport`, `--distance`, `--elapsed-time`) |
| `intervals delete-activity <activity-id>` | Delete an activity |
| `intervals search-activities <athlete-id> <query>` | Search activities by name |
| `intervals list-activity-messages <activity-id>` | List messages/comments on an activity |
| `intervals post-activity-message <activity-id> <content>` | Post a message/comment to an activity |
| `intervals list-activity-intervals <activity-id>` | List auto-detected intervals within an activity |

### Performance Curves

| Command | Description |
|---------|-------------|
| `intervals list-athlete-power-curves <athlete-id> <activity-type>` | List power curves for a sport type (e.g., Ride, Run) |
| `intervals list-athlete-hr-curves <athlete-id>` | List heart rate curves |
| `intervals list-athlete-pace-curves <athlete-id>` | List pace curves |

### Wellness & Weather

| Command | Description |
|---------|-------------|
| `intervals list-wellness <athlete-id>` | List wellness records (weight, resting HR, HRV, mood) |
| `intervals get-weather-forecast <athlete-id>` | Get weather forecast for athlete's location |

### Workouts & Events

| Command | Description |
|---------|-------------|
| `intervals list-workouts <athlete-id>` | List workouts in athlete's library |
| `intervals get-workout <athlete-id> <workout-id>` | Get a specific workout |
| `intervals list-folders <athlete-id>` | List workout folders and plans |
| `intervals list-events <athlete-id>` | List calendar events |
| `intervals create-event <athlete-id> --start-date-local <datetime> --event-type <type> --category <category>` | Create calendar event (options: `--name`, `--description`, `--uid`, `--calendar-id`, `--upsert-on-uid`) |

### Other

| Command | Description |
|---------|-------------|
| `intervals list-athlete-routes <athlete-id>` | List saved routes |
| `intervals list-chats <athlete-id>` | List chats |

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
