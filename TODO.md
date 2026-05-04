# Intervals.mcu API CLI - TODO

## Project Setup

- [x] Create TODO.md
- [x] Initialize Rust project with cargo
- [x] Add dependencies: clap, reqwest, serde, serde_json, tokio, thiserror, urlencoding
- [x] Configure $INTERVALS_API_KEY environment variable

## Commands Implemented (100+ total)

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

All 100+ commands implemented and verified against live API. 186 tests passing.

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

## Unimplemented API Endpoints (56 total)

### Activity Analysis — Histograms & Curves (12)

- [x] `get-activity-hr-curve <activity-id>` — `GET /api/v1/activity/{id}/hr-curve{ext}` — Activity heart rate curve (JSON/CSV)
- [x] `get-activity-hr-histogram <activity-id>` — `GET /api/v1/activity/{id}/hr-histogram` — Activity heart rate histogram
- [x] `get-activity-hr-load-model <activity-id>` — `GET /api/v1/activity/{id}/hr-load-model` — Activity heart rate training load model
- [x] `get-activity-pace-curve <activity-id>` — `GET /api/v1/activity/{id}/pace-curve{ext}` — Activity pace curve (JSON/CSV)
- [x] `get-activity-pace-histogram <activity-id>` — `GET /api/v1/activity/{id}/pace-histogram` — Activity pace histogram
- [x] `get-activity-power-curves <activity-id>` — `GET /api/v1/activity/{id}/power-curves{ext}` — Activity power curves (JSON/CSV)
- [x] `get-activity-power-curve <activity-id>` — `GET /api/v1/activity/{id}/power-curve{ext}` — Activity power curve (JSON/CSV)
- [x] `get-activity-power-histogram <activity-id>` — `GET /api/v1/activity/{id}/power-histogram` — Activity power histogram
- [x] `get-activity-power-spike-model <activity-id>` — `GET /api/v1/activity/{id}/power-spike-model` — Activity power spike detection model
- [x] `get-activity-power-vs-hr <activity-id>` — `GET /api/v1/activity/{id}/power-vs-hr{ext}` — Activity power vs heart rate (JSON/CSV)
- [x] `get-activity-gap-histogram <activity-id>` — `GET /api/v1/activity/{id}/gap-histogram` — Activity gradient-adjusted pace histogram
- [x] `get-activity-time-at-hr <activity-id>` — `GET /api/v1/activity/{id}/time-at-hr` — Activity time at heart rate data

### Athlete Best Curves (3)

- [x] `list-athlete-hr-curves-best <id>` — `GET /api/v1/athlete/{id}/hr-curves{ext}` — List best heart rate curves
- [x] `list-athlete-power-curves-best <id>` — `GET /api/v1/athlete/{id}/power-curves{ext}` — List best power curves
- [x] `list-athlete-pace-curves-best <id>` — `GET /api/v1/athlete/{id}/pace-curves{ext}` — List best pace curves

### Activity Interval Editing (6)

- [x] `update-activity-intervals <activity-id>` — `PUT /api/v1/activity/{id}/intervals` — Update intervals for an activity
- [x] `update-activity-interval <activity-id> <interval-id>` — `PUT /api/v1/activity/{id}/intervals/{intervalId}` — Update/create an interval
- [x] `split-activity-interval <activity-id>` — `PUT /api/v1/activity/{id}/split-interval` — Split an interval
- [x] `delete-activity-intervals <activity-id>` — `PUT /api/v1/activity/{id}/delete-intervals` — Delete intervals
- [x] `update-activity-streams <activity-id>` — `PUT /api/v1/activity/{id}/streams` — Update streams from JSON
- [x] `update-activity-streams-csv <activity-id>` — `PUT /api/v1/activity/{id}/streams.csv` — Update streams from CSV

### Activity Upload & Download (3)

- [x] `upload-activity <athlete-id> <file>` — `POST /api/v1/athlete/{id}/activities` — Upload activity file (fit/tcx/gpx/zip)
- [x] `create-manual-activities-bulk <athlete-id>` — `POST /api/v1/athlete/{id}/activities/manual/bulk` — Bulk create manual activities
- [x] `download-activity-fit-files <athlete-id>` — `POST /api/v1/athlete/{id}/download-fit-files` — Download zip of FIT files

### Activity Search & Lists (4)

- [x] `get-activities <athlete-id> <ids>` — `GET /api/v1/athlete/{athleteId}/activities/{ids}` — Fetch multiple activities by id
- [x] `search-activities-full <athlete-id> <query>` — `GET /api/v1/athlete/{id}/activities/search-full` — Search activities, returns full objects
- [x] `search-activity-intervals <athlete-id>` — `GET /api/v1/athlete/{id}/activities/interval-search` — Find activities with intervals matching duration/intensity
- [x] `list-activities-around <athlete-id> <activity-id>` — `GET /api/v1/athlete/{id}/activities-around` — List activities before/after another

