<script lang="ts">
	import { defined } from '$helpers';
	import type { Image2DView } from './types.ts';

	let { view = $bindable() }: { view: Image2DView } = $props();

	let canvas: HTMLCanvasElement | undefined = $state();
	let ctx = $derived(canvas?.getContext('2d'));

	let x = $state(0);
	let y = $state(0);

	function onmousedown(e: MouseEvent) {
		e.preventDefault();
		view.state.transformer.panStart(e.clientX, e.clientY);
	}

	function ontouchstart(te: TouchEvent) {
		const e = te.touches[0];
		view.state.transformer.panStart(e.clientX, e.clientY);
	}

	function onmousemove(e: MouseEvent) {
		e.preventDefault();
		if (!canvas) return;

		// Logic for calculating the coordinates of the mouse pointer.
		// if (!view.state.transformer.isDragging) {
		// 	const xTemp = Math.floor(
		// 		(e.clientX - view.state.transformer.offsetX) * (view.state.width / canvas.width)
		// 	);
		// 	const yTemp = Math.floor(
		// 		(e.clientY - view.state.transformer.offsetY) * (view.state.height / canvas.height)
		// 	);

		// 	if (Number.isFinite(xTemp) && !isNaN(xTemp)) x = xTemp;
		// 	if (Number.isFinite(yTemp) && !isNaN(yTemp)) y = yTemp;

		// 	return;
		// }

		view.state.transformer.pan(e.clientX, e.clientY);
	}

	function ontouchmove(te: TouchEvent) {
		const e = te.touches[0];
		view.state.transformer.pan(e.clientX, e.clientY);
	}

	$effect(() => {
		for (let x = 0; x < view.state.layers[0].rows; x++) {
			for (let y = 0; y < view.state.layers[0].cols; y++) {
				view.state.getTile(0, x, y);
			}
		}
	});

	$effect(() => {
		if (!canvas || !ctx || !view.state.tiles) return;
		render();
	});

	function render() {
		if (!canvas || !ctx) return;

		const vw = window.innerWidth;
		const vh = window.innerHeight;

		// Set backing resolution (actual pixels)
		canvas.width = vw * 2;
		canvas.height = vh * 2;

		// Optional: match display size to viewport (so it doesn’t look stretched)
		canvas.style.width = `${vw}px`;
		canvas.style.height = `${vh}px`;

		const TS = 1024;
		const CTS = TS * view.state.transformer.scale;

		for (let c = 0; c < view.state.layers[0].cols; c++) {
			for (let r = 0; r < view.state.layers[0].rows; r++) {
				if (!view.state.tiles[0][c][r]) continue;
				ctx.drawImage(
					view.state.tiles[0][c][r],
					0,
					0,
					TS,
					TS,
					view.state.transformer.offsetX + r * CTS,
					view.state.transformer.offsetY + c * CTS,
					CTS,
					CTS
				);
			}
		}
	}
</script>

<svelte:window
	onresize={render}
	{onmousemove}
	{ontouchmove}
	onmouseup={() => view.state.transformer.panStop()}
	ontouchend={() => view.state.transformer.panStop()}
	onwheel={(e) => view.state.transformer.zoom(e.deltaY, e.clientX, e.clientY)}
/>
<canvas
	{onmousedown}
	{ontouchstart}
	style="cursor: {view.state.transformer.isDragging ? 'grab' : 'crosshair'};"
	bind:this={canvas}
	class="absolute h-full w-full"
>
</canvas>
