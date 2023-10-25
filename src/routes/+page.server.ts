import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	try {
		const response = await fetch('http://127.0.0.1:3000/api/data');

		if (response.ok) {
			const data = await response.text();
			// Handle the data in your Svelte component
			return { data };
		} else {
			return 'Error';
		}
	} catch (error) {
		console.error(error);
	}
};
