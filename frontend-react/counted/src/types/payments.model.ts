import type { User } from './users.model';

export interface PaymentViewModel {
	id: number;
	expenseId: number;
	user: User | undefined;
	isDebt: boolean;
	amount: number;
	createdAt: string;
}

export interface Payment {
	id: number;
	expenseId: number;
	userId: number;
	isDebt: boolean;
	amount: number;
	createdAt: string;
}
