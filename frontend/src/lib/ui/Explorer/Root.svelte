<script lang="ts">
	import TopBar from './TopBar.svelte';
	import SidePanel from './SidePanel';
	import MainPanel from './MainPanel.svelte';
	import InnerBar from './InnerBar.svelte';
	import Uploader from './Uploader';
	import type { Bounds } from '$types';
	import { explorer } from '$states';
	import { defined } from '$helpers';

	let { contentSpaceBounds }: { contentSpaceBounds: Bounds } = $props();

	let explorerBounds: Bounds | undefined = $state();
	function resizeobserver(element: HTMLElement) {
		function update() {
			explorerBounds = element.getBoundingClientRect();
		}

		const observer = new ResizeObserver(update);
		observer.observe(element);

		update();

		return {
			destroy() {
				observer.unobserve(element);
			}
		};
	}
	$effect(() => {
		if (!defined(explorerBounds) || explorer.positionSet) return;

		explorer.position = {
			x: 0.5 * (contentSpaceBounds.width - explorerBounds.width),
			y: 0.5 * (contentSpaceBounds.height - explorerBounds.height)
		};

		explorer.positionSet = true;
	});
</script>

<Uploader />

<div
	class="panel flex max-w-[800px] origin-center origin-center flex-col !border-none"
	style="transform: translate({explorer.position.x}px, {explorer.position.y}px);"
	use:resizeobserver
>
	<TopBar
		bind:offsetX={explorer.position.x}
		bind:offsetY={explorer.position.y}
		{contentSpaceBounds}
		{explorerBounds}
	/>
	<div class="flex flex-1 flex-row rounded-[10px]">
		<SidePanel />
		<div class="relative flex flex-[8] flex-col">
			<InnerBar />
			<MainPanel />
		</div>
	</div>
</div>
