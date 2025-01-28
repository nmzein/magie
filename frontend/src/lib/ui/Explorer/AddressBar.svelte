<script lang="ts">
	import Button from '$components/Button.svelte';
	import { explorer } from '$states';
</script>

<div
	class="bg-primary/15 flex h-[35px] flex-[6] select-none flex-row items-center gap-[5px] overflow-x-auto overflow-y-hidden rounded-lg p-[5px] text-sm"
>
	{#each explorer!.directory.path as path, index}
		<Button
			onclick={() => explorer!.up(index)}
			class="whitespace-nowrap rounded-md px-[5px] py-[3px] hover:bg-black/25"
		>
			{path}
		</Button>
		{#if index < explorer!.directory.path.length - 1}
			<span class="text-secondary">/</span>
		{/if}
	{/each}
</div>
