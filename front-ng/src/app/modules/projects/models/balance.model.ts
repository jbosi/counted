export interface IBalanceDto {
	balances: UserBalanceDto[],
	currency: string,
	total_expenses: number,
}

export interface UserBalanceDto {
	amount: number,
	user_id: number,
	user_name: string,
}