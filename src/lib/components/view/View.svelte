<script lang="ts">
	import { onMount } from 'svelte';
	import { image, metadata, annotations } from '$lib/stores';
	import AnnotationCanvas from '$lib/components/view/AnnotationCanvas.svelte';
	import ImageCanvas from '$lib/components/view/ImageCanvas.svelte';

	let is_dragging = false;
	let mouse_start_x: number;
	let mouse_start_y: number;
	let offset_x = 0;
	let offset_y = 0;
	let scale = 1;

	onMount(() => {
		document.addEventListener('mousemove', HandleMouseMove);
		document.addEventListener('mouseup', HandleMouseUp);
		document.addEventListener('wheel', HandleWheel);

		return () => {
			document.removeEventListener('mousemove', HandleMouseMove);
			document.removeEventListener('mouseup', HandleMouseUp);
			document.removeEventListener('wheel', HandleWheel);
		};
	});

	function HandleMouseDown(event: MouseEvent) {
		event.preventDefault();
		is_dragging = true;
		mouse_start_x = event.clientX;
		mouse_start_y = event.clientY;
	}

	function HandleMouseUp() {
		is_dragging = false;
	}

	function HandleMouseMove(event: MouseEvent) {
		if (!is_dragging) {
			return;
		}

		event.preventDefault();

		const delta_x = event.clientX - mouse_start_x;
		const delta_y = event.clientY - mouse_start_y;

		offset_x += delta_x;
		offset_y += delta_y;

		mouse_start_x = event.clientX;
		mouse_start_y = event.clientY;
	}

	function HandleWheel(event: WheelEvent) {
		// Calculate the new scale factor based on the wheel delta.
		const new_scale = scale + event.deltaY * -0.005;

		// Limit the scale factor within a reasonable range.
		scale = Math.min(Math.max(new_scale, 0.4), 30);
	}
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div id="view" on:mousedown={HandleMouseDown} style={$image.length > 0 ? 'height: 100vh;' : ''}>
	<div id="container" style="--offset-x: {offset_x}px; --offset-y: {offset_y}px; --scale: {scale};">
		{#if $metadata && $image.length > 0}
			{#if $annotations}
				<AnnotationCanvas />
			{/if}
			<ImageCanvas />
		{/if}
	</div>
</div>

<style>
	#view {
		cursor: grab;
	}

	#container {
		height: auto;
		transform: translate(var(--offset-x), var(--offset-y)) scale(var(--scale));
	}
</style>
