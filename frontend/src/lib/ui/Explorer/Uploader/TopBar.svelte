<script lang="ts">
	import { explorer } from '$states';
	import Icon from '$icon';
	import Button from '$components/Button.svelte';
</script>

<div
	class="bg-primary/15 flex select-none items-center justify-center rounded-t-[9px] p-[15px] text-lg font-semibold"
>
	Uploader
	<Button onclick={() => (explorer.showUploader = false)} class="absolute right-3 rounded-full">
		<Icon name="close" class="h-8 w-8" />
	</Button>
</div>
