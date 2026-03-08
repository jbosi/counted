export type ProjectStatus = 'ongoing' | 'closed' | 'archived';

export interface ProjectDto {
	id: string;
	name: string;
	createdAt: string;
	currency: string;
	description: string;
	status: ProjectStatus;
}

export interface CreatableProject {
	name: string;
	description?: string;
	currency?: string;
}

export interface EditableProject extends CreatableProject {
	id: string;
	status?: ProjectStatus;
}
