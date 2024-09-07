<script lang="ts">
	import NavigationButtons from './NavigationButtons.svelte';
	import AddressBar from './AddressBar.svelte';
	import Search from './Search.svelte';
</script>

<div
	class="border-b-secondary/20 bg-primary/15 flex flex-row items-center gap-[10px] rounded-t-[inherit] border-b p-[10px]"
>
	<NavigationButtons />
	<AddressBar />
	<Search />
</div>
