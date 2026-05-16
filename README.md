# intervals

A CLI for the [intervals.icu](https://intervals.icu) API, written in Rust.

## Setup

```bash
export INTERVALS_API_KEY="your-api-key"
```

## Usage

```bash
# Get athlete profile
intervals get-athlete <athlete-id>

# List activities
intervals list-activities <athlete-id> --oldest 2024-01-01 --limit 10

# Create a manual activity
intervals create-manual-activity <athlete-id> \
  --start-date-local "2024-01-01T10:00:00" \
  --activity-type Run \
  --name "Morning Run" \
  --distance 5000 \
  --elapsed-time 1500

# Get activity streams
intervals get-activity-streams <activity-id> --types heartrate,time

# Download FIT file
intervals download-activity-fit-file <activity-id> output.fit
```

## Commands

### Athlete & Profile

| Command                                                    | Description                                                   |
| ---------------------------------------------------------- | ------------------------------------------------------------- |
| `get-athlete <id>`                                         | Get basic athlete profile                                     |
| `update-athlete <athlete-id> --data <json>`                | Update athlete profile                                        |
| `get-athlete-profile <id>`                                 | Get detailed athlete profile (city, country, timezone, sex)   |
| `get-athlete-summary <id> [--start <date>] [--end <date>]` | Get athlete summary (fitness data, CTL/ATL, training load)    |
| `get-athlete-settings <id> <device-class>`                 | Get settings for phone/tablet/desktop                         |
| `get-athlete-training-plan <id>`                           | Get current training plan                                     |
| `update-training-plan <athlete-id> --data <json>`          | Update athlete training plan                                  |
| `update-athlete-plans --data <json>`                       | Update training plans for multiple athletes                   |
| `apply-plan-changes <id>`                                  | Apply plan changes to calendar                                |
| `get-athlete-mmp-model <id> <activity-type>`               | Get MMP power model for a sport                               |
| `list-sport-settings <id>`                                 | List sport settings (FTP, LTHR per sport)                     |
| `get-sport-setting <athlete-id> <id>`                      | Get sport setting by ID                                       |
| `create-sport-setting <athlete-id>`                        | Create sport setting (`--types`, `--ftp`, `--lthr`, `--name`) |
| `update-sport-setting <athlete-id> <id>`                   | Update sport setting (`--types`, `--ftp`, `--lthr`, `--name`) |
| `delete-sport-setting <athlete-id> <id>`                   | Delete sport setting                                          |
| `apply-sport-setting <athlete-id> <id>`                    | Apply setting to matching activities                          |
| `list-sport-setting-matching <athlete-id> <id>`            | List activities matching setting                              |
| `list-sport-setting-pace-distances <athlete-id> <id>`      | List pace distances for sport                                 |
| `list-gear <id>`                                           | List athlete gear                                             |
| `get-gear <athlete-id> <gear-id>`                          | Get a specific gear item                                      |
| `create-gear <athlete-id>`                                 | Create new gear (`--name`, `--distance`, `--retired`)         |
| `update-gear <athlete-id> <gear-id>`                       | Update gear (`--name`, `--distance`, `--retired`)             |
| `delete-gear <athlete-id> <gear-id>`                       | Delete gear                                                   |
| `replace-gear <athlete-id> <gear-id>`                      | Retire and replace gear (`--new-gear-id`, `--retired-date`)   |
| `list-custom-items <id>`                                   | List custom items (charts, custom fields)                     |
| `get-custom-item <athlete-id> <item-id>`                   | Get a specific custom item                                    |
| `delete-custom-item <athlete-id> <item-id>`                | Delete a custom item                                          |
| `create-custom-item <athlete-id> --data <json>`            | Create a custom item                                          |
| `update-custom-item <athlete-id> <item-id> --data <json>`  | Update a custom item                                          |
| `update-custom-item-indexes <athlete-id> --data <json>`    | Re-order custom items                                         |
| `upload-custom-item-image <athlete-id> <item-id> <file>`   | Upload image for custom item                                  |

### Activities

| Command                                                              | Description                                                                                                                            |
| -------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- |
| `list-activities <id> --oldest <date>`                               | List activities (required: `--oldest`, optional: `--newest`, `--limit`, `--route-id`)                                                  |
| `list-activities-around <id> <activity-id>`                          | List activities before/after another (`--route-id`, `--limit`)                                                                         |
| `get-activity <id> <activity-id>`                                    | Get an activity's details                                                                                                              |
| `get-activities <id> <ids>`                                          | Fetch multiple activities by id (comma-separated, optional: `--intervals`)                                                             |
| `update-activity <activity-id>`                                      | Update activity (`--name`, `--description`, `--activity-type`, `--sport`, `--distance`, `--elapsed-time`)                              |
| `create-manual-activity <id>`                                        | Create manual activity (`--start-date-local`, `--activity-type`, `--name`, `--description`, `--sport`, `--distance`, `--elapsed-time`) |
| `delete-activity <activity-id>`                                      | Delete an activity                                                                                                                     |
| `search-activities <id> <query>`                                     | Search activities by name or tag                                                                                                       |
| `search-activities-full <id> <query>`                                | Search activities returning full details (`--limit`)                                                                                   |
| `search-activity-intervals <id>`                                     | Find activities with matching intervals (`--min-secs`, `--max-secs`, `--min-intensity`, `--max-intensity`)                             |
| `list-activity-messages <activity-id>`                               | List messages/comments on an activity                                                                                                  |
| `post-activity-message <activity-id> <content>`                      | Post a message to an activity                                                                                                          |
| `list-activity-intervals <activity-id>`                              | List auto-detected intervals within an activity                                                                                        |
| `list-activity-tags <id>`                                            | List all activity tags for an athlete                                                                                                  |
| `get-interval-stats <activity-id> <start-index> <end-index>`         | Get interval stats for a portion of an activity                                                                                        |
| `get-activity-best-efforts <activity-id> <stream>`                   | Find best efforts (`--duration`, `--distance`, `--count`)                                                                              |
| `get-activity-streams <activity-id>`                                 | Get time series data (`--types`, `--include-defaults`)                                                                                 |
| `get-activity-map <activity-id>`                                     | Get activity map data (lat/lng, bounds)                                                                                                |
| `get-activity-weather-summary <activity-id>`                         | Get weather summary for an activity                                                                                                    |
| `get-activity-segments <activity-id>`                                | Get activity segments                                                                                                                  |
| `get-activity-hr-curve <activity-id>`                                | Get activity heart rate curve                                                                                                          |
| `get-activity-hr-histogram <activity-id>`                            | Get activity heart rate histogram                                                                                                      |
| `get-activity-hr-load-model <activity-id>`                           | Get activity heart rate load model                                                                                                     |
| `get-activity-pace-curve <activity-id>`                              | Get activity pace curve                                                                                                                |
| `get-activity-pace-histogram <activity-id>`                          | Get activity pace histogram                                                                                                            |
| `get-activity-power-curves <activity-id>`                            | Get activity power curves                                                                                                              |
| `get-activity-power-curve <activity-id>`                             | Get activity power curve                                                                                                               |
| `get-activity-power-histogram <activity-id>`                         | Get activity power histogram                                                                                                           |
| `get-activity-power-spike-model <activity-id>`                       | Get activity power spike model                                                                                                         |
| `get-activity-power-vs-hr <activity-id>`                             | Get activity power vs heart rate                                                                                                       |
| `get-activity-gap-histogram <activity-id>`                           | Get activity gradient-adjusted pace histogram                                                                                          |
| `get-activity-time-at-hr <activity-id>`                              | Get activity time at heart rate                                                                                                        |
| `update-activity-intervals <activity-id> --intervals <json>`         | Update intervals for an activity                                                                                                       |
| `update-activity-interval <activity-id> <interval-id> --data <json>` | Update/create an interval                                                                                                              |
| `split-activity-interval <activity-id> --interval-id <id>`           | Split an interval                                                                                                                      |
| `delete-activity-intervals <activity-id> --interval-ids <ids>`       | Delete intervals                                                                                                                       |
| `update-activity-streams <activity-id> --data <json>`                | Update streams from JSON                                                                                                               |
| `update-activity-streams-csv <activity-id> --csv-file <path>`        | Update streams from CSV                                                                                                                |
| `download-activity-file <activity-id> <output>`                      | Download original activity file                                                                                                        |
| `download-activity-fit-file <activity-id> <output>`                  | Download FIT file (`--power`, `--hr`)                                                                                                  |
| `download-activity-gpx-file <activity-id> <output>`                  | Download GPX file (`--power`, `--hr`)                                                                                                  |
| `download-activities-csv <athlete-id> <output>`                      | Download all activities as CSV                                                                                                         |
| `upload-activity <athlete-id> <file>`                                | Upload activity file (fit/tcx/gpx/zip)                                                                                                 |
| `create-manual-activities-bulk <athlete-id> --activities <json>`     | Bulk create manual activities                                                                                                          |
| `download-activity-fit-files <athlete-id> --output <path>`           | Download all FIT files as zip                                                                                                          |
| `download-gear-csv <athlete-id> <output>`                            | Download gear as CSV                                                                                                                   |
| `download-events-csv <athlete-id> <output>`                          | Download events as CSV                                                                                                                 |
| `download-wellness-csv <athlete-id> <output>`                        | Download wellness as CSV                                                                                                               |
| `list-activity-streams-ext <activity-id>`                            | List activity streams                                                                                                                  |

### Wellness & Weather

| Command                                              | Description                                                                       |
| ---------------------------------------------------- | --------------------------------------------------------------------------------- |
| `list-wellness <id>`                                 | List wellness records (weight, resting HR, HRV, CTL/ATL)                          |
| `get-wellness <id> <date>`                           | Get wellness record for a specific date                                           |
| `update-wellness <id> <date>`                        | Update wellness record (`--weight`, `--resting-hr`, `--hrv`, `--mood`, `--sleep`) |
| `update-wellness-bulk <athlete-id> --records <json>` | Bulk update wellness records                                                      |
| `upload-wellness-csv <athlete-id> <file>`            | Upload wellness records from CSV                                                  |
| `get-weather-forecast <id>`                          | Get weather forecast for athlete's location                                       |
| `get-weather-config <id>`                            | Get weather forecast configuration                                                |
| `update-weather-config <id>`                         | Update weather forecast configuration (`--forecasts <json>`)                      |

### Workouts, Events & Calendar

| Command                                                                  | Description                                                                                                                       |
| ------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------- |
| `list-workouts <id>`                                                     | List workouts in athlete's library                                                                                                |
| `get-workout <id> <workout-id>`                                          | Get a specific workout                                                                                                            |
| `create-workout <athlete-id>`                                            | Create workout (`--name`, `--workout-type`, `--folder-id`)                                                                        |
| `update-workout <athlete-id> <workout-id>`                               | Update workout (`--name`, `--description`, `--workout-type`)                                                                      |
| `delete-workout <athlete-id> <workout-id>`                               | Delete a workout                                                                                                                  |
| `create-workouts-bulk <athlete-id> --workouts <json>`                    | Create multiple workouts                                                                                                          |
| `duplicate-workouts <athlete-id> --workout-ids <ids>`                    | Duplicate workouts                                                                                                                |
| `download-workouts-zip <athlete-id> <output>`                            | Download workouts as zip (`--oldest`, `--newest`, `--ext`)                                                                        |
| `download-workout <athlete-id> <workout-id> <output>`                    | Download a workout in specific format (`--ext`)                                                                                   |
| `download-workout-ext <workout-id> <output>`                             | Download a workout (no athlete context) (`--ext`)                                                                                 |
| `import-workout <athlete-id> <folder-id> <file>`                         | Import workout from file                                                                                                          |
| `list-folders <id>`                                                      | List workout folders and plans                                                                                                    |
| `list-folder-shared-with <athlete-id> <folder-id>`                       | List athletes a folder is shared with                                                                                             |
| `delete-folder <athlete-id> <folder-id>`                                 | Delete a folder and all its workouts                                                                                              |
| `create-folder <athlete-id>`                                             | Create folder (`--name`, `--description`, `--folder-type`)                                                                        |
| `update-folder <athlete-id> <folder-id>`                                 | Update folder (`--name`, `--description`, `--folder-type`)                                                                        |
| `update-folder-shared-with <athlete-id> <folder-id> --athlete-ids <ids>` | Update folder sharing                                                                                                             |
| `update-folder-workouts <athlete-id> <folder-id> --workout-ids <ids>`    | Update plan workouts                                                                                                              |
| `list-events <id>`                                                       | List calendar events (`--oldest`, `--newest`, `--category`, `--limit`)                                                            |
| `create-event <id>`                                                      | Create calendar event (`--start-date-local`, `--event-type`, `--category`, `--name`, `--description`, `--uid`, `--upsert-on-uid`) |
| `get-event <athlete-id> <event-id>`                                      | Get an event (planned workout, note, etc.)                                                                                        |
| `update-event <athlete-id> <event-id>`                                   | Update an event (`--name`, `--description`, `--notes`)                                                                            |
| `delete-event <athlete-id> <event-id>`                                   | Delete an event from calendar                                                                                                     |
| `mark-event-done <athlete-id> <event-id>`                                | Mark event as done (create manual activity from planned workout)                                                                  |
| `delete-events-range <athlete-id>`                                       | Delete events in date range (`--start-date`, `--end-date`)                                                                        |
| `update-events-range <athlete-id>`                                       | Update events in date range (`--start-date`, `--end-date`, `--name`, `--description`)                                             |
| `delete-events-bulk <athlete-id>`                                        | Bulk delete events (`--ids`, `--external-ids`)                                                                                    |
| `create-events-bulk <athlete-id> --events <json>`                        | Create multiple events                                                                                                            |
| `duplicate-events <athlete-id> --event-ids <ids>`                        | Duplicate events (`--num-copies`, `--weeks-between`)                                                                              |
| `apply-plan-to-events <athlete-id> --folder-id <id>`                     | Apply training plan to events                                                                                                     |
| `download-event-workout <athlete-id> <event-id> <output>`                | Download event workout (`--ext`)                                                                                                  |
| `list-event-tags <id>`                                                   | List event tags for an athlete                                                                                                    |
| `list-workout-tags <id>`                                                 | List workout tags for an athlete                                                                                                  |

### Performance Curves & Models

| Command                                               | Description                                          |
| ----------------------------------------------------- | ---------------------------------------------------- |
| `list-athlete-power-curves <id> <activity-type>`      | List power curves for a sport type (e.g., Ride, Run) |
| `list-athlete-hr-curves <id>`                         | List heart rate curves                               |
| `list-athlete-pace-curves <id>`                       | List pace curves                                     |
| `get-power-hr-curve <id> --start <date> --end <date>` | Get power vs heart rate curve                        |
| `list-pace-distances`                                 | List pace curve distances                            |
| `list-athlete-hr-curves-best <id>`                    | List best heart rate curves                          |
| `list-athlete-power-curves-best <id>`                 | List best power curves                               |
| `list-athlete-pace-curves-best <id>`                  | List best pace curves                                |

### Routes

| Command                                             | Description                                             |
| --------------------------------------------------- | ------------------------------------------------------- |
| `list-athlete-routes <id>`                          | List saved routes                                       |
| `get-route <athlete-id> <route-id>`                 | Get a route (`--include-path` for GPS data)             |
| `update-route <athlete-id> <route-id>`              | Update a route (`--name`, `--description`, `--commute`) |
| `compare-routes <athlete-id> <route-id> <other-id>` | Compare two routes for similarity                       |

### Chats

| Command                                            | Description                                         |
| -------------------------------------------------- | --------------------------------------------------- |
| `list-chats <id>`                                  | List chats                                          |
| `get-chat <chat-id>`                               | Get a chat by id                                    |
| `list-chat-messages <chat-id>`                     | List messages for a chat (`--before-id`, `--limit`) |
| `send-chat-message <athlete-id> <content>`         | Send a chat message (`--chat-id`)                   |
| `update-chat-message <chat-id> <msg-id> <content>` | Update a chat message                               |
| `delete-chat-message <chat-id> <msg-id>`           | Delete a chat message                               |
| `mark-chat-messages-seen <chat-id> <msg-id>`       | Mark messages as seen                               |

### Other

| Command                          | Description                                    |
| -------------------------------- | ---------------------------------------------- |
| `disconnect-app`                 | Disconnect the app from intervals.icu          |
| `get-shared-event <event-id>`    | Get a shared event (e.g. race)                 |
| `list-fitness-model-events <id>` | List events that influence fitness calculation |

## Installation

### For AI Agents (Claude, etc.)

Install the `intervals-cli` skill so agents can use this CLI on your behalf:

```bash
gh skill install yosmoc/intervals intervals-cli

# Pin to a specific version
gh skill install yosmoc/intervals intervals-cli --pin v0.0.9
```

### Install via Cargo

Install the `intervals` binary to use the CLI directly from your terminal:

```bash
# From crates.io (latest released version)
cargo install intervals-cli

# From GitHub (latest commits)
cargo install --git https://github.com/yosmoc/intervals.git
```

### Download from GitHub Releases

Pre-built binaries are available for Linux, macOS, and Windows on the
[Releases page](https://github.com/yosmoc/intervals/releases).

| Platform              | Archive                        |
| --------------------- | ------------------------------ |
| Linux (x86_64)        | `intervals-linux-amd64.tar.gz` |
| Linux (arm64)         | `intervals-linux-arm64.tar.gz` |
| macOS (x86_64)        | `intervals-macos-amd64.tar.gz` |
| macOS (Apple Silicon) | `intervals-macos-arm64.tar.gz` |
| Windows (x86_64)      | `intervals-windows-amd64.zip`  |

Extract the archive and place the `intervals` binary somewhere on your `PATH`.

### Build from Source

Requires [Rust](https://www.rust-lang.org/tools/install) (stable toolchain).

```bash
git clone https://github.com/yosmoc/intervals.git
cd intervals
cargo build --release
# Binary is at: target/release/intervals
```

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [Node.js](https://nodejs.org/) (for dev tooling: secretlint, oxfmt)

### Initial Setup

```bash
git clone https://github.com/yosmoc/intervals.git
cd intervals
npm install   # install dev tooling
```

### Build

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Test

```bash
# Run all tests
cargo test

# Run tests matching a pattern
cargo test <pattern>
```

Tests use [wiremock](https://github.com/LukeMathWalker/wiremock-rs) for HTTP
mocking. No live API access is required.

### Format

```bash
# Format code
cargo fmt --all

# Check formatting (CI mode)
cargo fmt --all -- --check
```
