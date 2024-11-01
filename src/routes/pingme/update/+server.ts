import { createClient } from 'redis';
import { PING_REST_API_URL, PING_REST_API_PORT, PING_REST_API_TOKEN } from '$env/static/private';

export async function POST({ request }) {
    const pings = createClient({
        password: PING_REST_API_TOKEN,
        socket: {
            host: PING_REST_API_URL,
            port: PING_REST_API_PORT
        }
    });
    await pings.connect();
    
	const addr = await request.text();
    console.log(addr);
    pings.set("ping_addr", addr);
	return new Response(null, { status: 204 });
}