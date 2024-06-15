<!-- 
	The root of the file explorer.
-->
<script lang="ts">
	import TopBar from './TopBar.svelte';
	import SidePanel from './SidePanel.svelte';
	import MainPanel from './MainPanel.svelte';
	import InnerBar from './InnerBar.svelte';
</script>

<div id="outer" class="flex-column panel">
	<TopBar />
	<div id="inner" class="flex-row">
		<SidePanel />
		<div class="flex-column" id="inner-inner">
			<InnerBar />
			<MainPanel />
		</div>
	</div>
</div>

<style lang="scss">
	#outer {
		width: 800px;
	}

	#inner {
		flex: 1;
	}

	#inner-inner {
		flex: 5;
		position: relative;
	}
</style>
