export const actions = {
	default: async (event) => {
		event.fetch("http://127.0.0.1:30000", {
            method: "POST",
            body: "pingmetest"
        });

        return { success: true };
	}
};