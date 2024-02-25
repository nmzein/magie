<script lang="ts">
	let { id, value } = $props<{ id: string; value: string }>();
</script>

<label for={id} style="--colour: {value};">
	<input type="color" {id} name={id} bind:value />
</label>

<style lang="scss">
	label {
		background-color: var(--colour);
		border-radius: 50%;
		width: 20px;
		height: 20px;
		cursor: pointer;
	}

	input[type='color'] {
		visibility: hidden;
		width: 20px;
		height: 20px;
		padding: 0;
		border: 0;
		border-radius: 50%;
	}
</style>
