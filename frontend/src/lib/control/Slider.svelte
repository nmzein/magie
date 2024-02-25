<script lang="ts">
	// Credit: https://www.w3schools.com/howto/howto_js_rangeslider.asp
	let { id, min, max, step, value, inputStyle, labelStyle } = $props<{
		id: string;
		min: number;
		max: number;
		step: number;
		value: number;
		inputStyle: string;
		labelStyle: string;
	}>();
</script>

<div style="display: flex; {inputStyle}">
	<input type="range" {id} name={id} {min} {max} {step} bind:value style="flex: 1;" />
</div>
<label for={id} style={labelStyle}>{value}</label>

<style lang="scss">
	div {
		height: 5px;
		border-radius: 10px;
		background: #c5c5c557;
		opacity: 0.7;
		transition: opacity 0.2s;
		margin: 15px 0;
		align-items: center;

		&:hover {
			opacity: 1;
			cursor: pointer;
		}
	}

	input {
		appearance: none;
		outline: none;
		background: transparent;
		height: 35px;
		margin: 0;
		cursor: pointer;

		&::-webkit-slider-thumb {
			appearance: none;
			width: 20px;
			height: 20px;
			border: none;
			border-radius: 50%;
			background: white;
			cursor: pointer;
		}

		&::-moz-range-thumb {
			width: 20px;
			height: 20px;
			border: none;
			border-radius: 50%;
			background: white;
			cursor: pointer;
		}
	}
</style>
