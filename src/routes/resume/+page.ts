import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch }) => {
	return {
		resumeSrc: await (await fetch(`/resume.html`)).text()
	};
}