<script lang="ts">
	import { context } from './context.svelte.ts';

	const explorer = context.get();
</script>

<input
	type="text"
	placeholder="Search"
	class="bg-primary/15 h-8.75 shrink-0 rounded-lg px-2.5 py-1.25 text-sm focus:outline-2 focus:outline-offset-2"
	bind:value={explorer.searchQuery}
/>
