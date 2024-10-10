<script lang="ts">
	import type { Picture } from 'vite-imagetools';
	export let images: Picture[] = [];
	export let alts: string[] = [];

	export let width = 2;
	export let height = 2;
	export let style = '';
</script>

<div {style} id="roulette-static">
	<div id="roulette">
		{#each { length: height } as _, y}
			<div class="row">
				{#each { length: width } as _, x}
					<enhanced:img src={images[y * width + x]} alt={alts[y * width + x]} class="image" />
				{/each}
			</div>
		{/each}
	</div>
</div>

<style>
	.row {
		display: flex;
		flex-direction: row;
		gap: 3rem;
	}
	#roulette {
		display: flex;
		flex-direction: column;
		gap: 3rem;
		animation-duration: 20s;
		animation-name: outerRotate;
		animation-iteration-count: infinite;
		animation-timing-function: linear;
		background-image: radial-gradient(rgb(46, 46, 46), transparent 75%);
	}
	.image {
		width: 6rem;
		height: auto;
		animation-duration: 20s;
		animation-name: innerRotate;
		animation-iteration-count: infinite;
		animation-timing-function: linear;
	}
	@keyframes outerRotate {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}
	@keyframes innerRotate {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(-360deg);
		}
	}
	#roulette-static {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}
</style>
