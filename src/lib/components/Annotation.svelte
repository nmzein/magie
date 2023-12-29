<script lang="ts">
	import { browser } from '$app/environment';
	import { Stage, Layer, Shape } from 'svelte-konva';
	// export let coordinates: [number, number][] = [];
	// export let parentElement: HTMLElement; // Reference to the parent element
	// export let offsetX = 0;
	// export let offsetY = 0;

	// function getRelativeCoordinates(coords: [number, number][]) {
	// 	if (!parentElement) return '';

	// 	// Get the position of the parent element
	// 	const parentRect = parentElement.getBoundingClientRect();

	// 	// Adjust coordinates relative to the parent element
	// 	return coords
	// 		.map((coord) => `${coord[0] - parentRect.left},${coord[1] - parentRect.top}`)
	// 		.join(' ');
	// }
</script>

<!-- <div
	class="polygon"
	style="clip-path: {`polygon(${points
		.map((point) => `${point[0] + offsetX}px ${point[1] + offsetY}px`)
		.join(', ')})`}"
/> -->

<!-- bind:this={polygon} -->

<Stage
	config={browser
		? { width: window.innerWidth, height: window.innerHeight }
		: { width: 0, height: 0 }}
>
	<Layer>
		<Shape
			config={{
				sceneFunc: function (context, shape) {
					context.beginPath();
					context.moveTo(20, 50);
					context.lineTo(220, 80);
					context.quadraticCurveTo(150, 100, 260, 170);
					context.closePath();

					// special Konva.js method
					context.fillStrokeShape(shape);
				},
				fill: '#00D2FF',
				stroke: 'black',
				strokeWidth: 4
			}}
		/>
	</Layer>
</Stage>

<!-- 
<svg>
	<polygon points={getRelativeCoordinates(coordinates)} />-->
<!-- <polygon points={coordinates.map((coord) => `${coord[0]},${coord[1]}`).join(' ')} /> -->
<!-- </svg>  -->

<style>
	/* svg {
		width: 100%;
		height: 100%;
	} */
	/* .polygon {
		position: absolute;
		border: 10px solid red;
	} */
</style>
