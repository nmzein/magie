<script lang="ts">
	import { twMerge } from 'tailwind-merge';
	import type { ClassValue } from 'svelte/elements';

	let { class: className = '' }: { class?: ClassValue } = $props();
</script>

<div class={twMerge('bg-secondary/20 my-2 h-[1px] w-full', className)}></div>
