<script lang="ts">
	import Switch from '$components/Switch.svelte';
	import Slider from '$components/Slider.svelte';
	import ColourPicker from '$components/ColourPicker.svelte';
	import type { Geometry2DLayer } from './types.ts';

	let { geometries = $bindable() }: { geometries: Geometry2DLayer[] } = $props();
</script>

<div class="panel ml-auto w-fit select-none">
	{#each geometries as layer}
		<div class="flex flex-col gap-[5px] p-[10px]">
			<div class="flex flex-row items-center gap-[10px]">
				<ColourPicker id={'fill-' + layer.tag} bind:value={layer.fill} />
				<span class="flex-1 text-sm">
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
					labelClass="w-7 text-right"
				/>
			</div>
		</div>
	{/each}
</div>
