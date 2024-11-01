export async function POST({ request }) {
	const addr = await request.text();
    console.log(addr);
	return new Response(null, { status: 204 });
}