type Req = {
	method: 'GET' | 'POST' | 'PATCH' | 'DELETE';
	url: string;
	query?: Record<string, any>;
	body?: Record<string, any>;
	type?: 'json' | 'form';
};

class FetchHandler {
	#url(req: Req): URL {
		const url = new URL(req.url);
		url.search = new URLSearchParams(req.query).toString();
		return url;
	}

	#content(req: Req) {
		if (!req.body) return;
		if (!req.type) req.type = 'json';

		switch (req.type) {
			case 'json': {
				const headers = { 'Content-Type': 'application/json' };
				const body = JSON.stringify(req.body);
				return { headers, body };
			}
			case 'form': {
				const body = new FormData();
				for (const [key, value] of Object.entries(req.body)) {
					if (defined(value)) body.append(key, value);
				}
				return { body };
			}
		}
	}

	async #response<T>(res: Response) {
		switch (res.headers.get('Content-Type')) {
			case 'application/json': {
				return await attempt<T>(res.json());
			}
			case 'image/jpeg': {
				return (await attempt(res.blob())) as [Error | null, T];
			}
			default: {
				return [null, null];
			}
		}
	}

	async #request<T>(req: Req): Promise<T | null> {
		const url = this.#url(req);
		const content = this.#content(req);

		return attempt(
			fetch(url, { method: req.method, headers: content?.headers, body: content?.body })
		).then(([error, response]) => {
			if (error) {
				console.error(`Fetch Error [${url.pathname}${url.search}]:`, error);
				return null;
			}

			if (!response.ok) {
				console.error(
					`Response Error [${url.pathname}${url.search}]:`,
					response.status,
					response.statusText
				);
				return null;
			}

			return this.#response<T>(response).then(([error, result]) => {
				if (error) {
					console.error(
						`Content-Type Error [${url.pathname}${url.search}]: No or Invalid Content-Type in Response: ${error}`
					);
					return null;
				}

				return result;
			});
		});
	}

	get<T>({ url, query }: Omit<Req, 'method' | 'body' | 'content_type'>): Promise<T | null> {
		return this.#request({ method: 'GET', url, query });
	}

	post<T>({ url, query, body, type }: Omit<Req, 'method'>): Promise<T | null> {
		return this.#request({ method: 'POST', url, query, body, type });
	}

	patch<T>({ url, query, body, type }: Omit<Req, 'method'>): Promise<T | null> {
		return this.#request({ method: 'PATCH', url, query, body, type });
	}

	delete<T>({ url, query, body, type }: Omit<Req, 'method'>): Promise<T | null> {
		return this.#request({ method: 'DELETE', url, query, body, type });
	}
}

export const request = new FetchHandler();

export function appendPx<T extends Record<string, number>>(values: T): T {
	const result = {} as T;
	Object.entries(values).forEach(([key, value]) => {
		result[key as keyof T] = `${value}px` as any;
	});
	return result;
}

export async function attempt<T>(fn: Promise<T>): Promise<[Error | null, T]> {
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
