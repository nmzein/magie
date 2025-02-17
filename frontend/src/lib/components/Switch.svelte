<script lang="ts">
	let {
		checked = $bindable(),
		sliderLength = 35,
		thumbSize = 14,
		thumbGap = 2
	}: {
		checked: boolean;
		sliderLength?: number;
		thumbSize?: number;
		thumbGap?: number;
	} = $props();
</script>

<label
	id="switch"
	for="switch"
	style:height="{thumbSize + 2 * thumbGap}px"
	style:width="{sliderLength}px"
	class="relative"
>
	<input
		id="switch"
		type="checkbox"
		bind:checked
		class="absolute z-[1] h-full w-full cursor-pointer opacity-0"
	/>
	<span
		id="slider"
		class="bg-primary/40 absolute inset-0 rounded-full transition-colors duration-300"
	>
		<span
			id="thumb"
			style:height="{thumbSize}px"
			style:width="{thumbSize}px"
			style:bottom="{thumbGap}px"
			style:left="{thumbGap}px"
			style:--translate="{sliderLength - thumbSize - 2 * thumbGap}px"
			class="bg-primary absolute rounded-full transition-transform duration-300"
		></span>
	</span>
</label>

<style>
	input:checked + #slider {
		background-color: var(--color-accent);
	}

	input:checked + #slider #thumb {
		transform: translateX(var(--translate));
	}
</style>
