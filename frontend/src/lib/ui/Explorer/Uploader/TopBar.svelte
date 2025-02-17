<script lang="ts">
	import Icon from '$icon';
	import Button from '$components/Button.svelte';
	import { context } from '../context.svelte.ts';

	const explorer = context.get();
</script>

<div class="bg-primary/10 flex items-center justify-end rounded-t-[9px] p-2 select-none">
	<Button
		onclick={() => explorer.uploader.close()}
		class="bg-primary/15 hover:bg-primary/20 rounded-full p-1 transition-colors"
	>
		<Icon name="close" class="h-4 w-4" />
	</Button>
</div>
