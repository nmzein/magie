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
