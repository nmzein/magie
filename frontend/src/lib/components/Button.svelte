<script lang="ts">
	import { defined } from '$helpers';
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';
	import { twMerge } from 'tailwind-merge';

	let {
		class: className,
		component = $bindable(),
		variant,
		invisible = false,
		disabled = false,
		children,
		...restProps
	}: {
		class?: string;
		component?: HTMLButtonElement;
		variant?: 'default' | 'primary';
		invisible?: boolean;
		disabled?: boolean;
		children?: Snippet;
	} & HTMLButtonAttributes = $props();

	let variants = {
		default: 'items-center px-[10px] py-[7.5px] rounded-[5px] hover:bg-primary/10',
		primary: 'items-center px-4 py-[10px] rounded-full text-[15px] bg-accent hover:bg-accent-light'
	};
</script>

<button
	bind:this={component}
	class={twMerge(
		`cursor-pointer text-sm ${variant ? variants[variant] : ''} ${className} ${invisible ? 'invisible' : ''} ${disabled ? 'cursor-default' : ''}`
	)}
	{disabled}
	{...restProps}
>
	{#if defined(children)}
		{@render children()}
	{/if}
</button>
