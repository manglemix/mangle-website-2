<header>
    <section id="header-buttons">
        <a href="/"><h2>Home</h2></a>
        <a href="/timeline"><h2>Timeline</h2></a>
        <a href="/resume"><h2>Resume</h2></a>
    </section>
</header>

<div id="content">
    <main>
        <slot></slot>
    </main>
</div>
    
<footer>
    <div id="footer-content">
        hello
    </div>
</footer>

<style>
    @import url('https://fonts.googleapis.com/css2?family=Open+Sans:ital,wght@0,300..800;1,300..800&display=swap');

    :global(*) {
        margin: 0;
        padding: 0;
        color: whitesmoke;
        font-family: "Open Sans", sans-serif;
        scroll-margin-top: 5rem;
    }

    :global(body) {
        background-color: #111111;
    }

    #header-buttons {
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
        width: 100%;
        display: flex;
        flex-direction: column;
        align-items: stretch;
    }

    main {
        margin-top: 1rem;
        min-height: calc(100vh - 10rem);
    }

    main, #footer-content {
        display: flex;
        flex-direction: column;
        align-self: center;
        width: min(50rem, 100%);
    }

    footer {
        margin-top: 1rem;
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 4rem;
        background-color: #181818;
        box-shadow: 0 0 10px 0 rgba(0, 0, 0, 1);
    }
</style>