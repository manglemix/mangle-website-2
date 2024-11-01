<script>
    import { enhance } from '$app/forms'; // Import the enhance action

    export let form;
</script>

<form method="POST" use:enhance>
    <label for="passkey">
        Passkey
    </label>
    <input type="text" id="passkey" name="passkey" required>
    <button>
        Ping Me!
    </button>
</form>

{#if form?.success}
    <output id="success">
        The ping was sent!
    </output>
{:else if form}
    <output id="failed">
        The passkey is invalid or was invalidated
    </output>
{/if}

<style>
    output {
        border-radius: 1rem;
        padding: 1rem;
        font-weight: bold;
        margin-top: 1rem;
    }

    #success {
        background-color: green;
    }

    #failed {
        background-color: red;
    }

    form button {
        margin-top: 0.7rem;
    }
    form label {
        font-weight: bold;
    }

    form {
        display: flex;
        flex-direction: column;
        align-items: start;
    }

    form input {
        color: black;
        padding: 0.5rem;
        border-radius: 0.5rem;
    }
</style>