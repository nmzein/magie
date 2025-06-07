<script lang="ts">
	import TopBar from './TopBar.svelte';
	import SidePanel from './SidePanel';
	import MainPanel from './MainPanel.svelte';
	import InnerBar from './InnerBar.svelte';
	import Uploader from './Uploader';
	import { context, Explorer } from './context.svelte.ts';

	context.set(new Explorer());
</script>

<div
	class="panel outline-tertiary flex w-[800px] min-w-48 origin-center flex-col !border-none outline"
	onwheel={(e) => e.stopPropagation()}
>
	<Uploader />

	<TopBar />
	<div class="@container flex flex-1 flex-row rounded-[10px]">
		<SidePanel />
		<div class="relative flex w-full flex-col">
			<InnerBar />
			<MainPanel />
		</div>
	</div>
</div>
