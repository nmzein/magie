export function rgbToHex(rgb: number[]) {
	// Ensure values are within valid range
	rgb[0] = Math.max(0, Math.min(255, rgb[0]));
	rgb[1] = Math.max(0, Math.min(255, rgb[1]));
	rgb[2] = Math.max(0, Math.min(255, rgb[2]));

	// Convert RGB values to hexadecimal
	const rHex = rgb[0].toString(16).padStart(2, '0');
	const gHex = rgb[1].toString(16).padStart(2, '0');
	const bHex = rgb[2].toString(16).padStart(2, '0');

	// Concatenate and return the hex value
	return '#' + rHex + gHex + bHex;
}

export function rgbaToHex(rgb: number[], opacity: number) {
	const rgbHex = rgbToHex(rgb);

	// Convert opacity to hexadecimal
	opacity = Math.max(0, Math.min(1, opacity));
	const opacityHex = Math.round(opacity * 255)
		.toString(16)
		.padStart(2, '0');

	// Concatenate and return the hex value
	return rgbHex + opacityHex;
}

export function rgbToCss(rgb: number[]) {
	return `rgb(${rgb[0]}, ${rgb[1]}, ${rgb[2]})`;
}

export function hexToRgb(hex: string) {
	// Convert hex color to RGB
	const red = parseInt(hex.substring(1, 3), 16);
	const green = parseInt(hex.substring(3, 5), 16);
	const blue = parseInt(hex.substring(5, 7), 16);

	return [red, green, blue];
}
