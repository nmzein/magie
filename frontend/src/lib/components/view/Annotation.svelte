<script lang="ts">
	// Credit: https://medium.com/@doomgoober/understanding-html-canvas-scaling-and-sizing-c04925d9a830
	// Credit: https://stackoverflow.com/questions/59287928/algorithm-to-create-a-polygon-from-points

	import { onMount } from 'svelte';
	import { metadata } from '$lib/stores';

	export let id: string;
	export let level: number;
	export let coordinates: number[][];
	export let colours = { fill: '#e0747099', stroke: '#a12c28' };

	function squaredPolar(coordinate: number[], centre: number[]) {
		return [
			Math.atan2(coordinate[1] - centre[1], coordinate[0] - centre[0]),
			(coordinate[0] - centre[0]) ** 2 + (coordinate[1] - centre[1]) ** 2 // Square of distance
		];
	}

	function polySort(coordinates: number[][]) {
		// Get "centre of mass"
		let centre = [
			coordinates.reduce((sum, p) => sum + p[0], 0) / coordinates.length,
			coordinates.reduce((sum, p) => sum + p[1], 0) / coordinates.length
		];

		// Sort by polar angle and distance, centered at this centre of mass.
		for (let coordinate of coordinates) coordinate.push(...squaredPolar(coordinate, centre));
		coordinates.sort((a, b) => a[2] - b[2] || a[3] - b[3]);
		// Throw away the temporary polar coordinates
		for (let coordinate of coordinates) coordinate.length -= 2;
	}

	function getObjectFitSize(
		contains: boolean /* true = contain, false = cover */,
		containerWidth: number,
		containerHeight: number,
		width: number,
		height: number
	) {
		var doRatio = width / height;
		var cRatio = containerWidth / containerHeight;
		var targetWidth = 0;
		var targetHeight = 0;
		var test = contains ? doRatio > cRatio : doRatio < cRatio;

		if (test) {
			targetWidth = containerWidth;
			targetHeight = targetWidth / doRatio;
		} else {
			targetHeight = containerHeight;
			targetWidth = targetHeight * doRatio;
		}

		return {
			width: targetWidth,
			height: targetHeight,
			x: (containerWidth - targetWidth) / 2,
			y: (containerHeight - targetHeight) / 2
		};
	}

	onMount(() => {
		if ($metadata) {
			const canvas = document.getElementById(id) as HTMLCanvasElement;

			const dimensions = getObjectFitSize(
				false,
				canvas.clientWidth,
				canvas.clientHeight,
				canvas.width,
				canvas.height
			);

			const dpr = window.devicePixelRatio || 1;
			canvas.width = dimensions.width * dpr;
			canvas.height = dimensions.height * dpr;

			const ctx = canvas?.getContext('2d');

			if (!ctx) {
				return;
			}

			// let imageWidth = $metadata[level].width;
			// let imageHeight = $metadata[level].height;

			// console.log(coordinates);
			// const scaleWidth = (x: number) => {
			// 	// console.log(
			// 	// 	'x: ',
			// 	// 	x,
			// 	// 	', canvas width: ',
			// 	// 	canvas.width,
			// 	// 	', metadata width: ',
			// 	// 	imageWidth,
			// 	// 	', scaled: ',
			// 	// 	Math.floor((x * canvas.width) / imageWidth)
			// 	// );
			// 	return Math.floor((x * canvas.width) / imageWidth);
			// };

			// const scaleHeight = (y: number) => {
			// 	// console.log(
			// 	// 	'y: ',
			// 	// 	y,
			// 	// 	', canvas height: ',
			// 	// 	canvas.height,
			// 	// 	', metadata height: ',
			// 	// 	imageHeight,
			// 	// 	', scaled: ',
			// 	// 	Math.floor((y * canvas.height) / imageHeight)
			// 	// );
			// 	return Math.floor((y * canvas.height) / imageHeight);
			// };
			let imageWidth = $metadata[level].cols * 1024;
			let imageHeight = $metadata[level].rows * 1024;

			const scale = (coordinate: number[]) => {
				return [
					Math.floor((coordinate[0] * canvas.width) / imageWidth),
					Math.floor((coordinate[1] * canvas.height) / imageHeight)
				];
			};

			const scale2 = (coordinate: number[]) => {
				return [
					(coordinate[0] * canvas.width) / imageWidth,
					(coordinate[1] * canvas.height) / imageHeight
				];
			};

			// console.log('Scale no floor: ', coordinates.map(scale2));
			// console.log('Scale: ', coordinates.map(scale));

			// Scale coordinates so that they are between 0 and 1.
			coordinates = coordinates.map(scale2);
			// Sort the coordinates
			polySort(coordinates);

			ctx.clearRect(0, 0, canvas.width, canvas.height);

			ctx.beginPath();
			ctx.moveTo(coordinates[0][0], coordinates[0][1]);
			for (let [x, y] of coordinates.slice(1)) ctx.lineTo(x, y);
			ctx.closePath();

			ctx.fillStyle = colours.fill;
			ctx.strokeStyle = colours.stroke;
			ctx.fill();
			ctx.stroke();
		}
	});
</script>

<canvas {id} />

<style>
	canvas {
		position: absolute;
		width: 100%;
		height: 100%;
	}
</style>
