export interface User {
	id: number;
	name: string;
	balance?: number;
	created_at?: string;
}

export interface CreatableUser extends Pick<User, 'name'> {
	projectId: string;
}
