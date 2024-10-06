<script>
    import Highlight from "svelte-highlight";
    import vbscriptHtml from "svelte-highlight/languages/vbscript-html";
    import ashes from "svelte-highlight/styles/ashes";
    
    export let data;
</script>

<svelte:head>
    <title>Resume | Najman Husaini</title>
    {@html ashes}
</svelte:head>

<embed id="resume-pdf" src="/resume.pdf" type="application/pdf">

<h2>Resume Source</h2>
<Highlight language={vbscriptHtml} code={data.resumeSrc} />
<style>
    #resume-pdf {
        width: 100%;
        height: 100vh;
    }

    h2 {
        margin-top: 2rem;
        margin-bottom: 0.5rem;
    }
</style>