<script lang="ts">
	import { onMount } from 'svelte';
	import { image, metadata, annotations } from '$lib/stores';
	import AnnotationCanvas from '$lib/components/view/AnnotationCanvas.svelte';
	import ImageCanvas from '$lib/components/view/ImageCanvas.svelte';

	let isDragging = false;
	let mouseStartX: number;
	let mouseStartY: number;
	let offsetX = 0;
	let offsetY = 0;
	let scale = 1;

	onMount(() => {
		document.addEventListener('mousemove', handleMouseMove);
		document.addEventListener('mouseup', handleMouseUp);
		document.addEventListener('wheel', handleWheel);

		return () => {
			document.removeEventListener('mousemove', handleMouseMove);
			document.removeEventListener('mouseup', handleMouseUp);
			document.removeEventListener('wheel', handleWheel);
		};
	});

	function handleMouseDown(event: MouseEvent) {
		event.preventDefault();
		isDragging = true;
		mouseStartX = event.clientX;
		mouseStartY = event.clientY;
	}

	function handleMouseUp() {
		isDragging = false;
	}

	function handleMouseMove(event: MouseEvent) {
		if (!isDragging) {
			return;
		}

		event.preventDefault();

		const delta_x = event.clientX - mouseStartX;
		const delta_y = event.clientY - mouseStartY;

		offsetX += delta_x;
		offsetY += delta_y;

		mouseStartX = event.clientX;
		mouseStartY = event.clientY;
	}

	function handleWheel(event: WheelEvent) {
		// Calculate the new scale factor based on the wheel delta.
		const new_scale = scale + event.deltaY * -0.005;

		// Limit the scale factor within a reasonable range.
		scale = Math.min(Math.max(new_scale, 0.4), 50);
	}
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	id="view"
	on:mousedown={handleMouseDown}
	style="{$image && $image.length > 0 ? 'height: 100vh;' : ''} cursor: {isDragging
		? 'grab'
		: 'crosshair'};"
>
	<div
		id="container"
		style="height: auto; transform: translate({offsetX}px, {offsetY}px) scale({scale});"
	>
		{#if $metadata && $image && $image.length > 0}
			{#if $annotations}
				<AnnotationCanvas />
			{/if}
			<ImageCanvas />
		{/if}
	</div>
</div>
