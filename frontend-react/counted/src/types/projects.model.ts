export interface ProjectDto {
	id: string;
	name: string;
	created_at: string;
	currency: string;
	description: string;
}

export interface CreatableProject {
	name: string;
	description?: string;
	currency?: string;
}
