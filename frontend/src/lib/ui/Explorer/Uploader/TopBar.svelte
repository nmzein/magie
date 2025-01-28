<script lang="ts">
	import { explorer } from '$states';
	import Icon from '$icon';
	import Button from '$components/Button.svelte';
</script>

<div class="bg-primary/10 flex select-none items-center justify-end rounded-t-[9px] p-2">
	<Button
		onclick={() => explorer!.uploader.close()}
		class="bg-primary/15 hover:bg-primary/20 rounded-full p-1 transition-colors"
	>
		<Icon name="close" class="h-4 w-4" />
	</Button>
</div>
