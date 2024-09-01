<script lang="ts">
	// Credit: https://www.w3schools.com/howto/howto_css_switch.asp
	let { checked = $bindable() }: { checked: boolean } = $props();
</script>

<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<label id="switch" for="switch" onclick={() => (checked = !checked)}>
	<input id="switch" type="checkbox" {checked} />
	<span id="slider"></span>
</label>

<style>
	#switch {
		position: relative;
		display: inline-block;
		width: 30px;
		height: 17px;
	}

	/* Hide default HTML checkbox */
	#switch input {
		opacity: 0;
		width: 0;
		height: 0;
	}

	#slider {
		position: absolute;
		cursor: pointer;
		top: 0;
		left: 0;
		right: 0;
		bottom: 0;
		background-color: rgba(255, 255, 255, 0.4);
		transition: transform 0.3s;
		border-radius: 34px;
	}

	#slider:before {
		position: absolute;
		content: '';
		height: 13px;
		width: 13px;
		left: 2px;
		bottom: 2px;
		background-color: white;
		transition: transform 0.3s;
		border-radius: 50%;
	}

	input:checked + #slider {
		background-color: #2196f3;
	}

	input:focus + #slider {
		box-shadow: 0 0 1px #2196f3;
	}

	input:checked + #slider:before {
		transform: translateX(13px);
	}
</style>
