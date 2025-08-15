<script>
	import Greeting from '$lib/greeting.svelte';
	import FancyPanel from '$lib/fancyPanel.svelte';
	import ThreeBody from '$lib/threeBody.svelte';
	import Roulette from '$lib/roulette.svelte';
	import { getContext } from 'svelte';

	import rust from '$lib/assets/rust.png?enhanced';
	import wgpu from '$lib/assets/wgpu.png?enhanced';
	import tokio from '$lib/assets/tokio.png?enhanced';
	import godot from '$lib/assets/godot.png?enhanced';
	import HeroImage from '$lib/heroImage.svelte';
	import UsrInsta from '$lib/usrInsta.svelte';

	const highlightEmail = getContext('highlightEmail');

	var webProjectIndex = 0;
	const maxWebProjects = 3;
</script>

<svelte:head>
	<title>Najman Husaini</title>
	<meta property="og:type" content="website" />
	<meta property="og:url" content="https://www.najmanhusaini.com/" />
	<meta property="og:title" content="Najman Husaini" />
	<meta property="og:description" content="Najman Husaini's Personal Portfolio" />
	<meta property="og:image" content="https://www.najmanhusaini.com/favicon.png" />
</svelte:head>

<HeroImage />

<Greeting>
	<button
		id="contact"
		on:click={() => {
			scroll(0, document.body.scrollHeight);
			highlightEmail.set(true);
		}}
	>
		Contact
	</button>
</Greeting>

<p id="accreditation">Photos taken by me with a Sony A7iii</p>
<h1 class="pl-4 pr-4">Summary</h1>
<p id="summary">
	&emsp;&emsp;&emsp;I am a Senior Year Software Development student at the University of Utah. My
	degree has an emphasis on game development, and I have extensive experience with game engines such
	as Godot and Unreal Engine 5. I was also the programming team lead of Utah Student Robotics, a
	student organization that competes in the annual NASA Lunabotics competition against 50 teams.
	Factoring in my coursework and experience as a Teaching Assistant for the School of Computing, I
	have formidable talent in multiple domains of software development along with the confidence to
	communicate and collaborate effectively, and I demostrated this as an intern at Silverlake Axis
	and I am looking to transition into the industry.
</p>
<FancyPanel panelClass="mt-16 flex flex-col" id="robotics">
	<h2 class="mb-3 text-center flex flex-row justify-center self-stretch">
		<enhanced:img src="$lib/assets/usr.png" alt="Utah Student Robotics" id="usr-logo" />Utah Student
		Robotics 1st Place
	</h2>
	<div class="flex flex-row flex-wrap gap-4">
		<div id="robot-vid">
			<iframe
				src="https://www.youtube.com/embed/riIPlJ0fpuw?si=d1aYAoKkQ-2I_coQ&amp;start=33194"
				title="YouTube video player"
				frameborder="0"
				allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
				referrerpolicy="strict-origin-when-cross-origin"
				allowfullscreen
			></iframe>
		</div>
		<p style="min-width: 50%; width: min-content; flex-grow: 3;">
			The competition is simple. Move the most amount of material from the excavation zone into the
			dump zone, and don't hit any rocks on the way. In 2024, we got 3rd place doing that, and in 2025
			we got 1st place. We moved the most amount of moon dirt (<a
				href="https://spaceresourcetech.com/products/lhs-1-lunar-highlands-simulant">LHS-1</a
			>) both years out of all 50+ teams from universities around the US and the military. My team and I
			developed the mission control software and robot systems almost from scratch in Rust (still the only
			team to do so) and we received numerous awards. Not only was our bandwidth the smallest, we were also
			the only team capable of streaming 7 cameras at once along with telemetry and remote control, which
			earned us an award in wireless communication.
		</p>
	</div>
	<div class="flex flex-row mt-8 flex-wrap gap-8 pb-4">
		<div class="flex-grow flex flex-col" style="min-width: 50%; width: min-content">
			<h2>Technology</h2>
			<p>
				The entire robot is programmed with Rust. <code>tokio</code> is used as the core model of
				execution, <code>rayon</code> for CPU level parallelism, and <code>wgpu</code> with compute
				shaders written in <code>WGSL</code> is used for any GPU programming. The code base can be
				explored at 
				<a href="https://github.com/utahrobotics/lunadev-2025/tree/main/urobotics"
					><code>lunadev-2025</code></a
				> for the 2025 competition. The mission control software is developed in Godot and is used to
				visualize the robot's state and direct the robot's autonomous actions. Communication uses a
				protocol that we developed for extreme minimalism over UDP, and we created a custom Behavior
				Tree implementation that is strongly typed and allows for certain compile-time safety checks.
			</p>
		</div>
		<Roulette
			images={[rust, wgpu, tokio, godot]}
			alts={['rust', 'wgpu', 'tokio', 'godot']}
			style="margin-right: 1.4rem; margin-left: 1.2rem"
		/>
	</div>
