<script lang="ts">
	let { layer } = $props<{ layer: AnnotationLayer }>();

	import type { AnnotationLayer } from '$types';

	import Switch from '$control/Switch.svelte';
	import Slider from '$control/Slider.svelte';
	import ColourPicker from '$control/ColourPicker.svelte';
</script>

<div style="display: flex; flex-direction: column; gap: 5px; padding: 10px;">
	<div style="display: flex; gap: 10px; align-items: center;">
		<ColourPicker id={'fill-' + layer.tag} bind:value={layer.fill} />
		<span style="flex: 1;">
			{layer.tag}
		</span>
		<Switch bind:checked={layer.visible} />
	</div>
	<div style="display: flex; gap: 10px; align-items: center;">
		<span style="font-size: 15px;"> α </span>
		<Slider
			id={'opacity-' + layer.tag}
			min={0}
			max={1}
			step={0.1}
			bind:value={layer.opacity}
			inputStyle="flex: 9;"
			labelStyle="flex: 1; text-align: right;"
		/>
	</div>
</div>
