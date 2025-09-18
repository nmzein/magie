<script lang="ts">
	import { defined } from '$helpers';
	import type { Snippet } from 'svelte';
	import type { HTMLButtonAttributes } from 'svelte/elements';
	import { tv } from 'tailwind-variants';

	let {
		class: classes,
		component = $bindable(),
		variant = 'plain',
		invisible = false,
		disabled = false,
		children,
		...restProps
	}: {
		class?: string;
		component?: HTMLButtonElement;
		variant?: 'plain' | 'primary' | 'accent';
		invisible?: boolean;
		disabled?: boolean;
		children?: Snippet;
	} & HTMLButtonAttributes = $props();

	const button = tv({
		base: 'cursor-pointer text-sm items-center focus-visible:outline-2 focus-visible:outline-offset-2 rounded-md',
		variants: {
			variant: {
				plain: '',
				primary: 'px-[10px] py-[7.5px] rounded-[5px] hover:bg-primary/10',
				accent: 'px-4 py-2 rounded-full font-medium text-[15px] bg-accent hover:bg-accent-light'
			},
			invisible: {
				true: 'invisible'
			},
			disabled: {
				true: 'cursor-default opacity-50 hover:!bg-transparent'
			}
		},
		compoundVariants: [
			{
				variant: 'primary',
				disabled: true,
				class: 'text-primary/50'
			},
			{
				variant: 'accent',
				disabled: true,
				class: 'bg-accent-dark'
			}
		]
	});
</script>

<button
	bind:this={component}
	{disabled}
	{...restProps}
	class={button({ variant, disabled, invisible, class: classes })}
>
	{#if defined(children)}
		{@render children()}
	{/if}
</button>
