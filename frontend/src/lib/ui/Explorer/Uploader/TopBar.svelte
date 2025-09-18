<script lang="ts">
	import Icon from '$icon';
	import Button from '$components/Button.svelte';
	import { context } from '../context.svelte.ts';
	import { context as wContext } from '$components/window/context.svelte.ts';

	const explorer = context.get();
	const windowContext = wContext.get();
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="bg-primary/10 flex items-center justify-end rounded-t-[9px] p-2 select-none active:cursor-grabbing"
	onmousedown={windowContext.drag}
>
	<Button
		onclick={() => explorer.uploader.close()}
		class="bg-primary/15 hover:bg-primary/20 rounded-full p-1 transition-colors"
	>
		<Icon name="close" class="h-4 w-4" />
	</Button>
</div>
