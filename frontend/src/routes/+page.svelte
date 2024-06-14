<script lang="ts">
	import '../global.scss';
	import View from '$view/View.svelte';
	import ControlPanel from '$control/ControlPanel.svelte';
	import { GetGenerators, GetRegistry } from '$api';
	import { ConnectWebSocket } from '$stores';

	$effect(() => {
		ConnectWebSocket();
		GetRegistry();
		GetGenerators();
	});
</script>

<View />
<ControlPanel />
