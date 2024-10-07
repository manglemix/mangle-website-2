<script>
    // https://www.mbaraa.com/blog/using-wasm-with-sveltekit
	import init, { } from 'hero-wasm';
	import { onMount } from 'svelte';

	onMount(async () => {
		await init(); // init initializes memory addresses needed by WASM and that will be used by JS/TS
	});
</script>

<svelte:head>
	<title>Najman Husaini</title>
</svelte:head>

<header>
    <section id="greeting">
        <h1>Hello World!</h1>
        <span id="i-am">I am</span><span id="name-title">Najman Husaini</span>
    </section>
    <section id="hero-wasm-panel">
        <h2>3 Body Problem</h2>
        <div id="hero-wasm-container">
        </div>
        <section id="wasm-sliders">
            <input type="range" min="0" max="100" value="50" />
            <input type="range" min="0" max="100" value="50" />
            <input type="range" min="0" max="100" value="50" />
        </section>
    </section>
</header>

<style>
	h1 {
		font-size: 3rem;
	}

	#i-am {
		font-size: 1.5rem;
		font-weight: 600;
	}

	#name-title {
		font-size: 2rem;
		margin-left: 0.5rem;
		font-weight: 600;
	}

    header {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;
        gap: 2rem;
        justify-content: space-between;
    }

    #hero-wasm-container {
        display: flex;
        overflow: clip;
        aspect-ratio: 1 / 1;
        border-radius: 2rem;
    }

    #hero-wasm-panel {
        flex-grow: 1;
        display: flex;
        max-width: 20rem;
        flex-direction: column;
        gap: 0.5rem;
        align-items: stretch;
    }

    #wasm-sliders {
        display: flex;
        flex-direction: row;
        gap: 0.5rem;
    }

    #wasm-sliders input {
        min-width: 0;
        flex-grow: 1;
    }
</style>
