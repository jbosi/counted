import type { User } from './users.model';

export interface ProjectSummary {
	reimbursementSuggestions: ReimbursementSuggestion[];
	summary: Record<User['id'], number>;
}

export interface ReimbursementSuggestion {
	amount: number;
	userIdDebtor: number;
	userIdPayer: number;
}
