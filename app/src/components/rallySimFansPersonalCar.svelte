<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/core';

	interface CarOptions {
		car_id: string;
		name: string;
		rbrvr_seat0: string;
		rbrvr_seat1: string;
		rbrvr_seat2: string;
		fulldashposition_2d: string;
		fulldashposition_vr: string;
		fmod_mastervolume: string;
		setuptarmac: string;
		setupgravel: string;
		setupsnow: string;
		mirroredsteeringwheel: string;
		steeringrotation: string;
		forcefeedbacksensitivitytarmac: string;
		forcefeedbacksensitivitygravel: string;
		forcefeedbacksensitivitysnow: string;
	}

	let carOptions: CarOptions[] = [];
	let errorMessage: string | null = null; // Renamed error variable to avoid conflicts

	const display = (value: string | number | undefined) =>
		value === '' || value == null ? 'N/A' : value;

	onMount(async () => {
		try {
			const result = await invoke<string>('get_car_options');
			console.log('Raw car options data:', result);
			carOptions = JSON.parse(result);
		} catch (err) {
			// Use a new variable for the error message
			errorMessage = `Failed to load car options: ${(err as Error).message}`;
		}
	});
</script>

<div class="max-w-full rounded-lg bg-white shadow-lg">
	<h2 class="mb-3 text-2xl font-semibold text-gray-800">Car Options / Specs</h2>

	{#if errorMessage}
		<p class="text-red-500">{errorMessage}</p>
	{:else if carOptions.length === 0}
		<p class="text-gray-500 italic">Loading car options...</p>
	{:else}
		<div class="overflow-x-auto">
			<table class="min-w-full table-auto text-left">
				<thead class="bg-indigo-600 text-white">
					<tr>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Car ID</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Name</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Seat 0</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Seat 1</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Seat 2</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Dash 2D</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Dash VR</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Volume</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Setup Tarmac</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Setup Gravel</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Setup Snow</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Mirror Wheel</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">Steering</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">FFB Tarmac</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">FFB Gravel</th>
						<th class="sticky top-0 px-4 py-2 text-sm font-medium">FFB Snow</th>
					</tr>
				</thead>
				<tbody class="divide-y divide-gray-200">
					{#each carOptions as car, i (car.car_id ?? i)}
						<tr class="hover:bg-indigo-50">
							<td class="px-4 py-2 text-sm">{display(car.car_id)}</td>
							<td class="px-4 py-2 text-sm">{display(car.name)}</td>
							<td class="px-4 py-2 text-sm">{display(car.rbrvr_seat0)}</td>
							<td class="px-4 py-2 text-sm">{display(car.rbrvr_seat1)}</td>
							<td class="px-4 py-2 text-sm">{display(car.rbrvr_seat2)}</td>
							<td class="px-4 py-2 text-sm">{display(car.fulldashposition_2d)}</td>
							<td class="px-4 py-2 text-sm">{display(car.fulldashposition_vr)}</td>
							<td class="px-4 py-2 text-sm">{display(car.fmod_mastervolume)}</td>
							<td class="px-4 py-2 text-sm">{display(car.setuptarmac)}</td>
							<td class="px-4 py-2 text-sm">{display(car.setupgravel)}</td>
							<td class="px-4 py-2 text-sm">{display(car.setupsnow)}</td>
							<td class="px-4 py-2 text-sm">{display(car.mirroredsteeringwheel)}</td>
							<td class="px-4 py-2 text-sm">{display(car.steeringrotation)}</td>
							<td class="px-4 py-2 text-sm">{display(car.forcefeedbacksensitivitytarmac)}</td>
							<td class="px-4 py-2 text-sm">{display(car.forcefeedbacksensitivitygravel)}</td>
							<td class="px-4 py-2 text-sm">{display(car.forcefeedbacksensitivitysnow)}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	{/if}
</div>
