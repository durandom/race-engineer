
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * _Unlike_ [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 * 
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * 
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 * 
 * You can override `.env` values from the command line like so:
 * 
 * ```bash
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const SHELL: string;
	export const npm_command: string;
	export const WINDOWID: string;
	export const COLORTERM: string;
	export const HYPRLAND_CMD: string;
	export const XDG_SESSION_PATH: string;
	export const NVM_INC: string;
	export const TERM_PROGRAM_VERSION: string;
	export const XDG_BACKEND: string;
	export const ZSH_CACHE_DIR: string;
	export const TMUX: string;
	export const _P9K_TTY: string;
	export const NODE: string;
	export const TAURI_ENV_DEBUG: string;
	export const npm_config_verify_deps_before_run: string;
	export const P9K_TTY: string;
	export const TMUX_PLUGIN_MANAGER_PATH: string;
	export const DESKTOP_SESSION: string;
	export const HL_INITIAL_WORKSPACE_TOKEN: string;
	export const XCURSOR_SIZE: string;
	export const PMSPEC: string;
	export const MACOSX_DEPLOYMENT_TARGET: string;
	export const XDG_SEAT: string;
	export const PWD: string;
	export const XDG_SESSION_DESKTOP: string;
	export const LOGNAME: string;
	export const TAURI_ENV_PLATFORM: string;
	export const XDG_SESSION_TYPE: string;
	export const MOTD_SHOWN: string;
	export const HOME: string;
	export const LANG: string;
	export const _JAVA_AWT_WM_NONREPARENTING: string;
	export const XDG_CURRENT_DESKTOP: string;
	export const npm_package_version: string;
	export const WAYLAND_DISPLAY: string;
	export const XDG_SEAT_PATH: string;
	export const INIT_CWD: string;
	export const ALACRITTY_SOCKET: string;
	export const npm_lifecycle_script: string;
	export const NVM_DIR: string;
	export const TAURI_ENV_PLATFORM_VERSION: string;
	export const ZPFX: string;
	export const XDG_SESSION_CLASS: string;
	export const TAURI_ENV_FAMILY: string;
	export const term: string;
	export const TERM: string;
	export const npm_package_name: string;
	export const USER: string;
	export const npm_config_frozen_lockfile: string;
	export const TMUX_PANE: string;
	export const HYPRLAND_INSTANCE_SIGNATURE: string;
	export const TAURI_ENV_TARGET_TRIPLE: string;
	export const DISPLAY: string;
	export const npm_lifecycle_event: string;
	export const SHLVL: string;
	export const NVM_CD_FLAGS: string;
	export const MOZ_ENABLE_WAYLAND: string;
	export const TAURI_CLI_VERBOSITY: string;
	export const _P9K_SSH_TTY: string;
	export const XDG_VTNR: string;
	export const XDG_SESSION_ID: string;
	export const npm_config_user_agent: string;
	export const PNPM_SCRIPT_SRC_DIR: string;
	export const npm_execpath: string;
	export const XDG_RUNTIME_DIR: string;
	export const NODE_PATH: string;
	export const DEBUGINFOD_URLS: string;
	export const TAURI_ENV_ARCH: string;
	export const P9K_SSH: string;
	export const PATH: string;
	export const npm_config_node_gyp: string;
	export const ALACRITTY_LOG: string;
	export const DBUS_SESSION_BUS_ADDRESS: string;
	export const NVM_BIN: string;
	export const MAIL: string;
	export const npm_config_registry: string;
	export const ALACRITTY_WINDOW_ID: string;
	export const npm_node_execpath: string;
	export const OLDPWD: string;
	export const HYPRCURSOR_SIZE: string;
	export const TERM_PROGRAM: string;
	export const NODE_ENV: string;
}

/**
 * Similar to [`$env/static/private`](https://svelte.dev/docs/kit/$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Values are replaced statically at build time.
 * 
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/main/packages/adapter-node) (or running [`vite preview`](https://svelte.dev/docs/kit/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://svelte.dev/docs/kit/configuration#env) (if configured).
 * 
 * This module cannot be imported into client-side code.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 * 
 * > In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		SHELL: string;
		npm_command: string;
		WINDOWID: string;
		COLORTERM: string;
		HYPRLAND_CMD: string;
		XDG_SESSION_PATH: string;
		NVM_INC: string;
		TERM_PROGRAM_VERSION: string;
		XDG_BACKEND: string;
		ZSH_CACHE_DIR: string;
		TMUX: string;
		_P9K_TTY: string;
		NODE: string;
		TAURI_ENV_DEBUG: string;
		npm_config_verify_deps_before_run: string;
		P9K_TTY: string;
		TMUX_PLUGIN_MANAGER_PATH: string;
		DESKTOP_SESSION: string;
		HL_INITIAL_WORKSPACE_TOKEN: string;
		XCURSOR_SIZE: string;
		PMSPEC: string;
		MACOSX_DEPLOYMENT_TARGET: string;
		XDG_SEAT: string;
		PWD: string;
		XDG_SESSION_DESKTOP: string;
		LOGNAME: string;
		TAURI_ENV_PLATFORM: string;
		XDG_SESSION_TYPE: string;
		MOTD_SHOWN: string;
		HOME: string;
		LANG: string;
		_JAVA_AWT_WM_NONREPARENTING: string;
		XDG_CURRENT_DESKTOP: string;
		npm_package_version: string;
		WAYLAND_DISPLAY: string;
		XDG_SEAT_PATH: string;
		INIT_CWD: string;
		ALACRITTY_SOCKET: string;
		npm_lifecycle_script: string;
		NVM_DIR: string;
		TAURI_ENV_PLATFORM_VERSION: string;
		ZPFX: string;
		XDG_SESSION_CLASS: string;
		TAURI_ENV_FAMILY: string;
		term: string;
		TERM: string;
		npm_package_name: string;
		USER: string;
		npm_config_frozen_lockfile: string;
		TMUX_PANE: string;
		HYPRLAND_INSTANCE_SIGNATURE: string;
		TAURI_ENV_TARGET_TRIPLE: string;
		DISPLAY: string;
		npm_lifecycle_event: string;
		SHLVL: string;
		NVM_CD_FLAGS: string;
		MOZ_ENABLE_WAYLAND: string;
		TAURI_CLI_VERBOSITY: string;
		_P9K_SSH_TTY: string;
		XDG_VTNR: string;
		XDG_SESSION_ID: string;
		npm_config_user_agent: string;
		PNPM_SCRIPT_SRC_DIR: string;
		npm_execpath: string;
		XDG_RUNTIME_DIR: string;
		NODE_PATH: string;
		DEBUGINFOD_URLS: string;
		TAURI_ENV_ARCH: string;
		P9K_SSH: string;
		PATH: string;
		npm_config_node_gyp: string;
		ALACRITTY_LOG: string;
		DBUS_SESSION_BUS_ADDRESS: string;
		NVM_BIN: string;
		MAIL: string;
		npm_config_registry: string;
		ALACRITTY_WINDOW_ID: string;
		npm_node_execpath: string;
		OLDPWD: string;
		HYPRCURSOR_SIZE: string;
		TERM_PROGRAM: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * Similar to [`$env/dynamic/private`](https://svelte.dev/docs/kit/$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://svelte.dev/docs/kit/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 * 
 * Dynamic environment variables cannot be used during prerendering.
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
