<script lang="ts">
	import * as Dropdown from '$components/dropdown';
	import { contextMenu } from '$states';

	const classes = {
		list: `min-w-[150px] flex flex-col mt-[4px] ml-[4px] bg-tertiary/90 rounded-[5px] border border-primary/10 backdrop-blur-[45px] z-10 text-sm`,
		item: 'flex flex-row gap-[10px] items-center m-[2px] px-[10px] py-[7.5px] rounded-[5px] hover:bg-primary/10'
	};
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
	class="absolute z-[1000]"
	style={`transform: translate(${contextMenu.position.x}px, ${contextMenu.position.y}px)`}
	oncontextmenu={(e) => e.preventDefault()}
>
	<Dropdown.Root {classes} bind:show={contextMenu.show}>
		<Dropdown.List>
			{#each contextMenu.items as item}
				<Dropdown.Item onclick={item.action}>
					{item.name}
				</Dropdown.Item>
			{/each}
		</Dropdown.List>
	</Dropdown.Root>
</div>
