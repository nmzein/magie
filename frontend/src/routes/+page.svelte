<script lang="ts">
	import '../global.scss';
	import View from '$view/View.svelte';
	import ControlPanel from '$ui/ControlPanel.svelte';
	import { generators, registry } from '$states';
	import { ConnectWebSocket } from '$api';

	$effect(() => {
		ConnectWebSocket();
		generators.init();
		registry.init();
	});
</script>

<View />
<ControlPanel />
