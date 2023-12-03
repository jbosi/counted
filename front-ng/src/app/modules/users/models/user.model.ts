export interface IUser {
	id: number,
	name: string,
	balance: number | null,
	created_at: string | null
}

export interface ICreatableUser {
	name: string,
	project_id?: string;
}

export type ICreatableUsers = ICreatableUser[]