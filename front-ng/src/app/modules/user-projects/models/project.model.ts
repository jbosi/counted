export interface IUserProjectDto {
	id: number,
	name: string,
	created_at: string,
	currency: string,
	users: number[]
}

export interface ICreatableUserProject {
	name: string,
	users: number[],
}