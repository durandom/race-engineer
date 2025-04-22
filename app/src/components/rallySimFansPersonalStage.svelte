<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	type StageEntry = {
		stage_id: string;
		name: string;
		length: string;
		weather: string;
		surface: string;
		difficulty: string;
		start_time: string;
		end_time: string;
		rally: string;
		path?: string;
	};

	let entries: StageEntry[] = [];
	let error = '';

	const display = (value: string | number | undefined) =>
		value === '' || value == null ? 'N/A' : value;

	onMount(async () => {
		try {
			const raw = await invoke<string>('get_stage');
			entries = JSON.parse(raw);
		} catch (err) {
			error = `Failed to load stage data: ${(err as Error).message}`;
		}
	});
</script>

<div class="max-w-full rounded-lg bg-white">
	<h2 class="mb-3 text-2xl font-semibold text-gray-800">Stage Data</h2>

	{#if error}
		<p class="text-red-500">{error}</p>
	{:else if entries.length === 0}
		<p class="text-gray-500 italic">Loading stage data...</p>
	{:else}
		<div class="overflow-x-auto">
			<table class="min-w-full table-auto text-left">
				<thead class="bg-indigo-600 text-white">
					<tr>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Stage ID</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Name</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Length</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Weather</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Surface</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Difficulty</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Start Time</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">End Time</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Rally</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Path</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-gray-200">
					{#each entries as stage (stage.stage_id)}
						<tr class="hover:bg-indigo-50">
							<td class="px-4 py-2 text-sm">{display(stage.stage_id)}</td>
							<td class="px-4 py-2 text-sm">{display(stage.name)}</td>
							<td class="px-4 py-2 text-sm">{display(stage.length)}</td>
							<td class="px-4 py-2 text-sm">{display(stage.weather)}</td>
							<td class="px-4 py-2 text-sm">{display(stage.surface)}</td>
							<td class="px-4 py-2 text-sm">{display(stage.difficulty)}</td>
							<td class="px-4 py-2 text-sm">{display(stage.start_time)}</td>
							<td class="px-4 py-2 text-sm">{display(stage.end_time)}</td>
							<td class="px-4 py-2 text-sm">{display(stage.rally)}</td>
							<td class="px-4 py-2 text-sm">{display(stage.path)}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>
