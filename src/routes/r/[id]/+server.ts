import { error, redirect, type RequestHandler } from '@sveltejs/kit';
import { get } from '@vercel/edge-config';

export const GET: RequestHandler = async ({ params }) => {
	const redirects: Record<string, string> = await get('redirects') ?? {};
	if (redirects[params.id!]) {
		redirect(307, redirects[params.id!]);
	}

	error(404, 'Not found');
};