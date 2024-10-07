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
    <div id="hero-wasm-container">
    </div>
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
        flex-grow: 1;
    }
</style>
