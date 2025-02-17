<script lang="ts">
	import Button from '$components/Button.svelte';
	import Icon from '$components/Icon.svelte';
	import { context, BIN_ID } from '../context.svelte.ts';

	const explorer = context.get();
</script>

<Button
	onclick={() => explorer.goto(BIN_ID)}
	class="bg-primary/10 hover:bg-primary/15 flex w-full flex-row items-center gap-[10px] rounded-lg px-[10px] py-[7.5px] select-none"
>
	<Icon name="trash" class="h-6 w-6" />
	Bin
</Button>