</FancyPanel>

<UsrInsta />

<FancyPanel panelClass="mt-16 flex flex-col" id="games">
	<h2 class="mb-3 text-center">Game Development</h2>
	<p>
		There are about 3.22 billion self-reported gamers around the world, and the market size is about
		220 billion USD and only going up. Despite how lucrative this might seem, I studied game
		development at the University of Utah because I wanted to make games that I would enjoy playing.
		The first proper game engine I learned was Godot and it is by far the engine I have the most
		experience in. Here at the U, I have taken courses in Unreal Engine 5 and GameMaker, and am now
		in Senior Capstone using Unity. Shown below is a game I made in my Traditional Game Development
		class.
	</p>
	<div id="itch-embed">
		<iframe frameborder="0" src="https://itch.io/embed/2860189" title="Itch Embed"
			><a href="https://manglemix.itch.io/portalcrawler">PortalCrawler by manglemix</a></iframe
		>
	</div>
	<div class="flex flex-row flex-wrap gap-4">
		<div id="bola-snippet">
			<enhanced:img src="$lib/assets/bola.png" alt="Bola Screenshot" />
		</div>
		<p style="min-width: 50%; width: min-content; flex-grow: 3;">
			<b><a href="https://bola.manglemix.com/">Bola</a></b> was the first game I made in college. It
			was a 2D physics platformer made in Godot that underwent several iterations, many of which I
			wrote during my classes. My friends enjoyed it and influenced its development, so of course
			the next natural step was to add leaderboards and multiplayer. The leaderboard database API
			was implemented in Rust with <code>Rocket</code> but have since been taken down. Multiplayer was
			never finished. I subsequently practiced developing multiplayer with an FPS shooter, also in Godot,
			and was able to have 2 players and a spectator fight against one AI, but that was the extent of
			it.
		</p>
	</div>
	<div id="steam-embed">
		<iframe src="https://store.steampowered.com/widget/3566060/" frameborder="0" width="500" height="190" title="Steam Embed"></iframe>
	</div>
	
	<p class="self-center" style="max-width: 45rem">
		Tiny Sheriff is the last video game I helped develop at the University of Utah. It is one of 6 games of
		the senior year undergraduates and each team is comprised of 30+ students. The game is developed exclusively
		in Unity with C#, and it represents a 2 semester effort from August 2024 to May 2025. The game includes full
		integration with Steam including Cloud Saves, Achievements, and Statistics. While the game is certainly an
		achievement for all of us in the team, we struggled with leadership without adequate support from faculty, and
		I believe we would all be able to create a much more compelling game with more mentoring.
	</p>
	<p class="italic self-center mt-4">
		Honorable mention: <a href="https://manglemix.itch.io/universe-within">Universe Within</a>
	</p>
</FancyPanel>

