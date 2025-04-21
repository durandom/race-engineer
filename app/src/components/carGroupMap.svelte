<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';

	type CarGroupMap = {
		group_id: number;
		car_id: number;
		id: number;
		name: string;
		ngp: number;
	};

	let carGroupMap: CarGroupMap[] = [];

	onMount(async () => {
		try {
			const data: string = await invoke('get_car_group_map');
			carGroupMap = JSON.parse(data);
		} catch (error) {
			console.error('Error fetching car group map:', error);
		}
	});
</script>

<div class="max-w-full rounded-lg bg-white">
	<h2 class="mb-3 text-2xl font-semibold text-gray-800">Car Group Map</h2>

	{#if carGroupMap.length > 0}
		<div class="h-[500px] overflow-auto">
			<table class="min-w-full table-auto text-left">
				<thead class="bg-indigo-600 text-white">
					<tr>
						<th class="sticky top-0 z-10 bg-indigo-600 px-4 py-2 text-sm font-medium">Group ID</th>
						<th class="sticky top-0 z-10 bg-indigo-600 px-4 py-2 text-sm font-medium">Car ID</th>
						<th class="sticky top-0 z-10 bg-indigo-600 px-4 py-2 text-sm font-medium">ID</th>
						<th class="sticky top-0 z-10 bg-indigo-600 px-4 py-2 text-sm font-medium">Name</th>
						<th class="sticky top-0 z-10 bg-indigo-600 px-4 py-2 text-sm font-medium">NGP</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-gray-200">
					{#each carGroupMap as { group_id, car_id, id, name, ngp } (id)}
						<tr class="hover:bg-indigo-50">
							<td class="px-4 py-3 text-sm text-gray-700">{group_id}</td>
							<td class="px-4 py-3 text-sm text-gray-600">{car_id}</td>
							<td class="px-4 py-3 text-sm text-gray-600">{id}</td>
							<td class="px-4 py-3 text-sm text-gray-600">{name}</td>
							<td class="px-4 py-3 text-sm text-gray-600">{ngp}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{:else}
		<p class="text-center text-gray-500 italic">No car group mappings available.</p>
	{/if}
</div>
