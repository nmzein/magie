<script lang="ts">
	import { explorer } from '$states';
	import Icon from '$icon';
</script>

<div class="flex flex-row">
	<Icon name="back" class="h-10 w-10" onclick={() => explorer.backward()} />
	<Icon name="forward" class="h-10 w-10" onclick={() => explorer.forward()} />
	<Icon name="up" class="h-10 w-10" onclick={() => explorer.up()} />
</div>
