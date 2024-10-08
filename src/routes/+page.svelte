<script>
    import Greeting from '$lib/greeting.svelte';
</script>

<svelte:head>
	<title>Najman Husaini</title>
</svelte:head>

<Greeting />
<hr id="greeting-hr">


<style>
    #greeting-hr {
        margin-top: 4rem;
        box-shadow: 0 0 30px rgb(119, 119, 119);
    }
</style>