export interface Expense {
	id: number;
	author_id: number;
	project_id: string;
	created_at: string;
	amount: number;
	description?: string;
	name: string;
	expense_type: ExpenseType;
}

export type ExpenseType = 'Expense' | 'Transfer' | 'Gain';
