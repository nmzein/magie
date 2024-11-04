import type { Directory } from '$types';
import { http } from '$api';

export class RepositoryState {
	public registry: Directory | undefined = $state();
	public generators: string[] = $state([]);
	public decoders: string[] = $state(['Auto (default)']);
	public encoders: string[] = $state(['OMEZarr']);

	constructor() {
		http.GetRegistry().then((registry) => {
			if (registry === undefined) return;
			this.registry = registry;
		});

		http.GetGenerators().then((generators) => {
			if (generators === undefined) return;
			this.generators = generators;
		});
	}
}
