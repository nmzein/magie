<script lang="ts">
	import { viewerManager } from '$states';
	import type { ViewerState } from '$lib/states/viewer-manager.svelte';

	let { viewer }: { viewer: ViewerState } = $props();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	onmousedown={() => (viewerManager.active = viewer.instance.id)}
	ontouchstart={() => (viewerManager.active = viewer.instance.id)}
	onwheelcapture={() => (viewerManager.active = viewer.instance.id)}
	id={viewer.instance.id}
	style:width="{viewerManager.width}%"
	style:height="{viewerManager.height}%"
	style:cursor={viewer.instance.isDragging ? 'grab' : 'crosshair'}
	class="relative"
></div>
