<script>
	import init, {
		set_body_1_mass,
		set_body_2_mass,
		set_body_3_mass,
		pause,
		play,
		reset,
		set_time_scale,
        run
	} from 'hero-wasm';
	import { onMount } from 'svelte';

	var body_1_mass = 1.0;
	var body_2_mass = 1.0;
	var body_3_mass = 1.0;
	var paused = true;
	var time_scale = 1.0;

	onMount(async () => {
		await init(); // init initializes memory addresses needed by WASM and that will be used by JS/TS
        run();
	});
</script>

<header>
	<section id="greeting">
		<h1>Hello World!</h1>
		<span id="i-am">I am</span><span id="name-title">Najman Husaini</span>
	</section>
	<section id="hero-wasm-panel">
		<h2>3 Body Problem</h2>
		<div id="hero-wasm-container"></div>
		<section id="pause-play">
			{#if paused}
				<button
					on:click={() => {
						paused = false;
						play();
					}}
				>
					Play
				</button>
			{:else}
				<button
					on:click={() => {
						paused = true;
						pause();
					}}
				>
					Pause
				</button>
			{/if}
			<button
				on:click={() => {
					paused = true;
					reset();
				}}
			>
				Reset
			</button>
		</section>
		<section id="time-controls">
			<label for="time-scale">Time Scale</label>
			0.5x
			<input
				id="time-scale"
				type="range"
				min="0.5"
				max="5.0"
				step="0.1"
				bind:value={time_scale}
				on:input={() => set_time_scale(time_scale)}
			/>
			5x
		</section>
		<h3>Masses</h3>
		<section id="wasm-sliders">
			<input
				type="range"
				min="10"
				max="100"
				bind:value={body_1_mass}
				on:input={() => set_body_1_mass(body_1_mass)}
			/>
			<input
				type="range"
				min="10"
				max="100"
				bind:value={body_2_mass}
				on:input={() => set_body_2_mass(body_2_mass)}
			/>
			<input
				type="range"
				min="10"
				max="100"
				bind:value={body_3_mass}
				on:input={() => set_body_3_mass(body_3_mass)}
			/>
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

	#pause-play {
		display: flex;
		flex-direction: row;
		gap: 0.5rem;
	}

	#pause-play button {
		width: 50%;
		background-color: #303030;
		font-weight: bold;
		font-size: 1.2rem;
		border-radius: 0.5rem;
	}

	#time-controls {
		display: flex;
		flex-direction: row;
		gap: 0.5rem;
	}

	#time-controls input {
		flex-grow: 1;
	}
</style>
