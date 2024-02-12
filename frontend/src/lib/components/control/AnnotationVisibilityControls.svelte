<script lang="ts">
	export let annotation: AnnotationLayer;

	import type { AnnotationLayer } from '$lib/types';
	import Switch from '$lib/components/control/Switch.svelte';

	let checked = true;

	function toggleAnnotationLayer(event: MouseEvent) {
		event.preventDefault();
		let annotation_layer = document.getElementById('annotation-layer-' + annotation.tag)?.style;

		if (annotation_layer?.visibility === 'hidden') {
			annotation_layer?.setProperty('visibility', 'visible');
			checked = true;
		} else {
			annotation_layer?.setProperty('visibility', 'hidden');
			checked = false;
		}
	}

	// TODO: Fix
	function updateAnnotationColour(event: Event) {
		event.preventDefault();
		annotation.colours.fill = (event.target as HTMLInputElement)?.value + '99';
	}
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div style="display: flex; gap: 10px;">
	<input
		type="color"
		on:change={(e) => updateAnnotationColour(e)}
		value={annotation.colours.fill.slice(0, -2)}
		style="--border: {annotation.colours.stroke};"
	/>
	{annotation.tag}
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div on:click={(e) => toggleAnnotationLayer(e)}>
		<Switch bind:checked />
	</div>
</div>

<style lang="scss">
	input[type='color'] {
		padding: 0;
		width: 20px;
		height: 20px;
		border: 1px solid var(--border);
		cursor: pointer;
	}
</style>
