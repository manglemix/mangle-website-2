<form method="POST" class="flex flex-col gap-4">
	<label class="flex flex-row gap-4">
        1st TA
        <input type="text" name="ta1" class="p-2" required>
	</label>
    <label class="flex flex-row gap-4">
        2nd TA
        <input type="text" name="ta2" class="p-2">
    </label>
	<label class="flex flex-col">
		Feedback
        <textarea name="feedback" cols="40" rows="5" class="p-2" maxlength="1700" required></textarea>
	</label>
	<button>Submit</button>
</form>

<style>
    textarea, input {
        color: black;
    }
</style>