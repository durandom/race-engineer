# Race Engineer Application — Requirements

## 1. Project Overview

A desktop application to view Richard Burns Rally (RBR) game state data, specifically cars and tracks, by reading JSON files directly from the RBR installation directory. The application does not use any local database; all data is displayed straight from the files. The only persistent configuration is the directory where Richard Burns Rally is installed.

## 2. Technology Stack

- **Tauri** (Rust backend)
- **SvelteKit** (frontend)
- **Skeleton** (UI framework) [https://www.skeleton.dev/](https://www.skeleton.dev/)
- **ts-rs** for TypeScript bindings to Rust ([Aleph-Alpha/ts-rs](https://github.com/Aleph-Alpha/ts-rs))
- No local database; all data is read directly from JSON files

## 3. Functional Requirements

- On first launch, prompt the user to select the RBR installation directory and persist this setting across reboots.
- Read and display car and track data from RBR JSON files located in the RBR directory.
- The application layout should follow a simple left-side navigation and main content area:
  - **Left-side navigation**:
    - **Cars and Tracks** section: lists all cars and tracks, each via its own Svelte component.
    - **Settings** section: allows user to configure the RBR directory.
  - **Main content area**: displays details for the selected car or track.
- No local database; all data is read live from files.
- **Separation of Concerns:**
  The UI (SvelteKit) must only handle user interactions and presentation. All functional and business logic, including file access and data processing, must be implemented in the Tauri (Rust) backend and exposed to the frontend via Tauri commands.
- The directory layout of the project should take inspiration from [mountain-loop/yaak](https://github.com/mountain-loop/yaak).

## 4. Testing Requirements

- All Svelte components must have unit tests.
- All Rust code that interacts with RBR JSON files must have unit tests.
- Emphasis on unit testing: all interactions with the RBR JSON files in the Rust part of the application must be unit tested.
- Focus on correctness of data reading and UI rendering.

## 5. Deliverables

- Complete source code for the Tauri/SvelteKit/Skeleton application.
- Basic setup instructions (README).
- All unit tests included and passing.