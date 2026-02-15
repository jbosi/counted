import { useCallback, useContext, useEffect, useMemo, useRef, useState } from 'react';
import { useLoaderData } from 'react-router';
import { AppHeader } from '../../components/appHeader';
import { ExpenseList } from '../../components/expenseList';
import { Loading } from '../../components/loading';
import { AddExpenseModal } from '../../components/modals/expense/addExpenseModal';
import { EditProjectModal } from '../../components/modals/project/editProjectModal';
import { UserSelectionDialog } from '../../components/modals/user/userSelectionDialog';
import { SummaryCard } from '../../components/summaryCard';
import { CountedLocalStorageContext } from '../../contexts/localStorageContext';
import { ProjectUsersContext } from '../../contexts/projectUsersContext';
import { useExpensesByProjectId, useExpenseSummary } from '../../hooks/useExpenses';
import { usePaymentsByProjectId } from '../../hooks/usePayments';
import { useProject } from '../../hooks/useProjects';
import { SettingsIcon } from '../../shared/icons/settingsIcon';
import type { ProjectSummary } from '../../types/summary.model';
import type { User } from '../../types/users.model';
import { getProjectUserIdFromLocalstorage } from '../../utils/get-project-from-localstorage';
import { openDialog } from '../../utils/open-dialog';
import { ExpenseBarChartComponent } from '../expenses/expensesBarChart';
import { ExpensesUserSection } from '../expenses/expensesUserSection';
import { ReimbursementSuggestions } from './reimbursementSuggestions';

interface ProjectDetailsProps {
	projectId: string;
}

type ActiveTab = 'ExpensesList' | 'Summary' | 'ReimbursementSuggestions';

