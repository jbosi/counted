export interface Expense {
	id: number;
	authorId: number;
	projectId: string;
	createdAt: string;
	amount: number;
	description?: string;
	name: string;
	expenseType: ExpenseType;
}

export const ExpenseTypeConst = ['Expense', 'Transfer', 'Gain'] as const;
export type ExpenseType = (typeof ExpenseTypeConst)[number];

export interface CreatableExpense {
	name: string;
	amount: number;
	expenseType: ExpenseType;
	projectId: string;
	payers: UserAmount[];
	debtors: UserAmount[];
	authorId: number;
	description?: string;
}

export interface EditableExpense extends CreatableExpense {
	id: number;
}

export interface UserAmount {
	userId: number;
	amount: number;
}
