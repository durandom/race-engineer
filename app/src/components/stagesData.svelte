<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	interface StageData {
		stage_id: number;
		name: string;
		deftime: number;
		length: number;
		surface_id: number;
		short_country: string;
		author: string;
		tarmac: number;
		gravel: number;
		snow: number;
		new_update: number;
		author_web: string;
		author_note: string;
		fattrib: string;
	}

	let stages: StageData[] = [];
	let error = '';

	const display = (value: string | number | undefined) =>
		value === '' || value == null ? 'N/A' : value;

	onMount(async () => {
		try {
			const result = await invoke<string>('get_stages_data');
			console.log('Raw stage data:', result);
			stages = JSON.parse(result);
		} catch (err) {
			error = `Failed to load stage data: ${(err as Error).message}`;
		}
	});
</script>

<div class="max-w-full rounded-lg bg-white">
	<h2 class="mb-3 text-2xl font-semibold text-gray-800">Players Stages Data</h2>

	{#if error}
		<p class="text-red-500">{error}</p>
	{:else if stages.length === 0}
		<p class="text-gray-500 italic">Loading stages...</p>
	{:else}
		<div class="overflow-x-auto">
			<table class="min-w-full table-auto text-left">
				<thead class="bg-indigo-600 text-white">
					<tr>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">ID</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Name</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Time</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Length</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Surface</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Country</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Author</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Tarmac</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Gravel</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Snow</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Updated</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Website</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Note</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Flags</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-gray-200">
					{#each stages as s (s.stage_id)}
						<tr class="hover:bg-indigo-50">
							<td class="px-4 py-2 text-sm">{display(s.stage_id)}</td>
							<td class="px-4 py-2 text-sm">{display(s.name)}</td>
							<td class="px-4 py-2 text-sm">{display(s.deftime)}</td>
							<td class="px-4 py-2 text-sm">{display(s.length)}</td>
							<td class="px-4 py-2 text-sm">{display(s.surface_id)}</td>
							<td class="px-4 py-2 text-sm">{display(s.short_country)}</td>
							<td class="px-4 py-2 text-sm">{display(s.author)}</td>
							<td class="px-4 py-2 text-sm">{display(s.tarmac)}</td>
							<td class="px-4 py-2 text-sm">{display(s.gravel)}</td>
							<td class="px-4 py-2 text-sm">{display(s.snow)}</td>
							<td class="px-4 py-2 text-sm">{display(s.new_update)}</td>
							<td class="px-4 py-2 text-sm">
								{#if s.author_web}
									<a href={s.author_web} target="_blank" class="text-indigo-600 underline">
										Website
									</a>
								{:else}
									N/A
								{/if}
							</td>
							<td class="px-4 py-2 text-sm">{display(s.author_note)}</td>
							<td class="px-4 py-2 text-sm">{display(s.fattrib)}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>
