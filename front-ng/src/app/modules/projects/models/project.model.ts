export interface IProject {
	id: number,
	name: string,
	created_at: string,
	currency: string,
}

export interface ICreatableProject {
	name: string,
	users: number[],
}