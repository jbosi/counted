import { inject, Injectable } from '@angular/core';
import { IBalanceDto, ProjectsHttpClient } from '@hcount/modules';
import { IBalanceViewModel, IUserBalanceViewModel } from '../../models';

@Injectable({providedIn: 'root'})
export class BalanceApplication {
	private readonly projectsHttpClient = inject(ProjectsHttpClient);

	public async getBalanceAsync(projectId: string): Promise<IBalanceViewModel | null> {
		const balance: IBalanceDto | null = await this.projectsHttpClient.getBalanceAsync(projectId)
			.catch(() => null);

		return forgeBalanceViewModel(balance);
	}

}

const forgeBalanceViewModel = (balance: IBalanceDto | null): IBalanceViewModel | null => {
	if (balance == null) {
		return null;
	}

	const balanceNumbers: number[] = balance.balances.map(b => b.amount);
	const maxBalance: number = Math.max(...balanceNumbers);
	const minBalance: number = Math.min(...balanceNumbers);

	const range = maxBalance - minBalance;

	const balances: IUserBalanceViewModel[] = balance.balances.map(b => ({
		amount: b.amount,
		user_id: b.user_id,
		user_name: b.user_name,
		amountProgressBar: ((b.amount + Math.abs(range / 2)) / range) * 100
	}))

	console.log(balances)

	return {
		balances,
		currency: balance.currency,
		total_expenses: balance.total_expenses
	}
}