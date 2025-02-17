<script lang="ts">
	import Separator from '$components/Separator.svelte';
	import PinnedSection from './PinnedSection.svelte';
	import StoresSection from './StoresSection.svelte';
	import Bin from './Bin.svelte';
</script>

<div
	class="border-secondary/20 bg-primary/15 flex w-1/4 flex-col gap-3 rounded-bl-[inherit] border-r px-5 py-4"
>
	<PinnedSection />
	<Separator />
	<StoresSection />
	<Bin />
</div>