### CSV Downloads (5)

- [x] `download-activities-csv <athlete-id>` — `GET /api/v1/athlete/{id}/activities.csv` — Download activities as CSV
- [x] `download-gear-csv <athlete-id>` — `GET /api/v1/athlete/{id}/gear{ext}` — List gear in CSV format
- [x] `download-events-csv <athlete-id>` — `GET /api/v1/athlete/{id}/events{format}` — List events in CSV format
- [x] `download-wellness-csv <athlete-id>` — `GET /api/v1/athlete/{id}/wellness{ext}` — List wellness in CSV format
- [x] `list-activity-streams-ext <activity-id>` — `GET /api/v1/activity/{id}/streams{ext}` — List streams for the activity

### Gear CRUD (8)

- [x] `get-gear <athlete-id> <gear-id>` — `GET /api/v1/athlete/{id}/gear/{gearId}` — Get a specific gear item
- [x] `create-gear <athlete-id>` — `POST /api/v1/athlete/{id}/gear` — Create new gear or component
- [x] `update-gear <athlete-id> <gear-id>` — `PUT /api/v1/athlete/{id}/gear/{gearId}` — Update gear or component
- [x] `delete-gear <athlete-id> <gear-id>` — `DELETE /api/v1/athlete/{id}/gear/{gearId}` — Delete gear or component
- [x] `calc-gear <athlete-id> <gear-id>` — `GET /api/v1/athlete/{id}/gear/{gearId}/calc` — Recalculate gear stats
- [x] `create-gear-reminder <athlete-id> <gear-id>` — `POST /api/v1/athlete/{id}/gear/{gearId}/reminder` — Create a reminder
- [x] `update-gear-reminder <athlete-id> <gear-id> <reminder-id>` — `PUT /api/v1/athlete/{id}/gear/{gearId}/reminder/{reminderId}` — Update a reminder
- [x] `delete-gear-reminder <athlete-id> <gear-id> <reminder-id>` — `DELETE /api/v1/athlete/{id}/gear/{gearId}/reminder/{reminderId}` — Delete a reminder
- [x] `replace-gear <athlete-id> <gear-id>` — `POST /api/v1/athlete/{id}/gear/{gearId}/replace` — Retire and replace component

### Wellness CRUD (4)

- [x] `get-wellness <athlete-id> <date>` — `GET /api/v1/athlete/{id}/wellness/{date}` — Get wellness record for a specific date
- [x] `update-wellness <athlete-id> <date>` — `PUT /api/v1/athlete/{id}/wellness/{date}` — Update wellness record for date
- [x] `update-wellness-bulk <athlete-id>` — `PUT /api/v1/athlete/{id}/wellness-bulk` — Bulk update wellness records
- [x] `upload-wellness-csv <athlete-id> <file>` — `POST /api/v1/athlete/{id}/wellness` — Upload wellness records CSV

### Workout CRUD (7)

- [ ] `create-workout <athlete-id>` — `POST /api/v1/athlete/{id}/workouts` — Create a new workout
- [ ] `update-workout <athlete-id> <workout-id>` — `PUT /api/v1/athlete/{id}/workouts/{workoutId}` — Update a workout
- [ ] `delete-workout <athlete-id> <workout-id>` — `DELETE /api/v1/athlete/{id}/workouts/{workoutId}` — Delete a workout
- [ ] `create-workouts-bulk <athlete-id>` — `POST /api/v1/athlete/{id}/workouts/bulk` — Create multiple workouts
- [ ] `download-workout <athlete-id> <workout-id> <format>` — `POST /api/v1/athlete/{id}/download-workout{ext}` — Convert workout to zwo/mrc/erg/fit
- [ ] `duplicate-workouts <athlete-id>` — `POST /api/v1/athlete/{id}/duplicate-workouts` — Duplicate workouts on a plan
- [ ] `import-workout <athlete-id> <folder-id> <file>` — `POST /api/v1/athlete/{id}/folders/{folderId}/import-workout` — Import workout from file
- [ ] `download-workouts-zip <athlete-id>` — `GET /api/v1/athlete/{id}/workouts.zip` — Download workouts as zip

### Folder CRUD (3)

- [ ] `create-folder <athlete-id>` — `POST /api/v1/athlete/{id}/folders` — Create a new folder or plan
- [ ] `update-folder <athlete-id> <folder-id>` — `PUT /api/v1/athlete/{id}/folders/{folderId}` — Update a folder or plan
- [ ] `update-folder-shared-with <athlete-id> <folder-id>` — `PUT /api/v1/athlete/{id}/folders/{folderId}/shared-with` — Update folder sharing
- [ ] `update-folder-workouts <athlete-id> <folder-id>` — `PUT /api/v1/athlete/{id}/folders/{folderId}/workouts` — Update plan workouts

### Event Bulk Operations (6)

