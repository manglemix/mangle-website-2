<script lang="ts">
	var greetingIndex = 0;

	let intervalId = setInterval(() => {
		greetingIndex++;
	}, 90);

	$: if (greetingIndex >= 32) {
		clearInterval(intervalId);
	}
</script>

<header id="greeting-header">
	<section id="greeting">
		{#if greetingIndex >= 32}
			<h1>Hello World!</h1><p><span id="i-am">I am</span><span id="name-title">Najman Husaini</span></p>
		{:else if greetingIndex >= 19}
			<h1>Hello World!</h1><p><span id="i-am">I am</span><span id="name-title">{"Najman Husaini".substring(0, greetingIndex - 18)}<div id="caret" /></span></p>
		{:else if greetingIndex >= 13}
			<h1>Hello World!</h1><p><span id="i-am">{"I am".substring(0, greetingIndex - 13)}<div id="caret" /></span><span id="name-title" style="visibility: hidden">Najman Husaini</span></p>
		{:else if greetingIndex > 0}
			<h1>{"Hello World!".substring(0, greetingIndex)}<div id="caret" /></h1><p><span id="name-title" style="visibility: hidden">Najman Husaini</span></p>
		{:else}
			<h1 style="visibility: hidden">Hello World!</h1><p><span id="name-title" style="visibility: hidden">Najman Husaini</span></p>
		{/if}
        <slot></slot>
	</section>
</header>

<style>
	h1 {
		font-size: 3rem;
		font-weight: 600;
		display: flex;
		flex-direction: row;
		align-items: stretch;
	}
	p span {
		display: inline-flex;
		flex-direction: row;
		align-items: stretch;
	}

	#i-am {
		font-size: 1.5rem;
		font-weight: 600;
	}

	#name-title {
		font-size: 2rem;
		margin-left: 0.5rem;
		font-weight: 600;
		align-self: end;
	}

	header {
		display: flex;
		flex-direction: row;
		flex-wrap: wrap;
		gap: 2rem;
		justify-content: space-between;
	}

	#greeting-header {
		height: calc(100vw / 3 * 2);
		/* position: absolute;
		top: 4rem;
		left: 0;
		width: 100vw;
		aspect-ratio: 16 / 9; */
	}

	#caret {
		width: 0.4rem;
		margin: 0.5rem;
		background-color: white;
		display: inline-block;
	}
</style>
