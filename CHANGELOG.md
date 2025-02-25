# Changelog

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