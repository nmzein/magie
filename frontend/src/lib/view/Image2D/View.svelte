<script lang="ts">
	import { WEBSOCKET_URL } from '$constants';
	import type { Image2DView } from './types';
	import { onMount } from 'svelte';

	let { view = $bindable() }: { view: Image2DView } = $props();

	let canvas: HTMLCanvasElement | undefined = $state();
	let worker: Worker | undefined = $state();
	let sharedInts: Int32Array | undefined;

	onMount(() => {
		if (!canvas || !window.innerWidth || !window.innerHeight || !view.state) return;

		// Prepare shared memory:
		// [0] viewportWidth
		// [1] viewportHeight
		// [2] offsetX
		// [3] offsetY
		// [4] scale * 1e6
		// [5] dirty flag
		const buf = new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT * 6);
		sharedInts = new Int32Array(buf);

		worker = new Worker(new URL('./worker.ts', import.meta.url), { type: 'module' });

		const offscreen = canvas.transferControlToOffscreen();
		const viewportWidth = window.innerWidth * 2;
		const viewportHeight = window.innerHeight * 2;

		// init message (includes shared buffer)
		worker.postMessage(
			{
				type: 'init',
				data: {
					canvas: offscreen,
					width: viewportWidth,
					height: viewportHeight,
					wsUrl: WEBSOCKET_URL,
					storeId: view.state.storeId,
					id: view.state.id,
					layers: JSON.stringify(view.state.layers),
					sharedBuf: buf
				}
			},
			[offscreen]
		);

		worker.onmessage = (e) => {
			const { type } = e.data;
			switch (type) {
				case 'connected':
					markDirty();
					break;
			}
		};

		return () => {
			worker?.postMessage({ type: 'disconnect' });
			worker?.terminate();
		};
	});

	function markDirty() {
		if (!sharedInts) return;
		const { scale, offsetX, offsetY } = view.state.transformer;

		sharedInts[0] = window.innerWidth * 2;
		sharedInts[1] = window.innerHeight * 2;
		sharedInts[2] = offsetX;
		sharedInts[3] = offsetY;
		sharedInts[4] = Math.floor(scale * 1e6);
		Atomics.store(sharedInts, 5, 1);
	}

	function onmousedown(e: MouseEvent) {
		e.preventDefault();
		view.state.transformer.panStart(e.clientX, e.clientY);
	}

	function ontouchstart(te: TouchEvent) {
		const e = te.touches[0];
		view.state.transformer.panStart(e.clientX, e.clientY);
	}

	function onmousemove(e: MouseEvent) {
		if (!view.state.transformer.isDragging) return;
		e.preventDefault();
		view.state.transformer.pan(e.clientX, e.clientY);
		markDirty();
	}

	function ontouchmove(te: TouchEvent) {
		if (!view.state.transformer.isDragging) return;
		const e = te.touches[0];
		view.state.transformer.pan(e.clientX, e.clientY);
		markDirty();
	}

	function onmouseup() {
		view.state.transformer.panStop();
		markDirty();
	}

	function ontouchend() {
		view.state.transformer.panStop();
		markDirty();
	}

	function onwheel(e: WheelEvent) {
		view.state.transformer.zoom(e.deltaY, e.clientX, e.clientY);
		markDirty();
	}

	function onresize() {
		markDirty();
	}
</script>

<svelte:window {onresize} {onmousemove} {ontouchmove} {onmouseup} {ontouchend} {onwheel} />

<canvas
	{onmousedown}
	{ontouchstart}
	style="cursor: {view.state.transformer.isDragging ? 'grab' : 'crosshair'};"
	bind:this={canvas}
	class="absolute h-full w-full"
></canvas>
