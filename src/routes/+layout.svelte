<header>
	<nav>
		<a href="/"><h2>Home</h2></a>
		<a href="/timeline"><h2>Timeline</h2></a>
		<a href="/resume"><h2>Resume</h2></a>
	</nav>
</header>

<div id="content">
	<main>
		<slot></slot>
	</main>
</div>

<footer>
	<div id="footer-content">
		<div class="footer-col">
			<a href="https://www.github.com/manglemix/"
				><enhanced:img src="$lib/assets/github.png" alt="Github" class="logo" /> GitHub</a
			>
			<a href="https://www.instagram.com/najmakesgames/"
				><enhanced:img src="$lib/assets/insta.png" alt="Instagram" class="logo" /> Instagram</a
			>
		</div>
		<div class="footer-col">
			<a href="mailto:najmanhusaini20@gmail.com"
				><enhanced:img src="$lib/assets/mail.png" alt="Email" class="logo" /> Email</a
			>
			<a href="https://www.linkedin.com/in/najman-husaini/"
				><enhanced:img src="$lib/assets/linkedin.png" alt="LinkedIn" class="logo" /> LinkedIn</a
			>
		</div>
		<div class="footer-col">
			<a href="https://manglemix.itch.io/"
				><enhanced:img src="$lib/assets/itch.png" alt="Itch.io" class="logo" /> Itch.io</a
			>
			<a href="https://www.pexels.com/@manglemix/"
				><enhanced:img src="$lib/assets/pexels.png" alt="Pexels" class="logo" /> Pexels</a
			>
		</div>
	</div>
</footer>

<style>
	@import url('https://fonts.googleapis.com/css2?family=Open+Sans:ital,wght@0,300..800;1,300..800&display=swap');

	:global(*) {
		margin: 0;
		padding: 0;
		color: whitesmoke;
		font-family: 'Open Sans', sans-serif;
		scroll-margin-top: 5rem;
	}

	:global(body) {
		background-color: #111111;
	}

	h2 {
		font-size: 1.2rem;
	}

	nav {
		display: flex;
		justify-content: space-between;
		align-items: center;
		flex-direction: row;
		gap: 2rem;
	}

	header {
		position: sticky;
		top: 0;
		display: flex;
		flex-direction: row;
		justify-content: center;
		background-color: #181818;
		box-shadow: 0 0 10px 0 rgba(0, 0, 0, 1);
		height: 4rem;
	}

	a {
		text-decoration: none;
	}

	#content {
		width: calc(100% - 2rem);
		display: flex;
		flex-direction: column;
		align-items: stretch;
		padding: 1rem;
	}

	main {
		min-height: calc(100vh - 14rem);
		flex-direction: column;
	}

	main,
	#footer-content {
		display: flex;
		align-self: center;
		width: min(50rem, 100%);
	}

	#footer-content {
		flex-direction: row;
		justify-content: space-around;
	}

	.footer-col {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	footer {
		display: flex;
		justify-content: center;
		align-items: center;
		min-height: 8rem;
		background-color: #181818;
		box-shadow: 0 0 10px 0 rgba(0, 0, 0, 1);
	}

	footer .footer-col a {
		display: flex;
		flex-direction: row;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8rem;
	}

	:global(footer .footer-col a picture) {
		display: flex;
		flex-direction: row;
		align-items: center;
	}

	.logo {
		width: 1.5rem;
		height: auto;
	}
</style>