export const ProjectDetails = () => {
	const { projectId }: ProjectDetailsProps = useLoaderData();
	const project = useProject(projectId);
	const { projectUsers: users } = useContext(ProjectUsersContext);
	const { data: expenses } = useExpensesByProjectId(projectId);
	const { data: payments } = usePaymentsByProjectId(projectId);
	const projectSummary = useExpenseSummary(projectId);

	const { countedLocalStorage } = useContext(CountedLocalStorageContext);
	const storedUserId = getProjectUserIdFromLocalstorage(countedLocalStorage, projectId);

	const [showMyPayments, setShowMyPayments] = useState(false);
	const [showMyDebts, setShowMyDebts] = useState(false);

	const filteredExpenses = useMemo(() => {
		if ((!showMyPayments && !showMyDebts) || !payments || storedUserId == null) {
			return expenses ?? [];
		}
		const myExpenseIds = new Set(
			payments.filter((p) => p.userId === storedUserId && ((showMyPayments && !p.isDebt) || (showMyDebts && p.isDebt))).map((p) => p.expenseId),
		);
		return (expenses ?? []).filter((e) => myExpenseIds.has(e.id));
	}, [expenses, payments, showMyPayments, showMyDebts, storedUserId]);

	const expenseDialogRef = useRef<HTMLDialogElement>(null);
	const projectDialogRef = useRef<HTMLDialogElement>(null);
	const userSelectionDialogRef = useRef<HTMLDialogElement>(null);

	const [activeTab, setActiveTab] = useState<ActiveTab>('ExpensesList');

	const [isProjectDialogOpen, setIsProjectDialogOpen] = useState(false);
	const [isExpenseDialogOpen, setIsExpenseDialogOpen] = useState(false);
	const [isUserSelectionDialogOpen, setIsUserselectionDialogOpen] = useState(false);

	const closeProjectDialog = () => {
		setIsProjectDialogOpen(false);
		projectDialogRef.current?.close();
	};

	const closeExpenseDialog = () => {
		setIsExpenseDialogOpen(false);
		expenseDialogRef.current?.close();
	};

	const closeUserSelectionDialog = () => {
		setIsUserselectionDialogOpen(false);
		userSelectionDialogRef.current?.close();
	};

	useEffect(() => {
		if (storedUserId == null) {
			openDialog(setIsUserselectionDialogOpen, userSelectionDialogRef, 400);
		}
	}, []);

	// Sum expenses - sum gains
	const globalTotal = useCallback(
		() => expenses?.filter((e) => e.expenseType !== 'Transfer')?.reduce((acc, e) => (e.expenseType === 'Expense' ? acc + e.amount : acc - e.amount), 0) ?? 0,
		[expenses],
	);

	return (
		<div className="container overflow-auto app-container p-4 max-w-md">
			{project.data ? (
				<>
					<AppHeader onEdit={() => openDialog(setIsProjectDialogOpen, projectDialogRef)} title={project.data?.name ?? ''} backButtonRoute=".." />
					{isProjectDialogOpen && (
						<EditProjectModal dialogRef={projectDialogRef} modalId={'EditProjectModal'} project={project.data} users={users ?? []} closeDialogFn={closeProjectDialog} />
					)}
					{isUserSelectionDialogOpen && (users ?? [])?.length > 0 && (
						<UserSelectionDialog
							modalId={'userSelectionDialog'}
							projectId={projectId}
							users={users ?? []}
							dialogRef={userSelectionDialogRef}
							closeDialogFn={closeUserSelectionDialog}
						/>
					)}
				</>
			) : (
				<div className="flex justify-center">
					<Loading />
				</div>
			)}

			{users && expenses ? (
				<>
					<ExpensesUserSection id={projectId} users={users ?? []} />

					<SummaryCard projectId={projectId} globalTotal={globalTotal()} />

					<div role="tablist" className="tabs tabs-box justify-center">
						<a role="tab" className={`tab ${activeTab === 'ExpensesList' ? 'tab-active' : ''}`} onClick={() => setActiveTab('ExpensesList')}>
							Dépenses
						</a>

						<a role="tab" className={`tab ${activeTab === 'Summary' ? 'tab-active' : ''}`} onClick={() => setActiveTab('Summary')}>
							Equilibre
						</a>

						<a role="tab" className={`tab ${activeTab === 'ReimbursementSuggestions' ? 'tab-active' : ''}`} onClick={() => setActiveTab('ReimbursementSuggestions')}>
							Remboursements
						</a>
					</div>

					{activeTab === 'ExpensesList' ? (
						<>
							<div className="dropdown dropdown-end self-end">
								<div tabIndex={0} role="button" className="btn btn-ghost btn-sm btn-circle">
									<SettingsIcon />
								</div>
								<div tabIndex={0} className="dropdown-content bg-base-200 rounded-box z-10 w-52 p-3 shadow flex flex-col gap-2">
									<label className="label cursor-pointer justify-between gap-2">
										<span className="text-sm">Mes paiements</span>
										<input type="checkbox" className="toggle toggle-sm" checked={showMyPayments} onChange={(e) => setShowMyPayments(e.target.checked)} />
									</label>
									<label className="label cursor-pointer justify-between gap-2">
										<span className="text-sm">Mes dettes</span>
										<input type="checkbox" className="toggle toggle-sm" checked={showMyDebts} onChange={(e) => setShowMyDebts(e.target.checked)} />
									</label>
								</div>
							</div>
							<ExpenseList expenses={filteredExpenses} />

							{(users?.length ?? 0) > 0 && (
								<>
									<button
										type="button"
										className="btn btn-circle btn-lg sticky bottom-0 self-center mt-6 btn-soft"
										onClick={() => openDialog(setIsExpenseDialogOpen, expenseDialogRef)}
									>
										+
									</button>

									{isExpenseDialogOpen && (
										<AddExpenseModal
											modalId={'addExpenseModal'}
											projectId={projectId}
											users={users ?? []}
											dialogRef={expenseDialogRef}
											closeDialogFn={closeExpenseDialog}
										/>
									)}
								</>
							)}
						</>
					) : activeTab === 'ReimbursementSuggestions' ? (
						<>
							<ReimbursementSuggestions reimbursementSuggestions={projectSummary.data?.reimbursementSuggestions} users={users} />
						</>
					) : (
						<ProjectSummary projectSummary={projectSummary.data} users={users} />
					)}
				</>
			) : (
				<div className="flex justify-center mt-20">
					<Loading />
				</div>
			)}
		</div>
	);
};

interface ProjectSummaryProps {
	projectSummary: ProjectSummary | undefined;
	users: User[];
}

function ProjectSummary({ projectSummary, users }: ProjectSummaryProps) {
	if (projectSummary === undefined) {
		return <></>;
	}

	const summary = projectSummary.summary;

	const usersWithoutExpense: User[] = users.filter((u) => !Object.entries(summary)?.some((s) => u.id === Number(s?.[0])));

	return (() => {
		const maxAmount = Math.max(...Object.values(summary).map((v) => Math.abs(v)), 1);

		return (
			<>
				<ul className="flex flex-col gap-3">
					{Object.entries(summary)
						.sort(([_, amount1], [__, amount2]) => amount1 - amount2)
						.map(([userIdStr, amount]) => {
							const userId = Number(userIdStr);
							const user = users?.find((u) => u.id === userId);
							if (!user) {
								return null;
							}

							return <ExpenseBarChartComponent key={userId} user={user} summaryAmount={amount} maxAmount={maxAmount} />;
						})}
					{usersWithoutExpense.length > 0 &&
						usersWithoutExpense.map((user) => <ExpenseBarChartComponent key={user.id} user={user} summaryAmount={0} maxAmount={maxAmount} />)}
				</ul>
			</>
		);
	})();
}
