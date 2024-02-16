<script lang="ts">
	// Credit: https://www.w3schools.com/howto/howto_css_switch.asp
	export let checked: boolean;
</script>

<label class="switch">
	<input type="checkbox" {checked} on:click={() => (checked = !checked)} />
	<span class="slider round" />
</label>

<style lang="scss">
	.switch {
		position: relative;
		display: inline-block;
		width: 30px;
		height: 17px;

		// Hide default HTML checkbox.
		& input {
			opacity: 0;
			width: 0;
			height: 0;
		}
	}

	.slider {
		position: absolute;
		cursor: pointer;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(255, 255, 255, 0.4);
		-webkit-transition: 0.4s;
		transition: 0.4s;

		&:before {
			position: absolute;
			content: '';
			height: 13px;
			width: 13px;
			left: 2px;
			bottom: 2px;
			background-color: white;
			-webkit-transition: 0.4s;
			transition: 0.4s;
		}
	}

	input:checked + .slider {
		background-color: #2196f3;
	}

	input:focus + .slider {
		box-shadow: 0 0 1px #2196f3;
	}

	input:checked + .slider:before {
		-webkit-transform: translateX(13px);
		-ms-transform: translateX(13px);
		transform: translateX(13px);
	}

	.slider.round {
		border-radius: 34px;
	}

	.slider.round:before {
		border-radius: 50%;
	}
</style>
