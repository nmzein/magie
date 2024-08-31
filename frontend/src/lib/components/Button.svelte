<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';

	let {
		variant = 'default',
		hidden,
		children,
		...restProps
	}: {
		variant?: 'default' | 'primary';
		hidden: boolean;
		children: Snippet;
	} & HTMLButtonAttributes = $props();
</script>

<button class={variant} class:hidden {...restProps}>{@render children()}</button>

<style lang="scss">
	.hidden {
		visibility: hidden;
	}

	.default {
		align-items: center;
		padding: 7.5px 10px;
		border-radius: 5px;

		&:hover {
			background-color: rgba(255, 255, 255, 0.1);
		}
	}

	.primary {
		align-items: center;
		padding: 10px 20px;
		font-size: 15px;
		border-radius: 9999px;
		background-color: rgb(12, 98, 197);

		&:hover {
			background-color: rgba(255, 255, 255, 0.1);
		}
	}
</style>
