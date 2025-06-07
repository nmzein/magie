<script lang="ts">
	import NavigationButtons from './NavigationButtons.svelte';
	import AddressBar from './AddressBar.svelte';
	import Search from './Search.svelte';
	import { context } from '$components/window/context.svelte.ts';

	const windowContext = context.get();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="border-b-secondary/20 bg-primary/15 flex flex-row items-center gap-[10px] rounded-t-[inherit] border-b p-[10px] active:cursor-grabbing"
	onmousedown={windowContext.drag}
>
	<NavigationButtons />
	<AddressBar />
	<Search />
</div>
