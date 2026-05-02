# Intervals.mcu API CLI - TODO

## Project Setup
- [x] Create TODO.md
- [x] Initialize Rust project with cargo
- [x] Add dependencies: clap, reqwest, serde, serde_json, tokio, thiserror
- [x] Configure $INTERVALS_API_KEY environment variable

## Commands to Implement

### Phase 1: Core Read Operations
- [x] `get-athlete <id>` - Get athlete profile and settings
- [x] `list-activities <id>` - List activities for a date range
- [x] `get-activity <id> <activityId>` - Get a single activity
- [x] `list-workouts <id>` - List workouts in athlete's library

### Phase 2: Write Operations
- [x] `update-activity <activityId>` - Update activity
- [x] `create-manual-activity <id>` - Create manual activity
- [x] `delete-activity <activityId>` - Delete activity

### Phase 3: Additional Features
- [x] `list-events <id>` - List calendar events
- [x] `list-wellness <id>` - List wellness records
- [x] `list-gear <id>` - List athlete gear

## Implementation Approach
- TDD: Write tests first, then implement
- One command at a time
- Commit after each command passes tests
- Use reqwest for HTTP calls
- Use clap for CLI argument parsing
- Output JSON by default

## Current Status
All planned commands implemented. 33 tests passing.
