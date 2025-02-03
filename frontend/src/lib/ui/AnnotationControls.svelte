<script lang="ts">
	import { images } from '$states';
	import Switch from '$components/Switch.svelte';
	import Slider from '$components/Slider.svelte';
	import ColourPicker from '$components/ColourPicker.svelte';
</script>

<div class="panel ml-auto w-fit select-none">
	<div class="text-secondary bg-primary/15 flex flex-row items-center rounded-t-[inherit] p-2.5">
		ANNOTATIONS
	</div>
	{#if images[0]?.initialised}
		{#each images[0].properties.annotations as layer}
			<!-- svelte-ignore a11y_no_static_element_interactions -->
			<div
				class="flex flex-col gap-[5px] p-[10px]"
				onmousemove={(e) => {
					if (images[0].transformer.isDragging) return;
					e.stopPropagation();
				}}
			>
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
	{/if}
</div>
