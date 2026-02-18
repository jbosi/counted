import * as z from 'zod';

const userSchema = z.object({
	name: z.string().min(2, 'Le nom doit contenir au moins 2 caractères').max(100, 'Le nom ne doit pas dépasser 100 caractères'),
	userId: z.number().optional(),
});

export const PROJECT_FORM_SCHEMA = z.object({
	projectName: z.string().min(2, 'Le nom doit contenir au moins 2 caractères').max(100, 'Le nom ne doit pas dépasser 100 caractères'),
	users: z.array(userSchema).min(2, 'Au moins 2 utilisateurs sont requis'),
	projectDescription: z.string().max(200, 'La description ne doit pas dépasser 200 caractères').optional(),
});
