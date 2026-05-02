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
All planned commands implemented. 75 tests passing.
