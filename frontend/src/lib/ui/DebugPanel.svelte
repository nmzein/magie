<script lang="ts">
	import type { Dimensions, Point } from '$types';

	let { dims, offset, scale }: { dims: Dimensions; offset: Point; scale: number } = $props();
</script>

<div
	class="panel absolute bottom-2.5 left-2.5 z-1000 flex flex-col px-1.75 py-0.75 text-sm select-none"
>
	<div class="flex justify-between gap-5">
		<span class="font-bold">Offset</span>
		({Math.round(offset.x).toFixed(0)}, {Math.round(offset.y).toFixed(0)})
	</div>
	<div class="flex justify-between gap-5">
		<span class="font-bold">Dimensions</span>
		({Math.round(dims.width).toFixed(0)}, {Math.round(dims.height).toFixed(0)})
	</div>
	<div class="flex justify-between gap-5">
		<span class="font-bold">Scale</span>
		{scale.toFixed(6)}
	</div>
</div>

<style>
	@media (hover: none) {
		div {
			/* Hide the element on touch-capable devices. */
			display: none;
		}
	}
</style>
