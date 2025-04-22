<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	type CarEntry = {
		id: string;
		name: string;
		path: string;
		hash: string;
		carmodel_id: string;
		user_id: string;
		base_group_id: string;
		test: string;
		ngp: string;
		custom_setups: string;
		rev: string;
		audio: string;
		audio_hash: string;
	};

	let entries: CarEntry[] = [];
	let error = '';

	const display = (value: string | number | null | undefined) =>
		value === '' || value === null || value === undefined ? 'N/A' : value.toString();

	onMount(async () => {
		try {
			const raw = await invoke<string>('get_car');
			const cars: CarEntry[] = JSON.parse(raw);

			entries = cars.map((car) => ({
				id: display(car.id),
				name: display(car.name),
				path: display(car.path),
				hash: display(car.hash),
				carmodel_id: display(car.carmodel_id),
				user_id: display(car.user_id),
				base_group_id: display(car.base_group_id),
				test: display(car.test),
				ngp: display(car.ngp),
				custom_setups: display(car.custom_setups),
				rev: display(car.rev),
				audio: display(car.audio),
				audio_hash: display(car.audio_hash)
			}));
		} catch (err) {
			error = `Failed to load car data: ${(err as Error).message}`;
		}
	});
</script>

<div class="max-w-full rounded-lg bg-white">
	<h2 class="mb-3 text-2xl font-semibold text-gray-800">Car Details</h2>

	{#if error}
		<p class="text-red-500">{error}</p>
	{:else if entries.length === 0}
		<p class="text-gray-500 italic">Loading car data...</p>
	{:else}
		<div class="h-[500px] overflow-auto">
			<table class="min-w-full table-auto text-left">
				<thead class="sticky top-0 z-10 bg-indigo-600 text-white">
					<tr>
						<th class="sticky top-0 px-4 py-2">ID</th>
						<th class="sticky top-0 px-4 py-2">Name</th>
						<th class="sticky top-0 px-4 py-2">Path</th>
						<th class="sticky top-0 px-4 py-2">Hash</th>
						<th class="sticky top-0 px-4 py-2">Car Model ID</th>
						<th class="sticky top-0 px-4 py-2">User ID</th>
						<th class="sticky top-0 px-4 py-2">Base Group ID</th>
						<th class="sticky top-0 px-4 py-2">Test</th>
						<th class="sticky top-0 px-4 py-2">NGP</th>
						<th class="sticky top-0 px-4 py-2">Custom Setups</th>
						<th class="sticky top-0 px-4 py-2">Rev</th>
						<th class="sticky top-0 px-4 py-2">Audio</th>
						<th class="sticky top-0 px-4 py-2">Audio Hash</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-gray-200">
					{#each entries as car (car.id)}
						<tr class="hover:bg-indigo-50">
							<td class="px-4 py-2">{car.id}</td>
							<td class="px-4 py-2">{car.name}</td>
							<td class="px-4 py-2">{car.path}</td>
							<td class="px-4 py-2">{car.hash}</td>
							<td class="px-4 py-2">{car.carmodel_id}</td>
							<td class="px-4 py-2">{car.user_id}</td>
							<td class="px-4 py-2">{car.base_group_id}</td>
							<td class="px-4 py-2">{car.test}</td>
							<td class="px-4 py-2">{car.ngp}</td>
							<td class="px-4 py-2">{car.custom_setups}</td>
							<td class="px-4 py-2">{car.rev}</td>
							<td class="px-4 py-2">{car.audio}</td>
							<td class="px-4 py-2">{car.audio_hash}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>
