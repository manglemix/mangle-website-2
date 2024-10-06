
export async function load({ fetch }) {
	return {
		resumeSrc: await (await fetch(`/resume.html`)).text()
	};
}