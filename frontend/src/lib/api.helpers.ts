import { defined } from '$helpers';

type ClientRequestContentType = 'application/json' | 'multipart/form-data';

export type ClientRequest =
	| {
			method: 'GET';
			url: string;
			query?: Record<string, any>;
	  }
	| {
			method: 'DELETE';
			url: string;
			query?: Record<string, any>;
			body?: Record<string, any>;
	  }
	| {
			method: 'PATCH';
			url: string;
			query?: Record<string, any>;
			body: Record<string, any>;
			content_type: Extract<ClientRequestContentType, 'application/json'>;
	  }
	| ({
			method: 'POST';
			url: string;
			query?: Record<string, any>;
	  } & (
			| {
					body: Record<string, any>;
					content_type: Extract<ClientRequestContentType, 'application/json'>;
			  }
			| {
					body: Record<string, any>;
					content_type: Extract<ClientRequestContentType, 'multipart/form-data'>;
			  }
			| { body?: never; content_type?: never }
	  ));

export const requestHandler: {
	[M in ClientRequest['method']]: (request: Extract<ClientRequest, { method: M }>) => {
		method: M;
		url: URL;
		body?: string | FormData;
		headers?: HeadersInit;
	};
} = {
	GET: (request) => {
		const url = new URL(request.url);
		if (request.query) url.search = new URLSearchParams(request.query).toString();
		return { method: 'GET', url };
	},
	DELETE: (request) => {
		const url = new URL(request.url);
		if (request.query) url.search = new URLSearchParams(request.query).toString();
		return { method: 'DELETE', url, body: JSON.stringify(request.body) };
	},
	PATCH: (request) => {
		const url = new URL(request.url);
		if (request.query) url.search = new URLSearchParams(request.query).toString();
		return {
			method: 'PATCH',
			url,
			body: JSON.stringify(request.body),
			headers: { 'Content-Type': request.content_type }
		};
	},
	POST: (request) => {
		const url = new URL(request.url);
		if (request.query) url.search = new URLSearchParams(request.query).toString();

		if (request.content_type === 'application/json') {
			return {
				method: 'POST',
				url,
				body: JSON.stringify(request.body),
				headers: { 'Content-Type': request.content_type }
			};
		} else if (request.content_type === 'multipart/form-data') {
			const formData = new FormData();
			// Loop over the body object and append each key-value pair to the FormData object.
			for (const [key, value] of Object.entries(request.body)) {
				if (defined(value)) formData.append(key, value);
			}

			return {
				method: 'POST',
				url,
				body: formData
			};
		}

		return {
			method: 'POST',
			url
		};
	}
};

export type ServerResponse =
	| { content_type: 'image/jpeg'; data_type: Blob }
	| { content_type: 'application/json'; data_type: Record<string, any> };

export const responseHandler: {
	[C in ServerResponse['content_type']]: (
		response: Response
	) => Promise<ServerResponse['data_type']>;
} = {
	'image/jpeg': async (response) => await response.blob(),
	'application/json': async (response) => await response.json()
};
