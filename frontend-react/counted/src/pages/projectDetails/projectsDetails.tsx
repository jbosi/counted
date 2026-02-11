import { useCallback, useContext, useRef, useState } from 'react';
import { useLoaderData } from 'react-router';
import { AppHeader } from '../../components/appHeader';
import { ExpenseList } from '../../components/expenseList';
import { Loading } from '../../components/loading';
import { AddExpenseModal } from '../../components/modals/expense/addExpenseModal';
import { EditProjectModal } from '../../components/modals/project/editProjectModal';
import { SummaryCard } from '../../components/summaryCard';
import { CountedLocalStorageContext } from '../../contexts/localStorageContext';
import { ProjectUsersContext } from '../../contexts/projectUsersContext';
import { useExpensesByProjectId, useExpenseSummary } from '../../hooks/useExpenses';
import { useAddToLocalStorage } from '../../hooks/useLocalStorage';
import { useProject } from '../../hooks/useProjects';
import type { ProjectSummary } from '../../types/summary.model';
import type { User } from '../../types/users.model';
import { ExpenseBarChartComponent } from '../expenses/expensesBarChart';
import { ExpensesUserSection } from '../expenses/expensesUserSection';
import { ReimbursementSuggestions } from './reimbursementSuggestions';

interface ProjectDetailsProps {
	projectId: string;
}

type ActiveTab = 'ExpensesList' | 'Summary' | 'ReimbursementSuggestions';

export const ProjectDetails = () => {
	const props: ProjectDetailsProps = useLoaderData();
	const project = useProject(props.projectId);
	const { projectUsers: users } = useContext(ProjectUsersContext);
	const { data: expenses } = useExpensesByProjectId(props.projectId);
	const projectSummary = useExpenseSummary(props.projectId);

	const expenseDialogRef = useRef<HTMLDialogElement>(null);
	const projectDialogRef = useRef<HTMLDialogElement>(null);

	const [activeTab, setActiveTab] = useState<ActiveTab>('ExpensesList');

	const [isProjectDialogOpen, setIsProjectDialogOpen] = useState(false);
	const [isExpenseDialogOpen, setIsExpenseDialogOpen] = useState(false);

	const openProjectModal = () => {
		setIsProjectDialogOpen(true);
		setTimeout(() => {
			projectDialogRef.current?.showModal();
		}, 100);
	};

	const openExpenseModal = () => {
		setIsExpenseDialogOpen(true);
		setTimeout(() => {
			expenseDialogRef.current?.showModal();
		}, 100);
	};

	const closeProjectDialog = () => {
		setIsProjectDialogOpen(false);
		projectDialogRef.current?.close();
	};

	const closeExpenseDialog = () => {
		setIsExpenseDialogOpen(false);
		expenseDialogRef.current?.close();
	};

	// Sum expenses - sum gains
	const globalTotal = useCallback(
		() => expenses?.filter((e) => e.expenseType !== 'Transfer')?.reduce((acc, e) => (e.expenseType === 'Expense' ? acc + e.amount : acc - e.amount), 0) ?? 0,
		[expenses],
	);

	const { countedLocalStorage, setCountedLocalStorage } = useContext(CountedLocalStorageContext);

	useAddToLocalStorage(countedLocalStorage, { projectId: props.projectId, userId: null }, setCountedLocalStorage);

	return (
		<div className="container overflow-auto app-container p-4 max-w-md">
			{project.data ? (
				<>
					<AppHeader onEdit={() => openProjectModal()} title={project.data?.name ?? ''} backButtonRoute=".." />
					{isProjectDialogOpen && (
						<EditProjectModal dialogRef={projectDialogRef} modalId={'EditProjectModal'} project={project.data} users={users ?? []} closeDialogFn={closeProjectDialog} />
					)}
				</>
			) : (
				<div className="flex justify-center">
					<Loading />
				</div>
			)}

			{users && expenses ? (
				<>
					<ExpensesUserSection id={props.projectId} users={users ?? []} />

					<SummaryCard users={users} projectId={props.projectId} globalTotal={globalTotal()} />

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
							<ExpenseList expenses={expenses ?? []} />

							{(users?.length ?? 0) > 0 && (
								<>
									<button type="button" className="btn btn-circle btn-lg sticky bottom-0 self-center mt-6 btn-soft" onClick={() => openExpenseModal()}>
										+
									</button>

									{isExpenseDialogOpen && (
										<AddExpenseModal
											modalId={'addExpenseModal'}
											projectId={props.projectId}
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
					{usersWithoutExpense.length > 0 ? (
						usersWithoutExpense.map((user) => <ExpenseBarChartComponent key={user.id} user={user} summaryAmount={0} maxAmount={maxAmount} />)
					) : (
						<></>
					)}
				</ul>
			</>
		);
	})();
}
