import { IBalanceDto, UserBalanceDto } from "@hcount/modules";

export interface IBalanceViewModel extends IBalanceDto {
	balances: IUserBalanceViewModel[];
}

export interface IUserBalanceViewModel extends UserBalanceDto {
	/**
	 * From 0 to 100
	 */
	amountProgressBar: number
}