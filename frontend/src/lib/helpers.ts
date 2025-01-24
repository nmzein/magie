import {
	type ClientRequest,
	type ServerResponse,
	requestHandler,
	responseHandler
} from './api.helpers';

export async function request<
	M extends ClientRequest['method'],
	D extends ServerResponse['data_type']
>(request: Extract<ClientRequest, { method: M }>): Promise<D | undefined> {
	const { url, method, headers, body } = requestHandler[request.method](request);

	return await attempt(fetch(url, { method, headers, body })).then(([error, response]) => {
		if (error) {
			console.error(`Fetch Error [${url.pathname}${url.search}]:`, error);
			return;
		}

		if (!response.ok) {
			console.error(
				`Response Error [${url.pathname}${url.search}]:`,
				response.status,
				response.statusText
			);
			return;
		}

		const contentType = response.headers.get('Content-Type') as ServerResponse['content_type'];
		const handler = responseHandler[contentType];

		if (!contentType || !handler) {
			console.error(
				`Content-Type Error [${url.pathname}${url.search}]: No or Invalid Content-Type in Response: ${contentType}`
			);
			return;
		}

		return attempt(handler(response) as Promise<D>).then(([error, result]) => {
			if (error) {
				console.error(`Parse Error [${url.pathname}${url.search}]:`, error);
				return;
			}

			return result;
		});
	});
}

export function appendPx<T extends Record<string, number>>(values: T): T {
	const result = {} as T;
	Object.entries(values).forEach(([key, value]) => {
		result[key as keyof T] = `${value}px` as any;
	});
	return result;
}

export function attempt<T>(fn: Promise<T>): Promise<[Error | null, T]> {
	return fn
		.then((data) => [null, data] as [null, T])
		.catch((error) => [
			error instanceof Error ? error : new Error('Unknown error'),
			null as unknown as T
		]);
}

export function defined<T>(value: T | undefined | null): value is T {
	return value !== undefined && value !== null;
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

export function applyDefaults<T extends Record<string, any>>(
	overrides: Partial<T> = {},
	defaults: T
): DeepRequired<T> {
	return {
		...defaults,
		...Object.keys(defaults).reduce((acc, key) => {
			const defaultValue = defaults[key as keyof T];
			const overrideValue = overrides[key as keyof T];

			// Check if the default value is an object (excluding arrays)
			if (
				typeof defaultValue === 'object' &&
				defaultValue !== null &&
				!Array.isArray(defaultValue)
			) {
				// Recursively apply defaults to nested objects
				(acc as any)[key] = applyDefaults(overrideValue, defaultValue);
			} else {
				// Use the override if provided; otherwise, use the default value
				(acc as any)[key] = overrideValue !== undefined ? overrideValue : defaultValue;
			}

			return acc;
		}, {} as T)
	} as DeepRequired<T>;
}

export type DeepRequired<T> = T extends object
	? T extends Array<infer U>
		? Array<DeepRequired<U>>
		: { [K in keyof T]-?: DeepRequired<T[K]> }
	: T;
