<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	type CarGroup = {
		id: string;
		name: string;
		user_id: string;
		main: string;
		test: string;
		ngp: string;
	};

	let carGroups: CarGroup[] = [];
	let error: string | null = null;

	onMount(async () => {
		try {
			const raw = await invoke<string>('get_car_groups');
			carGroups = JSON.parse(raw);
		} catch (err: unknown) {
			if (err instanceof Error) {
				error = `Error fetching car groups: ${err.message}`;
			} else {
				error = `Unknown error: ${JSON.stringify(err)}`;
			}
			console.error(error);
		}
	});
</script>

<div class="max-w-full rounded-lg bg-white">
	<h2 class="mb-3 text-2xl font-semibold text-gray-800">Car Groups</h2>

	{#if error}
		<p class="text-red-500">{error}</p>
	{:else if carGroups.length > 0}
		<div class="h-[500px] overflow-auto">
			<table class="min-w-full table-auto text-left">
				<thead class="sticky top-0 z-10 bg-indigo-600 text-white">
					<tr>
						<th class="sticky top-0 bg-indigo-600 px-4 py-2 text-sm font-medium">ID</th>
						<th class="sticky top-0 bg-indigo-600 px-4 py-2 text-sm font-medium">Name</th>
						<th class="sticky top-0 bg-indigo-600 px-4 py-2 text-sm font-medium">User ID</th>
						<th class="sticky top-0 bg-indigo-600 px-4 py-2 text-sm font-medium">Main</th>
						<th class="sticky top-0 bg-indigo-600 px-4 py-2 text-sm font-medium">Test</th>
						<th class="sticky top-0 bg-indigo-600 px-4 py-2 text-sm font-medium">NGP</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-gray-200">
					{#each carGroups as group (group.id)}
						<tr class="hover:bg-indigo-50">
							<td class="px-4 py-3 text-sm text-gray-700">{group.id}</td>
							<td class="px-4 py-3 text-sm text-gray-700">{group.name}</td>
							<td class="px-4 py-3 text-sm text-gray-700">{group.user_id}</td>
							<td class="px-4 py-3 text-sm text-gray-700">{group.main}</td>
							<td class="px-4 py-3 text-sm text-gray-700">{group.test}</td>
							<td class="px-4 py-3 text-sm text-gray-700">{group.ngp}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{:else}
		<p class="text-center text-gray-500 italic">No car groups available.</p>
	{/if}
</div>
