<script>
    import Greeting from '$lib/greeting.svelte';
    import FancyPanel from '$lib/fancyPanel.svelte';
    import ThreeBody from '$lib/threeBody.svelte';
    import Roulette from '$lib/roulette.svelte';
	import { getContext } from 'svelte';

    import rust from "$lib/assets/rust.png?enhanced";
    import wgpu from "$lib/assets/wgpu.png?enhanced";
    import tokio from "$lib/assets/tokio.png?enhanced";
    import godot from "$lib/assets/godot.png?enhanced";
	import HeroImage from '$lib/heroImage.svelte';

	const highlightEmail = getContext('highlightEmail');
</script>

<svelte:head>
	<title>Najman Husaini</title>
</svelte:head>

<HeroImage />

<Greeting>
    <button id="contact" on:click={() => {
        scroll(0, document.body.scrollHeight);
        highlightEmail.set(true);
    }}>
        Contact
    </button>
</Greeting>
<!-- <hr id="greeting-hr"> -->
<p id="accreditation">Photos taken by me with a Sony A7iii</p>
<h1>Summary</h1>
<p id="summary">
    &emsp;&emsp;&emsp;I am a Senior Year Software Development student at the University of Utah. My degree has an emphasis on game development,
    and I have extensive experience with game engines such as Godot and Unreal Engine 5. I am also the programming team lead
    of Utah Student Robotics, a student organization that competes in the annual NASA Lunabotics competition against 50 teams.
    Factoring in my coursework and experience as a Teaching Assistant for the School of Computing, I have formidable talent
    in multiple domains of software development along with the confidence to communicate and collaborate effectively.
</p>
<FancyPanel panelClass="mt-8" style="display: flex; flex-direction: column;" id="robotics">
    <h2 class="mb-3"><enhanced:img src="$lib/assets/usr.png" alt="Utah Student Robotics" id="usr-logo" />Utah Student Robotics</h2>
    <div class="flex flex-row flex-wrap gap-4">
        <div id="robot-vid">
            <iframe src="https://www.youtube.com/embed/OvzOrHbLd_M?si=4wOgsB6juSaCcEqy&amp;start=743" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" referrerpolicy="strict-origin-when-cross-origin" allowfullscreen></iframe>
        </div>
        <p style="min-width: 50%; width: min-content; flex-grow: 3;">
            The competition is simple. Move the most amount of material from the excavation zone into the dump zone, and don't hit any rocks on the way.
            In 2024, we did just that. We moved the most amount of moon dirt (<a href="https://spaceresourcetech.com/products/lhs-1-lunar-highlands-simulant">LHS-1</a>)
            out of all 50 teams from universities around the US and the military. My main contribution was implementing all of the software that allowed 2 pilots
            to simultaneously commandeer the robot wirelessly and in real time using 6 camera feeds streamed using FFmpeg over RTP. We also received points for being
            able to dig and dump autonomously using state machines. Due to our robust and noise-resistant communication protocol over UDP, we had the smallest bandwidth
            usage whilst having enough camera coverage and fidelity to avoid using the arena provided cameras.
        </p>
    </div>
    <div class="flex flex-row mt-8 flex-wrap gap-12 pr-6 pb-4">
        <div class="flex-grow flex flex-col" style="min-width: 50%; width: min-content">
            <h2>
                Technology
            </h2>
            <p>
                The entire robot is programmed with Rust. <code>tokio</code> is used as the core model of execution, and <code>wgpu</code> with compute shaders
                written in <code>WGSL</code> is used for any GPU programming. To promote rapid prototyping, a feature-complete robotics framework was developed
                from the ground up in the footsteps of the Robot Operating System. This framework was called <code>unros</code> and is now superceeded by
                <a href="https://github.com/utahrobotics/lunadev-2025/tree/main/urobotics"><code>urobotics</code></a> for the 2025 competition. The mission control
                software is developed in Godot and is used to visualize the robot's state and direct the robot's autonomous actions.
            </p>
        </div>
        <Roulette images={[rust, wgpu, tokio, godot]} alts={["rust", "wgpu", "tokio", "godot"]} style=""/>
    </div>
</FancyPanel>

<FancyPanel style="margin-top: 5rem; display: flex; flex-direction: column;" id="games">
    <!-- <iframe width="552" height="167" frameborder="0" src="https://itch.io/embed/2860189"><a href="https://manglemix.itch.io/portalcrawler">PortalCrawler by manglemix</a></iframe> -->
</FancyPanel>
<ThreeBody></ThreeBody>

<style>
    h1 {
        font-size: 1.5rem;
        font-weight: bold;
    }
    h2 {
        font-size: 1.3rem;
        font-weight: bold;
    }
    :global(html) {
        scroll-behavior: smooth;
    }
    #summary {
        margin-top: 0.3rem;
    }
    #robot-vid {
        min-width: none;
        /* min-width: min(20rem, 100%); */
        flex-grow: 1;
    }
    #contact {
        margin-top: 0.5rem;
        margin-left: 0.5rem;
    }
    #robot-vid iframe {
        width: 100%;
        height: auto;
        aspect-ratio: 16 / 9;
    }
    #usr-logo {
        width: 2.5rem;
		height: auto;
        float: left;
        margin-right: 0.5rem;
        background-image: radial-gradient(rgb(65, 65, 65) 20%, transparent 73%);
    }
    a {
        text-decoration: underline;
    }
    #accreditation {
        font-style: italic; align-self: stretch; text-align: end; position: relative; top: -2rem; color: rgb(211, 211, 211);
        font-weight: lighter;
    }
</style>