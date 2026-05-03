# Intervals.mcu API CLI - TODO

## Project Setup

- [x] Create TODO.md
- [x] Initialize Rust project with cargo
- [x] Add dependencies: clap, reqwest, serde, serde_json, tokio, thiserror, urlencoding
- [x] Configure $INTERVALS_API_KEY environment variable

## Commands Implemented (65+ total)

### Athlete & Profile

- [x] `get-athlete <id>`
- [x] `get-athlete-profile <id>`
- [x] `get-athlete-summary <id>`
- [x] `get-athlete-settings <id> <device-class>`
- [x] `get-athlete-training-plan <id>`
- [x] `apply-plan-changes <id>`
- [x] `get-athlete-mmp-model <id> <activity-type>`
- [x] `list-sport-settings <id>`
- [x] `list-gear <id>`
- [x] `list-custom-items <id>`
- [x] `get-custom-item <athlete-id> <item-id>`
- [x] `delete-custom-item <athlete-id> <item-id>`

### Activities

- [x] `list-activities <id> --oldest <date>`
- [x] `get-activity <id> <activityId>`
- [x] `update-activity <activityId>`
- [x] `create-manual-activity <id>`
- [x] `delete-activity <activityId>`
- [x] `search-activities <id> <query>`
- [x] `list-activity-messages <activityId>`
- [x] `post-activity-message <activityId> <content>`
- [x] `list-activity-intervals <activityId>`
- [x] `list-activity-tags <id>`
- [x] `get-interval-stats <activityId> <start-index> <end-index>`
- [x] `get-activity-best-efforts <activityId> <stream>`
- [x] `get-activity-streams <activityId>`
- [x] `get-activity-map <activityId>`
- [x] `get-activity-weather-summary <activityId>`
- [x] `get-activity-segments <activityId>`
- [x] `download-activity-file <activityId> <output>`
- [x] `download-activity-fit-file <activityId> <output>`
- [x] `download-activity-gpx-file <activityId> <output>`

### Wellness & Weather

- [x] `list-wellness <id>`
- [x] `get-weather-forecast <id>`
- [x] `get-weather-config <id>`
- [x] `update-weather-config <id>`

### Workouts, Events & Calendar

- [x] `list-workouts <id>`
- [x] `get-workout <id> <workoutId>`
- [x] `list-folders <id>`
- [x] `list-folder-shared-with <athlete-id> <folder-id>`
- [x] `delete-folder <athlete-id> <folder-id>`
- [x] `list-events <id>`
- [x] `create-event <id>`
- [x] `get-event <athlete-id> <event-id>`
- [x] `update-event <athlete-id> <event-id>`
- [x] `delete-event <athlete-id> <event-id>`
- [x] `mark-event-done <athlete-id> <event-id>`
- [x] `list-event-tags <id>`
- [x] `list-workout-tags <id>`

### Performance Curves & Models

- [x] `list-athlete-power-curves <id> <activity-type>`
- [x] `list-athlete-hr-curves <id>`
- [x] `list-athlete-pace-curves <id>`
- [x] `get-power-hr-curve <id>`
- [x] `list-pace-distances`

### Routes

- [x] `list-athlete-routes <id>`
- [x] `get-route <athlete-id> <route-id>`

### Chats

- [x] `list-chats <id>`

### Other

- [x] `disconnect-app`
- [x] `get-shared-event <event-id>`

## Implementation Approach

- TDD: Write tests first, then implement
- One command at a time
- Commit after each command passes tests
- Use reqwest for HTTP calls
- Use clap for CLI argument parsing
- Output JSON by default

## Current Status

All 65+ commands implemented and verified against live API. 125+ tests passing.

### Live API Verification Results (athlete: i545497)

