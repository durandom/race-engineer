<script lang="ts">
	import Logo from '$components/logo.svelte';
	import Spinner from '$components/spinner.svelte';
	import { goto } from '$app/navigation';

	setTimeout(() => {
		goto('/platform');
	}, 1500);
</script>

<main>
	<div class="flex h-screen w-screen flex-col items-center justify-center gap-1">
		<Logo />
		<Spinner />
	</div>
</main>
