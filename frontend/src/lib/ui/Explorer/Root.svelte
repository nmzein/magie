<script lang="ts">
	import TopBar from './TopBar.svelte';
	import SidePanel from './SidePanel';
	import MainPanel from './MainPanel.svelte';
	import InnerBar from './InnerBar.svelte';
	import Uploader from './Uploader';
</script>

<Uploader />

<div class="panel flex w-[800px] flex-col !border-none">
	<TopBar />
	<div class="flex flex-1 flex-row rounded-[10px]">
		<SidePanel />
		<div class="relative flex flex-[8] flex-col">
			<InnerBar />
			<MainPanel />
		</div>
	</div>
</div>
