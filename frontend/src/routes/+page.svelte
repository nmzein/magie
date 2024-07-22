<script lang="ts">
	import '../global.scss';
	import View from '$view/View.svelte';
	import ControlPanel from '$ui/ControlPanel.svelte';
	import { generators, explorer } from '$states';
	import { ConnectWebSocket } from '$api';

	$effect(() => {
		ConnectWebSocket();
		generators.init();
	});
</script>

<View />
<ControlPanel />
