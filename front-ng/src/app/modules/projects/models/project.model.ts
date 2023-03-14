export interface IProject {
	id: number,
	name: string,
	created_at: string,
	currency: string,
	users: number[] // TODO userIds
}

export interface ICreatableProject {
	name: string,
	users: number[],
}