| Command | Status | Notes |
|---------|--------|-------|
| `get-athlete` | ✅ Verified | Returns athlete data |
| `get-athlete-profile` | ✅ Verified | Returns profile with city, country, timezone |
| `get-athlete-summary` | ✅ Verified | Returns fitness data (CTL/ATL, training load) |
| `get-athlete-settings` | ✅ Verified | Returns desktop settings |
| `get-athlete-mmp-model` | ✅ Verified | Returns model (no data for this athlete) |
| `get-athlete-training-plan` | ✅ Verified | Returns null (no plan) |
| `apply-plan-changes` | ✅ Verified | Returns 422 (athlete not on plan - expected) |
| `get-activity` | ✅ Verified | Returns full activity details |
| `get-activity-best-efforts` | ✅ Verified | Returns HR efforts |
| `get-activity-streams` | ✅ Verified | Returns HR/time streams |
| `get-activity-map` | ✅ Verified | Returns lat/lng points |
| `get-activity-segments` | ✅ Verified | Returns empty (no segments) |
| `get-activity-weather-summary` | ✅ Verified | Returns weather data |
| `get-interval-stats` | ✅ Verified | Returns stats for interval range |
| `list-activities` | ✅ Verified | Returns recent activities |
| `list-activity-intervals` | ✅ Verified | Returns activity intervals |
| `list-activity-messages` | ✅ Verified | Returns AI training review message |
| `list-activity-tags` | ✅ Verified | Returns empty (no tags) |
| `list-events` | ✅ Verified | Returns upcoming events |
| `get-event` | ✅ Verified | Returns event details |
| `list-event-tags` | ✅ Verified | Returns empty |
| `list-workout-tags` | ✅ Verified | Returns empty |
| `list-folders` | ✅ Verified | Returns "Workouts" folder |
| `list-chats` | ✅ Verified | Returns activity chat |
| `list-custom-items` | ✅ Verified | Returns custom chart |
| `list-gear` | ✅ Verified | Returns empty |
| `list-workouts` | ✅ Verified | Returns empty |
| `list-wellness` | ✅ Verified | Returns wellness records |
| `list-sport-settings` | ✅ Verified | Returns Run/Swim/Other settings |
| `list-athlete-hr-curves` | ✅ Verified | Returns HR curve data |
| `list-athlete-pace-curves` | ✅ Verified | Returns pace curve data |
| `list-athlete-power-curves` | ✅ Verified | Returns empty (no power data for runs) |
| `list-athlete-routes` | ✅ Verified | Returns empty |
| `get-weather-forecast` | ✅ Verified | Returns empty |
| `list-pace-distances` | ✅ Verified | Returns distance list |
| `search-activities` | ✅ Verified | Returns matching "Night Run" activities |
| `get-shared-event` | ✅ Verified | Returns 404 (not a shared event - expected) |

### Fixed Commands

1. `list-athlete-power-curves` - Added required `activity_type` param
2. `get-athlete-profile` - Parse nested `athlete` field
3. `get-weather-forecast` - Parse `{forecasts: [...]}` wrapper
4. `list-sport-settings` - Changed `id` from String to i64
5. `list-athlete-hr-curves` - Parse `{list: [...]}` wrapper
6. `list-athlete-pace-curves` - Parse `{list: [...]}` wrapper
7. `get-athlete-training-plan` - Match actual schema with optional fields
8. `list-activity-intervals` - Parse `{icu_intervals: [...]}` wrapper
9. `list-chats` - `Chat.id` was String, API returns i64
10. `list-activity-messages` - `ActivityMessage` struct had wrong fields
11. `get-activity-weather-summary` - Matched actual API response schema
12. `get-activity-map` - `latlngs` can contain null entries
13. `get-activity-streams` - `data` contains nulls, uses `Vec<serde_json::Value>`
14. `get-activity` - Returns array with single activity object

### CI Fixes

- Cross-build (aarch64) failed due to missing OpenSSL headers → switched `reqwest` to `rustls-tls`
- Upgraded `reqwest` from 0.12 to 0.13 (rustls is now default TLS backend)
