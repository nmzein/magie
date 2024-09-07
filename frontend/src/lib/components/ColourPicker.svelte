<script lang="ts">
	let { id, value = $bindable() }: { id: string; value: string } = $props();
</script>

<label for={id} style:background-color={value} class="h-5 w-5 cursor-pointer rounded-full">
	<input
		type="color"
		{id}
		name={id}
		bind:value
		class="hidden h-5 w-5 rounded-full border-none p-0"
	/>
</label>
