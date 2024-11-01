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
    
	const body = await request.json();
    pings.set("ping_addr", body.url);
	return new Response(null, { status: 204 });
}