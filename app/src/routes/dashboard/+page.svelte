<script lang="ts">
	import Icon from '@iconify/svelte';
	import { load } from '@tauri-apps/plugin-store';
	import { open } from '@tauri-apps/plugin-dialog';
	import { onMount } from 'svelte';
	import Settings from '$components/settings.svelte';
	import CarGroupMap from '$components/carGroupMap.svelte';
	import CarGroups from '$components/carGroups.svelte';
	import CarModels from '$components/carModels.svelte';
	import CarData from '$components/carData.svelte';
	import Cars from '$components/cars.svelte';
	import RallySimFansPersonalStage from '$components/rallySimFansPersonalStage.svelte';
	import RallySimFansPersonalCar from '$components/rallySimFansPersonalCar.svelte';
	import StagesData from '$components/stagesData.svelte';

	const icons = [
		{ name: 'groups', icon: 'mdi:account-group' },
		{ name: 'profile', icon: 'mdi:map-marker-path' },
		{ name: 'car-models', icon: 'lucide:car' },
		{ name: 'car-data', icon: 'octicon:ai-model-16' },
		{ name: 'car', icon: 'mdi:car-estate' }, // Add this line
		{ name: 'stages_data', icon: 'fluent-mdl2:player-settings' },
		{ name: 'rally-sims-fans-personal-stage', icon: 'fluent:step-20-regular' }, // Add this line
		{ name: 'rally-sims-fans-personal-car', icon: 'carbon:rule-data-quality' },
		{ name: 'settings', icon: 'mdi:cog' }
	];

	let selected = 'groups';
	let directory = '';
	let store: Awaited<ReturnType<typeof load>>;

	// A key to trigger component remount when refresh happens
	let componentKey = 0;

	onMount(async () => {
		store = await load('settings.json');
		const saved = await store.get<string>('rbr_directory');
		if (saved) {
			directory = saved;
		}
	});

	const handleChangeDirectory = async () => {
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

	const shortenPath = (path: string) => {
		const parts = path.split('/');
		if (parts.length <= 3) return path;
		return `…/${parts.slice(-3).join('/')}`;
	};

	// Refresh function to update data and remount the component
	const refreshData = async () => {
		// Logic for refreshing data goes here (you can reload data or re-fetch from the store, etc.)
		// For example, if you're reloading the directory or settings:
		const saved = await store.get<string>('rbr_directory');
		if (saved) {
			directory = saved;
		}
		// Increment componentKey to trigger remount
		componentKey += 1;
	};
</script>

<div class="h-screen w-screen space-y-6 bg-[#F2F2F2] p-8">
	<header class="flex h-12 items-center justify-between">
		<div class="center-flex gap-8">
			<img src="/favicon.png" class="size-12 object-cover" alt="Logo" />
			<h1 class="text-3xl font-bold">Welcome to RBR Dashboard</h1>
		</div>

		<div class="center-flex space-x-3">
			{#if directory}
				<div class="hidden lg:block">
					<strong> Game Directory: </strong>
					<p class="break-words text-gray-700">{shortenPath(directory)}</p>
				</div>
				<button
					on:click={refreshData}
					class="center-flex gap-1 rounded-md bg-black px-4 py-2 text-white shadow hover:bg-slate-900 focus:ring-2 focus:ring-gray-500 focus:outline-none"
				>
					<Icon icon="mdi:refresh" class="size-5" />
					<span class="hidden lg:block">Refresh Data</span>
				</button>

				<button
					on:click={handleChangeDirectory}
					class="center-flex gap-1 rounded-md bg-orange-600 px-4 py-2 text-white shadow hover:bg-orange-700 focus:ring-2 focus:ring-orange-300 focus:outline-none"
				>
					<Icon icon="mdi:folder-edit" class="size-5" />
					<span class="hidden lg:block">Change Directory</span>
				</button>
			{:else}
				<p class="mb-4 text-gray-400 italic">No directory selected yet.</p>
				<button
					on:click={handleChangeDirectory}
					class="rounded-md bg-blue-600 px-4 py-2 text-white shadow hover:bg-blue-700 focus:ring-2 focus:ring-blue-300 focus:outline-none"
				>
					Select Directory
				</button>
			{/if}
		</div>
	</header>

	<main class="flex">
		<aside
			class="justify-left flex max-h-[calc(100vh-122px)] w-16 flex-col space-y-6 overflow-y-auto pr-4"
		>
			{#each icons as { name, icon } (name)}
				<button
					class="rounded-full p-3 transition-colors duration-300
      {selected === name ? 'bg-black' : 'bg-white'}"
					aria-label={name}
					on:click={() => (selected = name)}
				>
					<Icon
						{icon}
						class="mx-auto size-6 transition-colors duration-300
        {selected === name ? 'text-white' : 'text-black'}"
					/>
				</button>
			{/each}
		</aside>

		<section
			class="h-[calc(100vh-122px)] flex-1 overflow-hidden rounded-lg bg-white p-6"
			key={componentKey}
		>
			{#if selected === 'settings'}
				<Settings />
			{/if}
			{#if selected === 'groups'}
				<CarGroups />
			{/if}
			{#if selected === 'profile'}
				<CarGroupMap />
			{/if}
			{#if selected === 'car-models'}
				<CarModels />
			{/if}
			{#if selected === 'car-data'}
				<CarData />
			{/if}
			{#if selected === 'car'}
				<Cars />
			{/if}
			{#if selected === 'rally-sims-fans-personal-stage'}
				<RallySimFansPersonalStage />
			{/if}
			{#if selected === 'rally-sims-fans-personal-car'}
				<RallySimFansPersonalCar />
			{/if}
			{#if selected === 'stages_data'}
				<StagesData />
			{/if}
		</section>
	</main>
</div>
