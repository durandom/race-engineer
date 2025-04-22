<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	type CarModel = {
		id: number;
		name: string;
		path: string;
		filename: string;
		hash: string;
		fallback: number;
	};

	let carModels: CarModel[] = [];
	let error = '';

	onMount(async () => {
		try {
			const raw = await invoke<string>('get_car_models');
			carModels = JSON.parse(raw) as CarModel[];
		} catch (err) {
			error = `Failed to load car models: ${(err as Error).message}`;
			console.error(error);
		}
	});
</script>

<div class="max-w-full rounded-lg bg-white">
	<h2 class="mb-3 text-2xl font-semibold text-gray-800">Car Models</h2>

	{#if error}
		<p class="text-red-500">{error}</p>
	{:else if carModels.length > 0}
		<div class="h-[500px] overflow-auto">
			<table class="min-w-full table-auto text-left">
				<thead class="sticky top-0 z-10 bg-indigo-600 text-white">
					<tr>
						<th class="sticky top-0 bg-indigo-600 px-4 py-2 text-sm font-medium">ID</th>
						<th class="sticky top-0 bg-indigo-600 px-4 py-2 text-sm font-medium">Name</th>
						<th class="sticky top-0 bg-indigo-600 px-4 py-2 text-sm font-medium">Path</th>
						<th class="sticky top-0 bg-indigo-600 px-4 py-2 text-sm font-medium">Filename</th>
						<th class="sticky top-0 bg-indigo-600 px-4 py-2 text-sm font-medium">Hash</th>
						<th class="sticky top-0 bg-indigo-600 px-4 py-2 text-sm font-medium">Fallback</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-gray-200">
					{#each carModels as model (model.id)}
						<tr class="hover:bg-indigo-50">
							<td class="px-4 py-3 text-sm text-gray-700">{model.id}</td>
							<td class="px-4 py-3 text-sm text-gray-700">{model.name}</td>
							<td class="px-4 py-3 text-sm text-gray-700">{model.path}</td>
							<td class="px-4 py-3 text-sm text-gray-700">{model.filename}</td>
							<td class="px-4 py-3 text-sm text-gray-700">{model.hash}</td>
							<td class="px-4 py-3 text-sm text-gray-700">{model.fallback}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{:else}
		<p class="text-center text-gray-500 italic">No car models available.</p>
	{/if}
</div>
