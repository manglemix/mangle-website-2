<script>
	import { browser } from '$app/environment';

    // https://www.mbaraa.com/blog/using-wasm-with-sveltekit
	import init, { set_body_1_mass, set_body_2_mass, set_body_3_mass } from 'hero-wasm';
	import { onMount } from 'svelte';

    var body_1_mass = 1.0;
    var body_2_mass = 1.0;
    var body_3_mass = 1.0;

	onMount(async () => {
		await init(); // init initializes memory addresses needed by WASM and that will be used by JS/TS
	});

    // if (browser) {
    //     $: set_body_1_mass(body_1_mass);
    //     $: set_body_2_mass(body_2_mass);
    //     $: set_body_3_mass(body_3_mass);
    // }
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
        <h3>Masses</h3>
        <section id="wasm-sliders">
            <input type="range" min="1" max="10" bind:value={body_1_mass} on:input={() => set_body_1_mass(body_1_mass)} />
            <input type="range" min="1" max="10" bind:value={body_2_mass} on:input={() => set_body_2_mass(body_2_mass)} />
            <input type="range" min="1" max="10" bind:value={body_3_mass} on:input={() => set_body_3_mass(body_3_mass)} />
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
