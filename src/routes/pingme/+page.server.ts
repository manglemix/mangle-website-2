import { createClient } from 'redis';
import { PING_REST_API_URL, PING_REST_API_PORT, PING_REST_API_TOKEN, EDGE_CONFIG } from '$env/static/private';
import { createClient as createVClient } from '@vercel/edge-config';

export const actions = {
	default: async ({ request, fetch }) => {
        const pings = createClient({
            password: PING_REST_API_TOKEN,
            socket: {
                host: PING_REST_API_URL,
                port: PING_REST_API_PORT
            }
        });
        await pings.connect();
		const data = await request.formData();
        const config = createVClient(EDGE_CONFIG);
        
        const valid_passkeys = (await config.get("valid_passkeys")) as string;
        const valid_passkeys_list = valid_passkeys.split(",");

        const passkey = data.get("passkey") as string;
        if (!valid_passkeys_list.includes(passkey)) {
            return { success: false };
        }

        const addr = await pings.get("ping_addr") as string;
        console.log(addr);
        console.log(passkey);
		await fetch(new URL(addr), {
            method: "POST",
            body: passkey
        });

        return { success: true };
	}
};