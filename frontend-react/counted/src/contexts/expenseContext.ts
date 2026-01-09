import { createContext, type Dispatch, type SetStateAction } from 'react';
import type { Expense } from '../types/expenses.model';

export const ExpenseContext = createContext<ExpenseContextProperties>({ expense: undefined, setExpense: () => {} });

export interface ExpenseContextProperties {
	expense: Expense | undefined;
	setExpense: Dispatch<SetStateAction<Expense | undefined>>;
}
