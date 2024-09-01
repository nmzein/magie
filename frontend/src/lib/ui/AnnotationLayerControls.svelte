<script lang="ts">
	import type { AnnotationLayer } from '$types';
	import Switch from '$components/Switch.svelte';
	import Slider from '$components/Slider.svelte';
	import ColourPicker from '$components/ColourPicker.svelte';

	let { layer }: { layer: AnnotationLayer } = $props();
</script>

<div class="flex flex-col gap-[5px] p-[10px]">
	<div class="flex flex-row items-center gap-[10px]">
		<ColourPicker id={'fill-' + layer.tag} bind:value={layer.fill} />
		<span class="flex-1">
			{layer.tag}
		</span>
		<Switch bind:checked={layer.visible} />
	</div>
	<div class="flex flex-row items-center gap-[10px]">
		<span class="text-[15px]"> α </span>
		<Slider
			id={'opacity-' + layer.tag}
			min={0}
			max={1}
			step={0.1}
			bind:value={layer.opacity}
			inputClass="flex-[9]"
			labelClass="flex-1 text-right"
		/>
	</div>
</div>
