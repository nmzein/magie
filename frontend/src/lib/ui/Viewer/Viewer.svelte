<script lang="ts">
	import type Viewer from './viewer.svelte';

	let { viewer }: { viewer: Viewer } = $props();
</script>

<div
	id={viewer.id}
	style="cursor: {viewer.isDragging ? 'grab' : 'crosshair'};"
	class="absolute h-full w-full"
></div>
