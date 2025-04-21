<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	type CarDataEntry = {
		car_id: string;
		power: string;
		torque: string;
		drive_train: string;
		engine: string;
		transmission: string;
		weight: string;
		wdf: string;
		steering_wheel: string;
		skin: string;
		model: string;
		audio: string;
		year: string;
		shifter_type: string;
		id: string;
	};

	let entries: CarDataEntry[] = [];
	let error = '';
	let rawError: string | null = null;

	onMount(async () => {
		try {
			const raw = await invoke<string>('get_car_data');
			entries = JSON.parse(raw);
		} catch (err) {
			rawError = (err as Error).message;
			error = `Failed to load car data: ${rawError}`;
			console.error('Tauri Error:', err);
		}
	});
</script>

<div class="max-w-full rounded-lg bg-white p-4">
	<h2 class="mb-3 text-2xl font-semibold text-gray-800">Car Data</h2>

	{#if error}
		<p class="text-red-500">{error}</p>
		{#if rawError}
			<p class="text-gray-500 italic">Raw error: {rawError}</p>
		{/if}
	{:else if entries.length === 0}
		<p class="text-gray-500 italic">Loading car data...</p>
	{:else}
		<div class="h-[500px] overflow-auto">
			<table class="min-w-full table-auto border border-gray-200 text-left">
				<thead class="bg-indigo-600 text-white">
					<tr>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">ID</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Model</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Power</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Torque</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Transmission</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Weight</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Year</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-gray-200">
					{#each entries as car (car.id)}
						<tr class="hover:bg-indigo-50">
							<td class="px-4 py-2 text-sm">{car.id}</td>
							<td class="px-4 py-2 text-sm">{car.model ?? 'N/A'}</td>
							<td class="px-4 py-2 text-sm">{car.power ?? 'N/A'}</td>
							<td class="px-4 py-2 text-sm">{car.torque ?? 'N/A'}</td>
							<td class="px-4 py-2 text-sm">{car.transmission ?? 'N/A'}</td>
							<td class="px-4 py-2 text-sm">{car.weight ?? 'N/A'}</td>
							<td class="px-4 py-2 text-sm">{car.year ?? 'N/A'}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>
