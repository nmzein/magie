<script lang="ts">
	import Button from '$components/Button.svelte';
	import Icon from '$icon';

	let {
		asset = $bindable(),
		placeholder,
		onpopulate,
		onunpopulate
	}: {
		asset: File | undefined;
		placeholder: string;
		onpopulate?: () => void;
		onunpopulate?: () => void;
	} = $props();

	function ondrop(event: DragEvent) {
		event.preventDefault();
		asset = event.dataTransfer?.files?.[0];
		onpopulate?.();
	}

	function onbrowse(event: Event) {
		asset = (event.target as HTMLInputElement).files?.[0];
		onpopulate?.();
	}

	function onremove() {
		asset = undefined;
		onunpopulate?.();
	}
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	{ondrop}
	ondragover={(e) => e.preventDefault()}
	class="flex h-full w-full flex-1 flex-row items-center justify-center rounded-lg focus:outline-none"
>
	{#if asset}
		<Button
			class="bg-primary/10 hover:bg-primary/15 flex h-full flex-1 flex-col items-center justify-center gap-2 rounded-[inherit] transition-colors"
			onclick={onremove}
		>
			<Icon name="image" class="h-20 w-20" />
			<span class="overflow-hidden text-center text-sm text-ellipsis whitespace-nowrap select-none">
				{asset.name}
			</span>
		</Button>
	{:else}
		<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
		<label
			class="border-tertiary hover:border-secondary flex h-full flex-1 cursor-pointer flex-col items-center justify-center gap-2 rounded-[inherit] border transition-colors"
			for="browse-input-{placeholder}"
			onclick={onbrowse}
			onkeydown={(e) => e.key === 'Enter' && onbrowse(e)}
		>
			<Icon name="image" class="h-20 w-20" />
			<span class="text-secondary max-w-52 text-center text-[13px] select-none">
				{placeholder}
			</span>
		</label>

		<input id="browse-input-{placeholder}" type="file" class="hidden" onchange={onbrowse} />
	{/if}
</div>
