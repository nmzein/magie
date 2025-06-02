<script lang="ts">
	import { http } from '$api';
	import { defined } from '$helpers';
	import Geometry2DView from '$view/Geometry2D/View.svelte';
	import CoordinatesPanel from '$ui/CoordinatesPanel.svelte';
	import Layer from './Layer.svelte';
	import type { Image2DView } from './types.ts';

	let { view = $bindable() }: { view: Image2DView } = $props();

	let x = $state(0);
	let y = $state(0);

	function onmousedown(e: MouseEvent) {
		e.preventDefault();
		view.state.transformer.panStart(e.clientX, e.clientY);
	}

	function ontouchstart(te: TouchEvent) {
		const e = te.touches[0];
		view.state.transformer.panStart(e.clientX, e.clientY);
	}

	function onmousemove(e: MouseEvent) {
		e.preventDefault();

		// Logic for calculating the coordinates of the mouse pointer.
		if (!view.state.transformer.isDragging) {
			const imageDOMRect = document.getElementById('image-layer-0')?.getBoundingClientRect();
			if (!defined(imageDOMRect)) return;

			const xTemp = Math.floor(
				(e.clientX - view.state.transformer.offsetX) * (view.state.width / imageDOMRect.width)
			);
			const yTemp = Math.floor(
				(e.clientY - view.state.transformer.offsetY) * (view.state.height / imageDOMRect.height)
			);

			if (Number.isFinite(xTemp) && !isNaN(xTemp)) x = xTemp;
			if (Number.isFinite(yTemp) && !isNaN(yTemp)) y = yTemp;

			return;
		}

		view.state.transformer.pan(e.clientX, e.clientY);
	}

	function ontouchmove(te: TouchEvent) {
		const e = te.touches[0];
		view.state.transformer.pan(e.clientX, e.clientY);
	}
</script>

<svelte:document
	{onmousemove}
	{ontouchmove}
	onmouseup={() => view.state.transformer.panStop()}
	ontouchend={() => view.state.transformer.panStop()}
	onwheel={(e) => view.state.transformer.zoom(e.deltaY, e.clientX, e.clientY)}
/>

<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<div
	role="img"
	{onmousedown}
	{ontouchstart}
	class="absolute h-dvh overflow-hidden"
	style="cursor: {view.state.transformer.isDragging ? 'grab' : 'crosshair'};"
>
	<div
		class="relative h-full w-screen origin-top-left"
		style="transform: translate({view.state.transformer.offsetX}px, {view.state.transformer
			.offsetY}px) scale({view.state.transformer.scale});
			   {view.state.transformer.isDragging ? '' : 'transition: transform 0.2s;'}"
	>
		{#if view.state.geometries.length > 0}
			<Geometry2DView
				width={view.state.width}
				height={view.state.height}
				geometries={view.state.geometries}
				fetch={(layerId) => http.asset.geometry2d(view.state.storeId, view.state.id, layerId)}
			/>
		{/if}

		<div class="absolute z-10 h-full w-full">
			{#each view.state.layers as layer, layerIndex}
				<Layer
					{layer}
					{layerIndex}
					fetch={(l, x, y) => view.state.getTile(l, x, y)}
					display={layerIndex === view.state.transformer.currentLevel}
					zIndex={view.state.levels - layerIndex}
				/>
			{/each}
		</div>
	</div>
	<CoordinatesPanel {x} {y} />
</div>
