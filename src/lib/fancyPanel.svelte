<script lang="ts">
	var angle = 0.0;
	export let style = '';
	export let panelClass = '';
	export let id = '';
	var panel: HTMLElement;

	function handleMousemove(event: MouseEvent) {
		let rect = panel.getBoundingClientRect();
		let x = event.clientX - rect.x - rect.width / 2;
		let y = event.clientY - rect.y - rect.height / 2;
		angle = (Math.atan2(y, x) / Math.PI) * 180 + 90;
	}
</script>

<section
	class="panel {panelClass}"
	{id}
	style="--angle: {angle}deg;{style}"
	on:mousemove={handleMousemove}
	role="none"
	bind:this={panel}
>
	<slot></slot>
</section>

<style>
	.panel {
		background-image: linear-gradient(var(--angle), rgb(24, 24, 24), rgb(32, 32, 32));
		padding: 2rem;
		border-radius: 3rem;
		box-shadow: 0 5px 5px 0 rgba(0, 0, 0, 0.7);
	}
</style>
