<script lang="ts">
	let { layer } = $props<{ layer: AnnotationLayer }>();

	import type { AnnotationLayer } from '$types';
	import { rgbToHex, rgbToCss, hexToRgb } from '$lib/rgb';

	import Switch from '$control/Switch.svelte';
	import Slider from '$control/Slider.svelte';

	function toggleAnnotationLayer(event: MouseEvent) {
		event.preventDefault();
		layer.visible = !layer.visible;
	}

	function updateAnnotationColour(event: Event) {
		event.preventDefault();
		const hex = (event.target as HTMLInputElement).value;
		layer.fill = hexToRgb(hex);
	}
</script>

<div style="display: flex; flex-direction: column; gap: 5px; padding: 10px;">
	<div style="display: flex; gap: 10px; align-items: center;">
		<div id="input-wrapper" style="--border: {rgbToCss(layer.stroke)};">
			<input
				type="color"
				onchange={(e) => updateAnnotationColour(e)}
				value={rgbToHex(layer.fill)}
			/>
		</div>
		<span style="flex: 1;">
			{layer.tag}
		</span>
		<button onclick={(e) => toggleAnnotationLayer(e)}>
			<Switch bind:checked={layer.visible} />
		</button>
	</div>
	<div style="display: flex; gap: 10px; align-items: center;">
		<span style="font-size: 15px;"> α </span>
		<Slider
			id={'opacity' + layer.tag}
			min={0}
			max={1}
			step={0.1}
			bind:value={layer.opacity}
			inputStyle="flex: 9;"
			labelStyle="flex: 1; text-align: right;"
		/>
	</div>
</div>

<style lang="scss">
	#input-wrapper {
		display: flex;
		border: 1px solid var(--border);
		border-radius: 50%;
	}

	input[type='color'] {
		padding: 0;
		width: 20px;
		height: 20px;
		border: 0;
		border-radius: 50%;
		cursor: pointer;
	}

	input[type='color']::-moz-color-swatch {
		border: none;
		border-radius: 50%;
	}

	input[type='color']::-webkit-color-swatch-wrapper {
		padding: 0;
		border-radius: 50%;
	}

	input[type='color']::-webkit-color-swatch {
		border: none;
		border-radius: 50%;
	}
</style>
