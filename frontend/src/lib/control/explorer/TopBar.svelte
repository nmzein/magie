<!-- 
    Top bar of the explorer, contains the navigation buttons, address bar, and search bar.
-->
<script lang="ts">
	import NavigationButtons from './NavigationButtons.svelte';
	import AddressBar from './AddressBar.svelte';
	import Search from './Search.svelte';
</script>

<div class="flex-row light-layer">
	<NavigationButtons />
	<AddressBar />
	<Search />
</div>

<style lang="scss">
	div {
		gap: 10px;
		padding: 10px;
		align-items: center;
		border-radius: 10px 10px 0px 0px;
		border-bottom: 1px solid rgba(168, 168, 168, 0.2);
	}
</style>
