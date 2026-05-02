# Intervals.mcu API CLI - TODO

## Project Setup
- [x] Create TODO.md
- [x] Initialize Rust project with cargo
- [x] Add dependencies: clap, reqwest, serde, serde_json, tokio, thiserror
- [x] Configure $INTERVALS_API_KEY environment variable

## Commands to Implement

### Phase 1: Core Read Operations
- [x] `get-athlete <id>` - Get athlete profile and settings
- [ ] `list-activities <id>` - List activities for a date range
- [ ] `get-activity <id> <activityId>` - Get a single activity
- [ ] `list-workouts <id>` - List workouts in athlete's library

### Phase 2: Write Operations
- [ ] `update-activity <id> <activityId>` - Update activity
- [ ] `create-manual-activity <id>` - Create manual activity
- [ ] `delete-activity <id> <activityId>` - Delete activity

### Phase 3: Additional Features
- [ ] `list-events <id>` - List calendar events
- [ ] `list-wellness <id>` - List wellness records
- [ ] `list-gear <id>` - List athlete gear

## Implementation Approach
- TDD: Write tests first, then implement
- One command at a time
- Commit after each command passes tests
- Use reqwest for HTTP calls
- Use clap for CLI argument parsing
- Output JSON by default

## Current Status
Working on: `list-activities` command (next)
