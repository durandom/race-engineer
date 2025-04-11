# Race Engineer Application

**Status:** Not yet implemented

This repository is intended for the development of a desktop application to view Richard Burns Rally (RBR) game state data (cars and tracks) by reading JSON files directly from the RBR installation directory. The application will be built using Tauri (Rust backend), SvelteKit (frontend), and Skeleton (UI framework), with TypeScript bindings generated via [ts-rs](https://github.com/Aleph-Alpha/ts-rs).

## Requirements

The full requirements for this application are described in [docs/requirements.md](docs/requirements.md).

## Implementation Details

- The application will not use a local database; all data is read directly from JSON files.
- The only persistent configuration is the RBR installation directory, which the user will select on first launch.
- The UI will follow a simple left-side navigation and main content area layout, with dedicated components for listing cars and tracks, and a settings section for configuring the RBR directory.
- All business logic and file access will be handled in the Tauri (Rust) backend, with the SvelteKit frontend focused solely on user interaction and presentation.
- The project structure will take inspiration from [mountain-loop/yaak](https://github.com/mountain-loop/yaak).
- There will be a strong emphasis on unit testing for both Svelte components and all Rust code that interacts with RBR JSON files.

## Development Workflow

- The implementation should be broken down into several steps.
- It is expected that multiple pull requests are created for workable and meaningful milestones during the application development.
- Each pull request should represent a logical, testable increment of functionality and be reviewed before merging.

## AI-Assisted Coding

This project will leverage AI-assisted coding practices. For more information, see [docs/ai-assisted-coding.md](docs/ai-assisted-coding.md).

---