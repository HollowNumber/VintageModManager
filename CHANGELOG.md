# Changelog

## v[0.4.0] - 2025-02-19

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