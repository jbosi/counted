import * as z from 'zod';

export const PROJECT_FORM_SCHEMA = z.object({
	projectName: z.string().min(2).max(100),
	users: z
		.array(
			z.object({
				name: z.string().min(2).max(100),
			}),
		)
		.min(2),
	projectDescription: z.string().max(200).optional(),
});
