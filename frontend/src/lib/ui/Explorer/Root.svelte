<script lang="ts">
	import TopBar from './TopBar.svelte';
	import SidePanel from './SidePanel';
	import MainPanel from './MainPanel.svelte';
	import InnerBar from './InnerBar.svelte';
	import Uploader from './Uploader';
	import type { Bounds } from '$types';
	import { defined } from '$helpers';
	import { untrack } from 'svelte';
	import { ResizeObserver } from '$actions';
	import { context, Explorer } from './context.svelte.ts';

	const explorer = context.set(new Explorer());

	let { contentSpaceBounds }: { contentSpaceBounds: Bounds } = $props();

	let explorerBounds: Bounds | undefined = $state();

	$effect(() => {
		if (!defined(explorerBounds)) return;

		contentSpaceBounds.width;
		contentSpaceBounds.height;

		untrack(() => {
			explorer.position = {
				x: 0.5 * (contentSpaceBounds.width - explorerBounds!.width),
				y: 0.3 * (contentSpaceBounds.height - explorerBounds!.height)
			};
		});
	});
</script>

<div
	class="panel flex max-w-[800px] min-w-48 origin-center flex-col !border-none"
	style="transform: translate({explorer.position.x}px, {explorer.position.y}px);"
	use:ResizeObserver={(v) => (explorerBounds = v)}
	onwheel={(e) => e.stopPropagation()}
>
	<Uploader />

	<TopBar
		bind:offsetX={explorer.position.x}
		bind:offsetY={explorer.position.y}
		{contentSpaceBounds}
		{explorerBounds}
	/>
	<div class="@container flex flex-1 flex-row rounded-[10px]">
		<SidePanel />
		<div class="relative flex w-full flex-col">
			<InnerBar />
			<MainPanel />
		</div>
	</div>
</div>
