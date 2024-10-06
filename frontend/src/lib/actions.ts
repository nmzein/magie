export function boundingclientrect(element: HTMLElement, callback: (value: DOMRect) => void) {
	// Use requestAnimationFrame to continuously poll for changes.
	function loop() {
		callback(element.getBoundingClientRect());
		requestAnimationFrame(loop);
	}

	loop();
}

export function resizeobserver(element: HTMLElement, callback: (bounds: DOMRect) => void) {
	function update() {
		callback(element.getBoundingClientRect());
	}

	const observer = new ResizeObserver(update);
	observer.observe(element);

	update();

	return {
		destroy() {
			observer.disconnect();
		}
	};
}