<script>
    import Greeting from '$lib/greeting.svelte';
    import FancyPanel from '$lib/fancyPanel.svelte';
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
<FancyPanel style="margin-top: 5rem; display: flex; flex-direction: column;" id="robotics">
    <div class="flex flex-row">
        <iframe id="robot-vid" src="https://www.youtube.com/embed/OvzOrHbLd_M?si=4wOgsB6juSaCcEqy&amp;start=743" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
        <p>
            <enhanced:img src="$lib/assets/usr.png" alt="Utah Student Robotics" id="usr-logo" />
            The competition is simple. Move the most amount of material from the excavation zone into the dump zone, and don't hit any rocks on the way.
            In 2024, we did just that. We moved the most amount of moon dirt (<a href="https://spaceresourcetech.com/products/lhs-1-lunar-highlands-simulant">LHS-1</a>)
            out of all 50 teams from universities around the US and the military. My main contribution was implementing all of the software that allowed 2 pilots
            to simultaneously commandeer the robot wirelessly and in real time using 6 camera feeds streamed using FFmpeg over RTP. We also received points for being
            able to dig and dump autonomously using state machines. Due to our robust and noise-resistant communication protocol over UDP, we had the smallest bandwidth
            usage whilst having enough camera coverage and fidelity to avoid using the arena provided cameras.
        </p>
    </div>
    <div class="flex flex-row">
        <p>
            
        </p>
    </div>
</FancyPanel>

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
    #robot-vid {
        min-width: min(25rem, 100%);
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
        background-image: radial-gradient(rgb(65, 65, 65) 20%, transparent 73%);
    }
</style>