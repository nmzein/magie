export function appendPx<T extends Record<string, number>>(values: T): T {
	const result = {} as T;
	Object.entries(values).forEach(([key, value]) => {
		result[key as keyof T] = `${value}px` as any;
	});
	return result;
}

export function defined<T>(value: T | undefined): value is T {
	return value !== undefined;
}

export function truncateNumber(num: number, digits: number = 2) {
	// Convert the number to a string with 2 decimal places
	const number = num.toString();

	if (number.includes('.')) {
		const [integer, decimal] = number.split('.');

		if (integer.length >= digits) {
			// For when scale == 101.2
			return integer;
		} else if (integer.length + decimal.length >= digits) {
			// Most cases.
			return integer + '.' + decimal.slice(0, digits - integer.length);
		} else {
			// For when scale is number like 0.1, 6.0, etc.
			return integer + '.' + decimal + '0'.repeat(digits - integer.length - decimal.length);
		}
	} else {
		const integer = number;

		if (integer.length >= digits) {
			return integer;
		} else {
			// For when scale is integer like 1.
			return integer + '.' + '0'.repeat(digits - integer.length);
		}
	}
}
