<script lang="ts">
	import { browser } from '$app/environment';
	import { Stage, Layer, Shape } from 'svelte-konva';
	import type { Point } from '$lib/types';

	export let coordinates: Point[];
	export let colours = { fill: '#e07470', stroke: '#a12c28' };
	export let scale_x = 1;
	export let scale_y = 1;
	export let image_grid_width: number;
	export let image_grid_height: number;
</script>

{#if browser && document.getElementById('image-grid')}
	<div>
		<Stage
			config={{
				width: image_grid_width,
				height: image_grid_height
			}}
		>
			<Layer>
				<Shape
					config={{
						sceneFunc: function (context, shape) {
							context.beginPath();

							context.moveTo(coordinates[0].x * scale_x, coordinates[0].y * scale_y);
							for (let i = 1; i < coordinates.length; i++) {
								context.lineTo(coordinates[i].x * scale_x, coordinates[i].y * scale_y);
							}

							context.closePath();

							// special Konva.js method
							context.fillStrokeShape(shape);
						},
						fill: colours.fill,
						stroke: colours.stroke,
						strokeWidth: 1
					}}
				/>
			</Layer>
		</Stage>
	</div>
{/if}

<style>
	div {
		position: absolute;
	}
</style>
