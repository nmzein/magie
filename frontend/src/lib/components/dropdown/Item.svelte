<script lang="ts">
	import type { Snippet } from 'svelte';
	import { getDropdownState } from './context.svelte';
	import Button from '$components/Button.svelte';

	let { onclick = () => {}, children }: { onclick?: () => void; children: Snippet } = $props();

	let dState = getDropdownState();
</script>

<Button
	onclick={() => {
		dState.close();
		onclick();
	}}
	class={dState.classes.item}
>
	{@render children()}
</Button>
