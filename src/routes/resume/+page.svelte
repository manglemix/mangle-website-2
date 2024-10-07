<script>
	import Highlight from 'svelte-highlight';
	import vbscriptHtml from 'svelte-highlight/languages/vbscript-html';
	import base16Monokai from 'svelte-highlight/styles/base16-monokai';

	export let data;
</script>

<svelte:head>
	<title>Resume | Najman Husaini</title>
	{@html base16Monokai}
</svelte:head>

<embed id="resume-pdf" src="/resume.pdf" type="application/pdf" />

<h2>Resume Source</h2>
<p>I wrote my resume with HTML and CSS because I love flexbox.</p>
<Highlight language={vbscriptHtml} code={data.resumeSrc} />

<style>
	#resume-pdf {
		width: 100%;
		height: 100vh;
	}

	h2 {
		margin-top: 2rem;
	}

	p {
		margin-bottom: 0.5rem;
	}
</style>
