export function boundingclientrect(element: HTMLElement, callback: (value: DOMRect) => void) {
	// Use requestAnimationFrame to continuously poll for changes.
	function loop() {
		callback(element.getBoundingClientRect());
		requestAnimationFrame(loop);
	}

	loop();
}
