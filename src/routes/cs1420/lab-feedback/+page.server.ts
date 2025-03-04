import type { Actions } from './$types';
import { LAB_FEEDBACK_WEBHOOK } from '$env/static/private';
import { WebhookClient } from 'discord.js';

export const actions = {
	default: async ({ request }) => {
		const data = await request.formData();
		let feedback = data.get('feedback')?.valueOf() as string;
		feedback = feedback.trim();

		let ta1 = data.get('ta1')?.valueOf() as string ?? "Unknown";
		let ta2 = data.get('ta2')?.valueOf() as string ?? "Unknown";
		ta1 = ta1.trim();
		ta2 = ta2.trim();
		
		const webhookClient = new WebhookClient({ url: LAB_FEEDBACK_WEBHOOK });

		await webhookClient?.send({
			content: `*${ta1} and ${ta2}*\n${feedback}`,
		});
	}
} satisfies Actions;
