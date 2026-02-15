import * as z from 'zod';

const userSchema = z.object({
	name: z.string().min(2, 'Le nom doit contenir au moins 2 caractères').max(100),
	userId: z.number().optional(),
});

export const PROJECT_FORM_SCHEMA = z.object({
	projectName: z.string().min(2).max(100),
	users: z.array(userSchema).min(2, 'Au moins 2 utilisateurs sont requis'),
	projectDescription: z.string().max(200).optional(),
});
