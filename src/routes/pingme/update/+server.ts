import { createClient } from '@vercel/kv';
import { PING_REST_API_URL, PING_REST_API_TOKEN } from '$env/static/private';

export async function POST({ request }) {
    const pings = createClient({
        url: PING_REST_API_URL,
        token: PING_REST_API_TOKEN,
    });
    
	const addr = await request.text();
    pings.set("ping_addr", addr);
	return new Response(null, { status: 204 });
}