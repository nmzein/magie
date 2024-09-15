<script lang="ts">
	import TopBar from './TopBar.svelte';
	import SidePanel from './SidePanel';
	import MainPanel from './MainPanel.svelte';
	import InnerBar from './InnerBar.svelte';
	import Uploader from './Uploader';
	import type { Bounds } from '$types';

	let { contentSpaceBounds }: { contentSpaceBounds: Bounds } = $props();

	let offsetX = $state(0);
	let offsetY = $state(0);

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
</script>

<Uploader />

<div
	class="panel flex max-w-[800px] origin-center flex-col !border-none"
	style="transform: translate({offsetX}px, {offsetY}px);"
	use:resizeobserver
>
	<TopBar bind:offsetX bind:offsetY {contentSpaceBounds} {explorerBounds} />
	<div class="flex flex-1 flex-row rounded-[10px]">
		<SidePanel />
		<div class="relative flex flex-[8] flex-col">
			<InnerBar />
			<MainPanel />
		</div>
	</div>
</div>
