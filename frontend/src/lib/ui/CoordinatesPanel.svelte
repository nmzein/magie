<script lang="ts">
	import { defined } from '$helpers';

	let { x, y, z }: { x: number; y: number; z?: number } = $props();
</script>

<div class="panel absolute bottom-[10px] left-[10px] px-[7px] py-[3px] select-none">
	<span class="font-bold">x:</span>
	{x},
	<span class="font-bold">y:</span>
	{y}{#if defined(z)},
		<span class="font-bold">z:</span>
		{z}
	{/if}
</div>

<style>
	@media (hover: none) {
		div {
			/* Hide the element on touch-capable devices. */
			display: none;
		}
	}
</style>
