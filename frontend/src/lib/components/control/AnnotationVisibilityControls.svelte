<script lang="ts">
	let { annotation } = $props<{ annotation: AnnotationLayer }>();

	import type { AnnotationLayer } from '$types';
	import Switch from '$control/Switch.svelte';

	let checked = $state(true);

	function toggleAnnotationLayer(event: MouseEvent) {
		event.preventDefault();
		let annotationLayer = document.getElementById('annotation-layer-' + annotation.tag)?.style;

		if (annotationLayer?.visibility === 'hidden') {
			annotationLayer?.setProperty('visibility', 'visible');
			checked = true;
		} else {
			annotationLayer?.setProperty('visibility', 'hidden');
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
		onchange={(e) => updateAnnotationColour(e)}
		value={annotation.colours.fill.slice(0, -2)}
		style="--border: {annotation.colours.stroke};"
	/>
	{annotation.tag}
	<Switch bind:checked onclick={(e) => toggleAnnotationLayer(e)} />
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