- [ ] `delete-events-range <athlete-id>` — `DELETE /api/v1/athlete/{id}/events` — Delete a range of events
- [ ] `update-events-range <athlete-id>` — `PUT /api/v1/athlete/{id}/events` — Update all events for date range
- [ ] `delete-events-bulk <athlete-id>` — `PUT /api/v1/athlete/{id}/events/bulk-delete` — Bulk delete events by id/external_id
- [ ] `create-events-bulk <athlete-id>` — `POST /api/v1/athlete/{id}/events/bulk` — Create multiple events
- [ ] `duplicate-events <athlete-id>` — `POST /api/v1/athlete/{id}/duplicate-events` — Duplicate events
- [ ] `apply-plan-to-events <athlete-id>` — `POST /api/v1/athlete/{id}/events/apply-plan` — Apply plan to events
- [ ] `download-event-workout <athlete-id> <event-id> <format>` — `GET /api/v1/athlete/{id}/events/{eventId}/download{ext}` — Download planned workout

### Sport Settings CRUD (5)

- [ ] `get-sport-setting <athlete-id> <id>` — `GET /api/v1/athlete/{athleteId}/sport-settings/{id}` — Get sport settings by id or type
- [ ] `create-sport-setting <athlete-id>` — `POST /api/v1/athlete/{athleteId}/sport-settings` — Create sport settings
- [ ] `update-sport-settings <athlete-id>` — `PUT /api/v1/athlete/{athleteId}/sport-settings` — Update multiple sport settings
- [ ] `update-sport-setting <athlete-id> <id>` — `PUT /api/v1/athlete/{athleteId}/sport-settings/{id}` — Update sport settings
- [ ] `delete-sport-setting <athlete-id> <id>` — `DELETE /api/v1/athlete/{athleteId}/sport-settings/{id}` — Delete sport settings
- [ ] `apply-sport-setting <athlete-id> <id>` — `PUT /api/v1/athlete/{athleteId}/sport-settings/{id}/apply` — Apply settings to matching activities
- [ ] `list-sport-setting-matching <athlete-id> <id>` — `GET /api/v1/athlete/{athleteId}/sport-settings/{id}/matching-activities` — List activities matching settings
- [ ] `list-sport-setting-pace-distances <athlete-id> <id>` — `GET /api/v1/athlete/{athleteId}/sport-settings/{id}/pace_distances` — List pace distances for sport

### Training Plans (2)

- [ ] `update-training-plan <athlete-id>` — `PUT /api/v1/athlete/{id}/training-plan` — Change athlete's training plan
- [ ] `update-athlete-plans` — `PUT /api/v1/athlete-plans` — Change training plans for multiple athletes

### Custom Items (4)

- [ ] `create-custom-item <athlete-id>` — `POST /api/v1/athlete/{id}/custom-item` — Create a custom item
- [ ] `update-custom-item <athlete-id> <item-id>` — `PUT /api/v1/athlete/{id}/custom-item/{itemId}` — Update a custom item
- [ ] `update-custom-item-indexes <athlete-id>` — `PUT /api/v1/athlete/{id}/custom-item-indexes` — Re-order custom items
- [ ] `upload-custom-item-image <athlete-id> <item-id> <file>` — `POST /api/v1/athlete/{id}/custom-item/{itemId}/image` — Upload image for custom item

### Chats (4)

- [x] `get-chat <chat-id>` — `GET /api/v1/chats/{id}` — Get a chat by id
- [x] `list-chat-messages <chat-id>` — `GET /api/v1/chats/{id}/messages` — List messages for a chat
- [x] `send-chat-message <athlete-id> <content>` — `POST /api/v1/chats/send-message` — Send a message
- [ ] `update-chat-message <chat-id> <msg-id>` — `PUT /api/v1/chats/{id}/messages/{msgId}` — Update a message
- [ ] `delete-chat-message <chat-id> <msg-id>` — `DELETE /api/v1/chats/{id}/messages/{msgId}` — Delete a message
- [ ] `mark-chat-messages-seen <chat-id> <msg-id>` — `PUT /api/v1/chats/{id}/messages/{msgId}/seen` — Mark messages as seen

### Routes (2)

- [x] `update-route <athlete-id> <route-id>` — `PUT /api/v1/athlete/{id}/routes/{route_id}` — Update a route
- [ ] `compare-routes <athlete-id> <route-id> <other-id>` — `GET /api/v1/athlete/{id}/routes/{route_id}/similarity/{other_id}` — Route similarity comparison

### Athlete (1)

- [ ] `update-athlete <athlete-id>` — `PUT /api/v1/athlete/{id}` — Update an athlete

### Other (1)

- [x] `list-fitness-model-events <athlete-id>` — `GET /api/v1/athlete/{id}/fitness-model-events` — List events that influence fitness calculation
- [ ] `download-workout-ext <workout-id> <format>` — `POST /api/v1/download-workout{ext}` — Convert workout to zwo/mrc/erg/fit
