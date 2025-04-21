<script lang="ts">
	import { open } from '@tauri-apps/plugin-dialog';
	import { load } from '@tauri-apps/plugin-store';
	import { goto } from '$app/navigation'; // Import SvelteKit's `goto` function
	import Logo from '$components/logo.svelte';

	let directory = '';
	let store: Awaited<ReturnType<typeof load>>;

	// Load store and optionally prefill directory
	(async () => {
		store = await load('settings.json');

		const saved = await store.get<string>('rbr_directory');
		if (saved) {
			directory = saved;
			goto('/dashboard'); // Automatically redirect if directory already exists
		}
	})();

	const handleSelectDirectory = async () => {
		const selected = await open({
			directory: true,
			multiple: false,
			title: 'Select your RBR installation directory'
		});

		if (selected && typeof selected === 'string') {
			directory = selected;
			await store.set('rbr_directory', directory);
			await store.save();
		} else {
			alert('No directory selected.');
		}
	};

	const handleProceed = () => {
		if (directory) {
			goto('/dashboard'); // Redirect when user clicks Proceed
		}
	};
</script>

<div class="flex h-screen items-center justify-center gap-10 bg-gray-100">
	<!-- Left: Logo -->
	<div
		class="animate__animated animate__fadeIn animate__delay-0.2s hidden w-full max-w-sm text-center md:block"
	>
		<Logo />
	</div>

	<!-- Right: Form -->
	<div
		class="animate__animated animate__zoomIn animate__delay-0.2s w-full max-w-sm rounded-lg bg-white p-8 shadow-xl"
	>
		<h2 class="mb-6 text-center text-2xl font-semibold text-gray-800">Select RBR Directory</h2>

		<button
			on:click={handleSelectDirectory}
			class="w-full rounded-md bg-orange-600 py-3 font-semibold text-white shadow-md hover:bg-orange-700 focus:ring-4 focus:ring-orange-300 focus:outline-none"
		>
			Select Directory
		</button>

		{#if directory}
			<div
				class="mt-4 rounded-md border border-gray-300 bg-gray-50 p-4 text-left text-sm text-gray-700 shadow-inner"
			>
				<div class="mb-1 flex items-center font-medium text-gray-600">
					<strong>Selected Directory</strong>
				</div>
				<div class="mt-1 break-words text-gray-800">{directory}</div>
			</div>

			<button
				on:click={handleProceed}
				class="mt-6 w-full rounded-md bg-green-600 py-3 font-semibold text-white shadow-md hover:bg-green-700 focus:ring-4 focus:ring-green-300 focus:outline-none"
			>
				Proceed
			</button>
		{/if}
	</div>
</div>

<style>
	@import 'https://cdnjs.cloudflare.com/ajax/libs/animate.css/4.1.1/animate.min.css';
</style>
