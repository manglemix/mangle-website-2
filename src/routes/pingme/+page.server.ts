import { createClient } from 'redis';
import { PING_REST_API_URL, PING_REST_API_PORT, PING_REST_API_TOKEN } from '$env/static/private';

export const actions = {
	default: async (event) => {
        const pings = createClient({
            password: PING_REST_API_TOKEN,
            socket: {
                host: PING_REST_API_URL,
                port: PING_REST_API_PORT
            }
        });
        await pings.connect();

        const addr = await pings.get("ping_addr") as string;
        console.log(addr);
		event.fetch(new URL("http://" + addr), {
            method: "POST",
            body: "pingmetest"
        });

        return { success: true };
	}
};