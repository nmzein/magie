<script lang="ts">
	import Button from '$components/Button.svelte';
	import Icon from '$components/Icon.svelte';
	import { explorer } from '$states';
</script>

<!-- Directory with id of 1 is always set to the global bin. -->
<Button
	onclick={() => explorer!.routeTo([1])}
	class="bg-primary/10 hover:bg-primary/15 flex w-full select-none flex-row items-center gap-[10px] rounded-lg px-[10px] py-[7.5px]"
>
	<Icon name="trash" class="h-6 w-6" />
	Bin
</Button>
