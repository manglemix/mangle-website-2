<script>
    import Greeting from '$lib/greeting.svelte';
	import { getContext } from 'svelte';

	const highlightEmail = getContext('highlightEmail');
</script>

<svelte:head>
	<title>Najman Husaini</title>
</svelte:head>

<Greeting>
    <button id="contacts" on:click={() => {
        scroll(0, document.body.scrollHeight);
        highlightEmail.set(true);
    }}>
        Contact
    </button>
</Greeting>
<hr id="greeting-hr">
<h1>Summary</h1>
<p id="summary">
    &emsp;&emsp;&emsp;I am a Senior Year Software Development student at the University of Utah. My degree has an emphasis on game development,
    and I have extensive experience with game engines such as Godot and Unreal Engine 5. I am also the programming team lead
    of Utah Student Robotics, a student organization that competes in the annual NASA Lunabotics competition against 50 teams.
    Factoring in my coursework and experience as a Teaching Assistant for the School of Computing, I have formidable talent
    in multiple domains of software development along with the confidence to communicate and collaborate effectively.
</p>
<section id="robotics">
    <p>
        <iframe id="robot-vid" src="https://www.youtube.com/embed/OvzOrHbLd_M?si=4wOgsB6juSaCcEqy&amp;start=743" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
        <enhanced:img src="$lib/assets/usr.png" alt="Utah Student Robotics" id="usr-logo" />
        TODO
    </p>
</section>

<style>
    #contacts {
        background: rgb(51, 51, 51);
        padding: 0.5rem 0.7rem 0.5rem 0.7rem;
        margin-left: 0.5rem;
        border-radius: 0.6rem;
        border: none;
        font-weight: bold;
        font-size: 1.1rem;
        margin-top: 0.5rem;
    }
    #contacts:active {
        background: rgb(32, 32, 32);
    }
    #greeting-hr {
        margin-top: 4rem;
        margin-bottom: 1rem;
        box-shadow: 0 0 30px rgb(119, 119, 119);
    }
    :global(html) {
        scroll-behavior: smooth;
    }
    #summary {
        margin-top: 0.3rem;
    }
    #robotics {
        margin-top: 2rem;
    }
    /* .section-col {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    } */
    #robot-vid {
        width: min(25rem, 100%);
        height: auto;
        aspect-ratio: 16 / 9;
        float: left;
        margin-right: 1rem;
        margin-bottom: 1rem;
    }
    #usr-logo {
        width: 8rem;
		height: auto;
        float: left;
        margin-right: 0.5rem;
    }
</style>