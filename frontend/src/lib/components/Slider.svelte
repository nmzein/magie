<script lang="ts">
	import { twMerge } from 'tailwind-merge';

	let {
		id,
		min,
		max,
		step,
		value = $bindable(),
		inputClass,
		labelClass
	}: {
		id: string;
		min: number;
		max: number;
		step: number;
		value: number;
		inputClass: string;
		labelClass: string;
	} = $props();
</script>

<div
	class={twMerge(`my-[15px] flex h-[5px] cursor-pointer items-center rounded-[10px] ${inputClass}`)}
>
	<input type="range" {id} name={id} {min} {max} {step} bind:value class="flex-1" />
</div>
<label for={id} class={labelClass}>{value}</label>

<style>
	div {
		background: #c5c5c557;
		opacity: 0.7;
		transition: opacity 0.2s;
	}

	div:hover {
		opacity: 1;
	}

	input {
		appearance: none;
		outline: none;
		background: transparent;
		height: 35px;
		margin: 0;
		cursor: pointer;
	}

	input::-webkit-slider-thumb {
		appearance: none;
		width: 20px;
		height: 20px;
		border: none;
		border-radius: 50%;
		background: white;
		cursor: pointer;
	}

	input::-moz-range-thumb {
		width: 20px;
		height: 20px;
		border: none;
		border-radius: 50%;
		background: white;
		cursor: pointer;
	}
</style>
