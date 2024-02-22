<script lang="ts">
	import '../global.scss';
	import View from '$view/View.svelte';
	import Controls from '$control/Controls.svelte';
	import { GetGenerators, GetStores } from '$api';
	import { ConnectWebSocket } from '$stores';

	$effect(() => {
		ConnectWebSocket();
		GetStores();
		GetGenerators();
	});
</script>

<View />
<Controls />
