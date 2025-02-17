<script lang="ts">
	import Icon from '$icon';
	import { context } from './context.svelte.ts';

	const explorer = context.get();
</script>

<div class="flex flex-row select-none">
	<Icon name="back" class="h-10 w-10" onclick={() => explorer.undo()} />
	<Icon name="forward" class="h-10 w-10" onclick={() => explorer.redo()} />
	<Icon name="up" class="h-10 w-10" onclick={() => explorer.up()} />
</div>
