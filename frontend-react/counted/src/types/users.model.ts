export interface User {
	id: number;
	name: string;
	balance?: number | null;
	created_at?: string | null;
}

export interface CreatableUser extends Pick<User, 'name'> {
	projectId: string;
}
