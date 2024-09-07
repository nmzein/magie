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
		setAsset(event.dataTransfer?.files);
	}

	function handleBrowse(event: Event) {
		setAsset((event.target as HTMLInputElement).files);
	}

	function setAsset(files: FileList | undefined | null) {
		if (files && files.length > 0) {
			asset = files[0];
		}
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	ondrop={(e) => handleDrop(e)}
	ondragover={(e) => e.preventDefault()}
	class="flex h-full w-full flex-1 flex-row items-center justify-center rounded-lg focus:outline-none"
>
	{#if asset}
		<Button
			class="bg-primary/10 flex h-full flex-1 flex-col items-center justify-center gap-2 rounded-[inherit]"
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
			class="border-primary/70 flex h-full flex-1 cursor-pointer flex-col items-center justify-center gap-2 rounded-[inherit] border-2 border-dashed"
			for={'browse-input-' + placeholder}
			onclick={(e) => handleBrowse(e)}
			onkeydown={(e) => e.key === 'Enter' && handleBrowse(e)}
		>
			<Icon name="image" class="h-20 w-20" />
			<span
				class="text-secondary select-none overflow-hidden text-ellipsis whitespace-nowrap text-center text-sm"
			>
				{placeholder}
			</span>
		</label>

		<input
			id={'browse-input-' + placeholder}
			type="file"
			class="hidden"
			onchange={(e) => handleBrowse(e)}
		/>
	{/if}
</div>
