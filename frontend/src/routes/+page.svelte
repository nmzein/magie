<script lang="ts">
	import '../global.scss';
	import View from '$view/View.svelte';
	import ControlPanel from '$ui/ControlPanel.svelte';
	import { GetGenerators, GetRegistry } from '$api';
	import { ConnectWebSocket } from '$states';

	$effect(() => {
		ConnectWebSocket();
		GetRegistry();
		GetGenerators();
	});
</script>

<View />
<ControlPanel />
