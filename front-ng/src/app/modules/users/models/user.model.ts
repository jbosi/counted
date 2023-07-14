export interface IUser {
	id: number,
	name: string,
	balance: number | null,
	created_at: string | null
}

export interface ICreatableUser {
	name: string
}

export type ICreatableUsers = ICreatableUser[]