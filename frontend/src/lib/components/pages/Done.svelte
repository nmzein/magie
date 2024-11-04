<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getPagesState } from './context.svelte.js';
	import Button from '$components/Button.svelte';

	let { enabled, done, children }: { enabled: boolean; done: () => void; children: Snippet } =
		$props();

	let pState = getPagesState();
</script>

<Button
	variant="primary"
	class="text-sm"
	hidden={!pState.lastPage}
	disabled={!enabled}
	onclick={done}
>
	{@render children()}
</Button>
