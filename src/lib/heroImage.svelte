<script>
    import alcatraz from "$lib/assets/alcatraz.jpg?enhanced";
    // import dogs from "$lib/assets/dogs.jpg?enhanced";
    import halfDome from "$lib/assets/half-dome.jpg?enhanced";
    import iceland from "$lib/assets/iceland.jpg?enhanced";
    import snowyRoad from "$lib/assets/snowy-road.jpg?enhanced";

    const images = [halfDome, alcatraz, iceland, snowyRoad];
    var lastImage = 0;
    var currentImage = 0;

    setInterval(() => {
        currentImage = (currentImage + 1) % images.length;
    }, 10000);
</script>

{#if currentImage == lastImage}
<enhanced:img src={images[currentImage]} alt="Hero" class="hero-image" />
{:else}
<enhanced:img src={images[lastImage]} alt="Hero" class="hero-image" />
<enhanced:img src={images[currentImage]} alt="Hero" class="hero-image new-hero-image" on:animationend={() => lastImage = currentImage} />
{/if}
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

    .new-hero-image {
        animation: fadeIn 4s;
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

    @keyframes fadeIn {
        from {
            opacity: 0;
        }
        to {
            opacity: 1;
        }
    }
</style>