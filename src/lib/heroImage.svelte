<script>
    import alcatraz from "$lib/assets/alcatraz.jpg?enhanced";
    // import dogs from "$lib/assets/dogs.jpg?enhanced";
    import halfDome from "$lib/assets/half-dome.jpg?enhanced";
    import iceland from "$lib/assets/iceland.jpg?enhanced";
    import snowyRoad from "$lib/assets/snowy-road.jpg?enhanced";
</script>

<enhanced:img src={halfDome} alt="Hero Image" class="hero-image" />
<div class="image-occlude">
</div>

<style>
    .hero-image {
        z-index: -2;
        position: absolute;
        top: 4rem;
        width: 100%;
        left: 0;
    }

    .image-occlude {
        background-image: linear-gradient(transparent 80%, #111111 95%);
        position: absolute;
        top: 4rem;
        width: 100%;
        left: 0;
        z-index: -1;
        height: calc(100vw / 3 * 2 + 1px);
    }
</style>