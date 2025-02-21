<script lang="ts">
	import { context } from './context.svelte.ts';

	const explorer = context.get();
</script>

<input
	type="text"
	placeholder="Search"
	class="bg-primary/15 h-[35px] shrink-0 rounded-lg px-[10px] py-[5px] text-sm focus:outline-2 focus:outline-offset-2"
	bind:value={explorer.searchQuery}
/>
