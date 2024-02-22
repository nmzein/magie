<script lang="ts">
	// Credit: https://medium.com/@doomgoober/understanding-html-canvas-scaling-and-sizing-c04925d9a830
	// Credit: https://stackoverflow.com/questions/59287928/algorithm-to-create-a-polygon-from-points
	let {
		id,
		coordinates,
		colours = { fill: '#e0747099', stroke: '#a12c28' },
		imageWidth,
		imageHeight
	} = $props<{
		id: string;
		coordinates: number[][];
		colours: { fill: string; stroke: string };
		imageWidth: number;
		imageHeight: number;
	}>();

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

	$effect(() => {
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
		if (!ctx) return;

		let scaleWidth = canvas.width / imageWidth;
		let scaleHeight = canvas.height / imageHeight;

		const scale = (coordinate: number[]) => [
			coordinate[0] * scaleWidth,
			coordinate[1] * scaleHeight
		];

		// Scale coordinates so that they are between 0 and 1.
		coordinates = coordinates.map(scale);
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
