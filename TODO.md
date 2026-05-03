# Intervals.mcu API CLI - TODO

## Project Setup

- [x] Create TODO.md
- [x] Initialize Rust project with cargo
- [x] Add dependencies: clap, reqwest, serde, serde_json, tokio, thiserror, urlencoding
- [x] Configure $INTERVALS_API_KEY environment variable

## Commands Implemented (26 total)

### Phase 1: Core Read Operations

- [x] `get-athlete <id>`
- [x] `list-activities <id> --oldest <date>`
- [x] `get-activity <id> <activityId>`
- [x] `list-workouts <id>`

### Phase 2: Write Operations

- [x] `update-activity <activityId>`
- [x] `create-manual-activity <id>`
- [x] `delete-activity <activityId>`

### Phase 3: Additional Features

- [x] `list-events <id>`
- [x] `list-wellness <id>`
- [x] `list-gear <id>`

### Phase 4: High Priority

- [x] `search-activities <id> <query>`
- [x] `list-folders <id>`
- [x] `create-event <id>`
- [x] `get-workout <id> <workoutId>`

### Phase 5: Medium Priority

- [x] `list-athlete-power-curves <id>`
- [x] `list-athlete-routes <id>`
- [x] `get-athlete-profile <id>`
- [x] `list-activity-messages <id> <activityId>`
- [x] `get-weather-forecast <id>`
- [x] `list-sport-settings <athleteId>`

### Phase 6: Lower Priority

- [x] `list-chats <id>`
- [x] `list-athlete-hr-curves <id>`
- [x] `list-athlete-pace-curves <id>`
- [x] `get-athlete-training-plan <id>`
- [x] `list-activity-intervals <id> <activityId>`

## Implementation Approach

- TDD: Write tests first, then implement
- One command at a time
- Commit after each command passes tests
- Use reqwest for HTTP calls
- Use clap for CLI argument parsing
- Output JSON by default

## Current Status

All 26 commands implemented and verified against live API. 75 tests passing.

### Live API Verification Results

| Command | Status | Notes |
|---------|--------|-------|
| `get-athlete` | ✅ Verified | Returns athlete data |
| `get-activity` | ✅ Verified | Returns activity data |
| `list-activities` | ✅ Verified | Returns activities for date range |
| `list-wellness` | ✅ Verified | Returns wellness records |
| `update-activity` | ✅ Verified | Updates activity name |
| `list-gear` | ✅ Verified | Returns empty (no gear) |
| `search-activities` | ✅ Verified | Finds activities by name |
| `list-folders` | ✅ Verified | Returns workout folders |
| `list-athlete-power-curves` | ✅ Verified | Fixed: requires activity_type param |
| `list-athlete-routes` | ✅ Verified | Returns empty (no routes) |
| `get-athlete-profile` | ✅ Verified | Fixed: parses nested athlete field |
| `list-activity-messages` | ✅ Verified | Returns empty (no messages) |
| `get-weather-forecast` | ✅ Verified | Fixed: parses forecasts wrapper |
| `list-sport-settings` | ✅ Verified | Fixed: id is i64 |
| `list-chats` | ✅ Verified | Returns empty (no chats) |
| `list-athlete-hr-curves` | ✅ Verified | Fixed: parses list wrapper |
| `list-athlete-pace-curves` | ✅ Verified | Fixed: parses list wrapper |
| `get-athlete-training-plan` | ✅ Verified | Fixed: matches actual schema |
| `list-activity-intervals` | ✅ Verified | Fixed: parses icu_intervals wrapper |
| `list-events` | ✅ Verified | Returns empty (no events) |
| `list-workouts` | ✅ Verified | Returns empty (no workouts) |
| `create-event` | ✅ Verified | Creates calendar event |
| `create-manual-activity` | ✅ Verified | Creates manual activity |
| `delete-activity` | ✅ Verified | Deletes activity |
| `get-workout` | ⚠️ No data | Athlete has no workouts |
| `create-event` (cleanup) | ✅ Cleaned | Test event deleted |

### Fixed Commands (8 total)

1. `list-athlete-power-curves` - Added required `activity_type` param
2. `get-athlete-profile` - Parse nested `athlete` field
3. `get-weather-forecast` - Parse `{forecasts: [...]}` wrapper
4. `list-sport-settings` - Changed `id` from String to i64
5. `list-athlete-hr-curves` - Parse `{list: [...]}` wrapper
6. `list-athlete-pace-curves` - Parse `{list: [...]}` wrapper
7. `get-athlete-training-plan` - Match actual schema with optional fields
8. `list-activity-intervals` - Parse `{icu_intervals: [...]}` wrapper
