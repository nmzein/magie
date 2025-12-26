<script lang="ts">
	import type Viewer from './viewer.svelte';

	let { instance = $bindable() }: { instance: Viewer } = $props();
</script>

<canvas
	id={instance.canvasId}
	style="cursor: {instance.isDragging ? 'grab' : 'crosshair'};"
	class="absolute h-full w-full"
></canvas>
