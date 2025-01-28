<script lang="ts">
	import Button from '$components/Button.svelte';
	import Icon from '$icon';

	let {
		asset = $bindable(),
		placeholder
	}: {
		asset: File | undefined;
		placeholder: string;
	} = $props();

	function handleDrop(event: DragEvent) {
		event.preventDefault();
		asset = event.dataTransfer?.files?.[0];
	}

	function handleBrowse(event: Event) {
		asset = (event.target as HTMLInputElement).files?.[0];
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	ondrop={handleDrop}
	ondragover={(e) => e.preventDefault()}
	class="flex h-full w-full flex-1 flex-row items-center justify-center rounded-lg focus:outline-none"
>
	{#if asset}
		<Button
			class="bg-primary/10 hover:bg-primary/15 flex h-full flex-1 flex-col items-center justify-center gap-2 rounded-[inherit] transition-colors"
			onclick={() => (asset = undefined)}
		>
			<Icon name="image" class="h-20 w-20" />
			<span class="select-none overflow-hidden text-ellipsis whitespace-nowrap text-center text-sm">
				{asset.name}
			</span>
		</Button>
	{:else}
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<label
			class="border-tertiary hover:border-secondary flex h-full flex-1 cursor-pointer flex-col items-center justify-center gap-2 rounded-[inherit] border transition-colors"
			for="browse-input-{placeholder}"
			onclick={handleBrowse}
			onkeydown={(e) => e.key === 'Enter' && handleBrowse(e)}
		>
			<Icon name="image" class="h-20 w-20" />
			<span class="text-secondary max-w-52 select-none text-center text-[13px]">
				{placeholder}
			</span>
		</label>

		<input
			id="browse-input-{placeholder}"
			type="file"
			class="hidden"
			onchange={(e) => handleBrowse(e)}
		/>
	{/if}
</div>
