# Changelog

## [v0.7.2] 2025-08-18

### Major Features

- **Enhanced Version Compatibility System**: Implemented comprehensive game version filtering and compatibility checks
  for mod downloads and updates
- **Improved Error Handling**: Added graceful handling for non-existent mods with proper 404 response parsing
- **Smart Update Logic**: Updates now prioritise version compatibility and only suggest appropriate mod versions for the
  current game version

### API & Client Improvements

- **Renamed ApiError to ClientError**: Cleaned up error handling architecture for better consistency
- **Generic Function Implementation**: Reduced code duplication by making functions more generic where applicable
- **Enhanced Mod Update Logic**: Improved version filtering based on detected game version with fallback mechanisms

### Configuration & Version Management

- **New Configuration Manager**: Implemented dedicated module with improved version detection logic `config_manager`
- **Game Version API Integration**: Added API method for dynamic version mapping `fetch_game_versions`
- **Fixed Version Mapping Logic**: Updated to use index as for proper version correlation `tagid`

#### COnfiguration Management

**New Configuration Module**: Introduced dedicated module with intelligent game version detection. Automatically detects
Vintage Story installations and correlates game versions with API compatibility tags. Stores configuration in
platform-appropriate directories with TOML format. `config_manager`
**Available Commands**:

- `config init [--force]` - Initialize configuration file
- `config set-path <path>` - Set game installation directory
- `config show` - Display current settings and detected version
- `config update-versions [--verbose]` - Fetch latest version mappings from API
- `config list-versions` - Show all supported game versions
- `config validate` - Check configuration integrity
- `config reset [--yes]` - Reset to defaults
- `config set-game-version <version>` - Manual version override

**Automatic Integration**: Configuration seamlessly integrates with download, update, and search operations.
Automatically filters mod results by detected game version. Updates only suggest compatible mod versions. Search results
pre-filtered for version compatibility.
**Version Detection**: Multi-step detection process analyzes installation directory, reads version from game files, maps
to API version tags, and maintains local compatibility cache. Provides intelligent fallbacks when exact version matches
unavailable.
**Cross-Platform Support**: Handles Windows (`%APPDATA%`), Linux (`~/.config`), and macOS (
`~/Library/Application Support`) configuration directories. Validates paths and provides recovery options for corrupted
configurations.

### Code Quality & Maintenance

- Applied Cargo clippy recommendations and formatting
- Cleaned up Cargo.lock formatting
- General code cleanup and refactoring

## [v0.6.4] 2025-02-25

God is testing me

## [v0.6.3] 2025-02-25

Accidentally removed a token field.

## [v0.6.2] 2025-02-25

- Fixed a bug in the release action

## [v0.6.1] 2025-02-25

- Fixed a bug in the release action

## [v0.6.0] 2025-02-25

### Changes

- Added confirmation prompt before downloading mods in the `download` command

### Additions

- Added pagination support to mod browsing in the `download` command:
  - New interactive prompt for browsing and selecting mods from moddb
  - Shows mod details including name, author and downloads
  - Added filtering capability while browsing mods
  - Added proper exit handling
  - Clean screen handling for better UX
- Added interactivity to `export` command:
  - New `--interactive` flag for selecting specific mods to export
  - Multi-select interface for choosing mods

## [v0.5.4] - 2025-02-20

### Changes

- Renamed readme.md to README.md to make it more visible on GitHub
- Updated cargo.toml to include the new README.md file
- Ran fmt on the project to make sure everything is formatted correctly

### Additions

+ Added clippy configuration to the project
+ Added rustfmt configuration to the project

## [v0.5.3] - 2025-02-20

### Changes

- Wrote more information in the Readme about the new flags and how to use them
- Version bump to 0.5.3

## [v0.5.2] - 2025-02-20

### Additions

+ Added support for the `--mods` flag in the `download` command
+ Added support for the `--mod` flag in the `download` command
+ Added `DownloadOptions` struct to hold the download options

### Changes

- Refactored `import_mods` function to use `DownloadOptions` struct
- Split up the `import_mods` function into smaller functions to allow the usage of the new flags.
- Updated `Readme` to reflect the new changes

## [v0.4.1] - 2025-02-19

- Fixed api data deserialization issue

## [v0.4.0] - 2025-02-19

- Version bump to 0.4.0

## [v0.1.5] - 2025-02-19

- Potentially fixed the trailing comma issue when deserializing mod infos from the mod folder

## [v0.1.4] - 2025-02-19

- Renamed `ModOptions` struct to `CliOptions`
- Fixed `None` value in `CliOptions` struct causing a panic when running the program without any arguments
- Added Default trait to `CliOptions` struct to allow for default values
- Added `CliOptions` support to `Export` command

## [v0.1.3] - 2025-02-19

- Added proper build target to the CI/CD pipeline

## [v0.1.2] - 2025-02-19

- Attempting to fix the build process