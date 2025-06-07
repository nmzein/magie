<script lang="ts">
	import * as Dropdown from '$components/dropdown';
	import Separator from '$components/Separator.svelte';
	import type { ContextMenuItem } from './state.svelte.ts';
	import { contextMenu } from '$states';

	const classes = {
		list: `min-w-[150px] flex flex-col mt-[4px] ml-[4px] bg-[#333]/90 rounded-[5px] border border-primary/10 backdrop-blur-[45px] z-10 text-sm`,
		item: 'flex gap-[10px] justify-between items-center mx-1 my-0.5 px-[10px] gap-5 py-[7.5px] rounded-[5px] hover:bg-primary/10'
	};

	function clean(items: ContextMenuItem[]) {
		let cleaned: ContextMenuItem[] = [];
		let lastWasSeparator = false;

		for (const item of items) {
			// Skip hidden items
			if (typeof item === 'object' && item.hidden) continue;

			if (item === 'separator') {
				// Skip if first item or last was a separator
				if (cleaned.length === 0 || lastWasSeparator) continue;

				lastWasSeparator = true;
				cleaned.push(item);
			} else {
				lastWasSeparator = false;
				cleaned.push(item);
			}
		}

		// Remove trailing separator if present
		if (cleaned[cleaned.length - 1] === 'separator') {
			cleaned.pop();
		}

		return cleaned;
	}
</script>

<svelte:window
	onkeydown={(e) => {
		if (e.key === 'Escape' || e.key === 'Enter') {
			contextMenu.close();
		}
	}}
/>

<!-- TODO: Focus keyboard on first item. -->
<!-- TODO: Intelligently display based on viewport size. -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="pointer-events-auto absolute z-[1000]"
	style="transform: translate({contextMenu.position.x}px, {contextMenu.position.y}px)"
	oncontextmenu={(e) => e.preventDefault()}
>
	<Dropdown.Root {classes} bind:show={contextMenu.show}>
		<Dropdown.List>
			{#each clean(contextMenu.items) as item}
				{#if item === 'separator'}
					<div class="flex w-full justify-center">
						<Separator class="my-1" />
					</div>
				{:else}
					<Dropdown.Item onclick={item.action} disabled={item.disabled} hidden={item.hidden}>
						{item.name}
						<span class="text-secondary">
							{item.shortcut}
						</span>
					</Dropdown.Item>
				{/if}
			{/each}
		</Dropdown.List>
	</Dropdown.Root>
</div>
