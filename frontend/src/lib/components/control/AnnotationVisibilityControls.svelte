<script lang="ts">
	let { annotationLayer, annotationLayerIndex } = $props<{
		annotationLayer: AnnotationLayer;
		annotationLayerIndex: number;
	}>();

	import type { AnnotationLayer } from '$types';
	import Switch from '$control/Switch.svelte';

	let checked = $state(true);

	function toggleAnnotationLayer(event: MouseEvent) {
		event.preventDefault();
		const layer = document.getElementById('annotation-layer-' + annotationLayerIndex)?.style;

		if (layer?.visibility === 'hidden') {
			layer?.setProperty('visibility', 'visible');
			checked = true;
		} else {
			layer?.setProperty('visibility', 'hidden');
			checked = false;
		}
	}

	// TODO: Fix
	function updateAnnotationColour(event: Event) {
		event.preventDefault();
		annotationLayer.colours.fill = (event.target as HTMLInputElement)?.value + '99';
	}
</script>

<div style="display: flex; gap: 10px;">
	<input
		type="color"
		onchange={(e) => updateAnnotationColour(e)}
		value={annotationLayer.colours.fill.slice(0, -2)}
		style="--border: {annotationLayer.colours.stroke};"
	/>
	{annotationLayer.tag}
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<!-- svelte-ignore a11y-click-events-have-key-events -->
	<div onclick={(e) => toggleAnnotationLayer(e)}>
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
