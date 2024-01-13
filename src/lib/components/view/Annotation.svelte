<script lang="ts">
	// Credit: https://medium.com/@doomgoober/understanding-html-canvas-scaling-and-sizing-c04925d9a830
	import type { Point } from '$lib/types';
	import { onMount } from 'svelte';

	export let id: string;
	export let coordinates: Point[];
	export let colours = { fill: '#e07470', stroke: '#a12c28' };

	// only values from 0 to 1 are allowed
	// TODO: fix to use metadata when the zarr conversion is fixed
	// divide all coordinates by 2048
	coordinates = coordinates.map((coordinate) => {
		return {
			x: coordinate.x / 2048,
			y: coordinate.y / 2048
		};
	});

	onMount(() => {
		render();
	});

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

	function render() {
		// TODO: pass these values instead of having every component calculate
		// them for itself
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

		if (ctx) {
			function scale_width(coordinate: number) {
				return Math.floor(coordinate * canvas.width);
			}

			function scale_height(coordinate: number) {
				return Math.floor(coordinate * canvas.height);
			}

			ctx.beginPath();
			ctx.moveTo(scale_width(coordinates[0].x), scale_height(coordinates[0].y));
			for (let i = 1; i < coordinates.length; i++) {
				ctx.lineTo(scale_width(coordinates[i].x), scale_height(coordinates[i].y));
			}
			ctx.closePath();

			ctx.fillStyle = colours.fill;
			ctx.fill();
			ctx.strokeStyle = colours.stroke;
			ctx.stroke();
		}
	}
</script>

<canvas {id} />

<style>
	canvas {
		position: absolute;
		width: 100%;
		height: 100%;
	}
</style>
