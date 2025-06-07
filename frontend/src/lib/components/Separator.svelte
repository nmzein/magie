<script lang="ts">
	import { twMerge } from 'tailwind-merge';

	let { class: className = '' }: { class?: string } = $props();
</script>

<div class={twMerge('bg-secondary/20 my-2 h-[1px] w-full', className)}></div>
