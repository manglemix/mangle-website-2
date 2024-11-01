import { createClient } from '@vercel/kv';
import { PING_REST_API_URL, PING_REST_API_TOKEN } from '$env/static/private';

export const actions = {
	default: async (event) => {
        const pings = createClient({
            url: PING_REST_API_URL,
            token: PING_REST_API_TOKEN,
        });

        const addr = await pings.get("ping_addr") as string;
		event.fetch(new URL(addr), {
            method: "POST",
            body: "pingmetest"
        });

        return { success: true };
	}
};