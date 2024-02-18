<script lang="ts">
	import '../global.scss';
	import View from '$lib/components/view/View.svelte';
	import Controls from '$lib/components/control/Controls.svelte';
	import { onMount } from 'svelte';
	import { ConnectWebSocket, GetGenerators, GetStores } from '$lib/api';

	onMount(() => {
		ConnectWebSocket();
		GetStores();
		GetGenerators();
	});
</script>

<View />
<Controls />
