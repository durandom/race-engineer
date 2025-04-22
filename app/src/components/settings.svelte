<script lang="ts">
	import { invoke } from '@tauri-apps/api/core';
	import { onMount } from 'svelte';

	let settings: { key: string; value: string }[] = [];

	onMount(async () => {
		try {
			const settingsData: string = await invoke('get_settings');
			settings = JSON.parse(settingsData);
		} catch (error) {
			console.error('Error fetching settings:', error);
		}
	});
</script>

<div class="max-w-full rounded-lg bg-white">
	<h2 class="mb-3 text-2xl font-semibold text-gray-800">Settings</h2>

	{#if settings.length > 0}
		<div class="h-[500px] overflow-auto">
			<table class="min-w-full table-auto text-left">
				<thead class="bg-indigo-600 text-white">
					<tr>
						<th class="sticky top-0 z-10 bg-indigo-600 px-4 py-2 text-sm font-medium"> Key </th>
						<th class="sticky top-0 z-10 bg-indigo-600 px-4 py-2 text-sm font-medium"> Value </th>
					</tr>
				</thead>
				<tbody class="divide-y divide-gray-200">
					{#each settings as { key, value } (key)}
						<tr class="hover:bg-indigo-50">
							<td class="px-4 py-3 text-sm text-gray-700">{key}</td>
							<td class="px-4 py-3 text-sm text-gray-600">{value}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{:else}
		<p class="text-center text-gray-500 italic">No settings available.</p>
	{/if}
</div>