<FancyPanel panelClass="flex flex-col gap-4 mt-16 items-stretch" id="games">
	<div class="flex flex-row justify-between">
		<button
			style="font-size: 2rem"
			on:click={() => (webProjectIndex = (maxWebProjects + webProjectIndex - 1) % maxWebProjects)}
		>
			👈
		</button>
		<h2>Web Projects</h2>
		<button
			style="font-size: 2rem"
			on:click={() => (webProjectIndex = (webProjectIndex + 1) % maxWebProjects)}
		>
			👉
		</button>
	</div>
	<div class="flex flex-row-reverse gap-8 flex-wrap">
		{#if webProjectIndex == 0}
			<ThreeBody style="flex-grow: 1"></ThreeBody>
			<div class="flex flex-col w-min" style="min-width: 40%; flex-grow: 4;">
				<h2>3 Body Problem</h2>
				<p>
					A simulation of three gravitational bodies written in Rust and compiled to WebAssembly.
					The simulation is done on the GPU using WebGPU through <code>wgpu</code> and
					<code>wgsl</code>. <code>Rapier2D</code> was used to implement physics collisions. The color
					of each pixel represents which body has the most gravitational influence. For example, the
					region near the blue circle is shaded blue as most of the gravity is coming from the blue circle.
					However, at the exact center of the canvas at the beginning, the color is black as the gravity
					from all 3 bodies cancel each other out.
				</p>
			</div>
		{:else if webProjectIndex == 1}
			<div style="min-width: 15rem; max-width: 40%; flex-grow: 1; margin: auto;">
				<enhanced:img src="$lib/assets/detrash.png" alt="DeTrash" />
			</div>
			<div class="flex flex-col w-min" style="min-width: 40%; flex-grow: 4;">
				<h2>DeTrash</h2>
				<p>
					My team's entry into the Hack OHI/O 11 hackathon that received 2nd place. Users can submit
					images of trash which are verified by YOLOv5 and then geotagged. The geotagged trash is
					then displayed on a heat-map that clean-up crews can use to prioritize areas to clean. The
					project was developed in SvelteKit with a backend developed in Flask, with Google Maps
					integration. While the backend is not active, you can still check out the website <a
						href="https://trash-tests.vercel.app/">here</a
					>.
				</p>
			</div>
		{:else}
			<div style="min-width: 15rem; max-width: 40%; flex-grow: 1; margin: auto;">
				<enhanced:img src="$lib/assets/chat-a-way.png" alt="ChatAWay" />
			</div>
			<div class="flex flex-col w-min" style="min-width: 40%; flex-grow: 4;">
				<h2>Chat-A-Way</h2>
				<p>
					Another team entry, but this time for MHacks 2023. Chat-A-Way is a chat application with
					GPT-4 that can create routes using Google Maps based on what the user wants. If the user
					describes an interest in watching a movie then eating dessert at a specific restaurant,
					Chat-A-Way will generate a route that takes the user to the movie theater first and then
					to the restaurant. The project was also developed in SvelteKit with a backend developed
					with LangChain.
				</p>
			</div>
		{/if}
	</div>
</FancyPanel>

<section id="outro">
	<p>
		Thanks for taking the time to check out my website. This may be my largest frontend project so
		far.
	</p>
	<p>If you are a <b>recruiter</b>, consider checking out my <a href="/resume">resume</a>.</p>
	<p>
		If you are interested in viewing the <b>source code</b>, check it out
		<a href="https://github.com/manglemix/mangle-website-2">here</a>.
	</p>
</section>

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
		padding-left: 1rem;
		padding-right: 1rem;
	}
	#robot-vid {
		min-width: none;
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
		font-style: italic;
		align-self: stretch;
		text-align: end;
		position: relative;
		top: -2rem;
		color: rgb(211, 211, 211);
		font-weight: lighter;
		padding-left: 1rem;
		padding-right: 1rem;
	}

	#itch-embed {
		width: 100%;
		margin-top: 2rem;
		margin-bottom: 2rem;
	}

	#steam-embed {
		width: 100%;
		margin-top: 2rem;
		padding-left: calc(50% - 250px);
	}

	#itch-embed iframe {
		width: 100%;
		height: auto;
	}

	#bola-snippet {
		min-width: 10rem;
		max-width: 20rem;
		flex-grow: 1;
	}

	#outro {
		align-self: stretch;
		display: flex;
		flex-direction: column;
		gap: 0.7rem;
		margin-top: 6rem;
		margin-bottom: 6rem;
		padding-left: 1rem;
		padding-right: 1rem;
	}

	#outro p {
		text-align: end;
	}

	p {
		text-align: justify;
	}
</style>
