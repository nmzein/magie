<script lang="ts">
	import '../global.scss';
	import View from '$view/View.svelte';
	import ControlPanel from '$ui/ControlPanel.svelte';
	import { ConnectWebSocket } from '$api';

	$effect(() => {
		ConnectWebSocket();
	});
</script>

<View />
<ControlPanel />
