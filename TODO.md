# Intervals.mcu API CLI - TODO

## Project Setup
- [x] Create TODO.md
- [x] Initialize Rust project with cargo
- [x] Add dependencies: clap, reqwest, serde, serde_json, tokio, thiserror
- [x] Configure $INTERVALS_API_KEY environment variable

## Commands Implemented
- [x] `get-athlete <id>`
- [x] `list-activities <id> --oldest <date>`
- [x] `get-activity <id> <activityId>`
- [x] `list-workouts <id>`
- [x] `list-events <id>`
- [x] `list-wellness <id>`
- [x] `list-gear <id>`
- [x] `update-activity <activityId>`
- [x] `create-manual-activity <id>`
- [x] `delete-activity <activityId>`

## Phase 4: High Priority
- [x] `search-activities <id> --query <q>` - Search activities by name or tag
- [x] `list-folders <id>` - List workout folders, plans, and workouts
- [ ] `create-event <id>` - Create a planned workout/note on calendar
- [ ] `get-workout <id> <workoutId>` - Get a single workout from library
- [ ] `upload-activity <id> <file>` - Upload FIT/GPX/TCX file

## Phase 5: Medium Priority
- [ ] `list-athlete-power-curves <id>` - Best power curves for athlete
- [ ] `list-athlete-routes <id>` - List routes with activity counts
- [ ] `get-athlete-profile <id>` - Athlete profile info
- [ ] `list-activity-messages <id> <activityId>` - List comments on activity
- [ ] `get-weather-forecast <id>` - Weather forecast
- [ ] `list-sport-settings <athleteId>` - List sport settings

## Phase 6: Lower Priority
- [ ] `list-chats <id>` - List chats
- [ ] `list-athlete-hr-curves <id>` - Best HR curves
- [ ] `list-athlete-pace-curves <id>` - Best pace curves
- [ ] `get-athlete-training-plan <id>` - Get training plan
- [ ] `list-activity-intervals <id> <activityId>` - Get activity intervals

## Implementation Approach
- TDD: Write tests first, then implement
- One command at a time
- Commit after each command passes tests
- Use reqwest for HTTP calls
- Use clap for CLI argument parsing
- Output JSON by default

## Current Status
Implementing Phase 4: search-activities
