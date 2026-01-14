import { useContext, useRef, useState, type RefObject } from 'react';
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

interface ProjectDetailsProps {
	projectId: string;
}

type ActiveTab = 'ExpensesList' | 'Summary';

export const ProjectDetails = () => {
	const props: ProjectDetailsProps = useLoaderData();
	const project = useProject(props.projectId);
	const { projectUsers: users } = useContext(ProjectUsersContext);
	const expenses = useExpensesByProjectId(props.projectId);
	const summary = useExpenseSummary(props.projectId);
	const expenseDialogRef = useRef<HTMLDialogElement>(null);
	const projectDialogRef = useRef<HTMLDialogElement>(null);

	const [activeTab, setActiveTab] = useState<ActiveTab>('ExpensesList');

	const globalTotal = expenses?.data?.reduce((acc, e) => acc + e.amount, 0) ?? 0;

	const { countedLocalStorage, setCountedLocalStorage } = useContext(CountedLocalStorageContext);

	useAddToLocalStorage(countedLocalStorage, props.projectId, setCountedLocalStorage);

	return (
		<div className="container overflow-auto app-container w-96 bg-base-200 p-4 max-w-md rounded-xl flex flex-col">
			{project.data ? (
				<>
					<AppHeader onEdit={() => (projectDialogRef as RefObject<HTMLDialogElement>).current.showModal()} title={project.data?.name ?? ''} backButtonRoute=".." />
					<EditProjectModal dialogRef={projectDialogRef} modalId={'AddProjectModal'} project={project.data} />
				</>
			) : (
				<div className="flex justify-center">
					<Loading />
				</div>
			)}

			{users && expenses ? (
				<>
					<ExpensesUserSection id={props.projectId} users={users ?? []} />

					<SummaryCard myTotal={625.0} globalTotal={globalTotal} />

					<div role="tablist" className="tabs tabs-box justify-center">
						<a role="tab" className={`tab ${activeTab === 'ExpensesList' ? 'tab-active' : ''}`} onClick={() => setActiveTab('ExpensesList')}>
							Liste des dépenses
						</a>

						<a role="tab" className={`tab ${activeTab === 'Summary' ? 'tab-active' : ''}`} onClick={() => setActiveTab('Summary')}>
							Equilibre
						</a>
					</div>

					{activeTab === 'ExpensesList' ? (
						<>
							<div className="mt-6">
								<ExpenseList expenses={expenses.data ?? []} />
							</div>

							{(users?.length ?? 0) > 0 && (
								<>
									<button
										type="button"
										className="btn btn-circle btn-outline btn-lg sticky bottom-0 self-center mt-6"
										onClick={() => (expenseDialogRef as RefObject<HTMLDialogElement>).current.showModal()}
									>
										+
									</button>

									<AddExpenseModal modalId={'addExpenseModal'} projectId={props.projectId} users={users ?? []} dialogRef={expenseDialogRef} />
								</>
							)}
						</>
					) : (
						<ProjectSummary summary={summary.data} users={users} />
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
	summary: ProjectSummary | undefined;
	users: User[];
}

function ProjectSummary({ summary, users }: ProjectSummaryProps) {
	if (summary === undefined) {
		return <></>;
	}

	const usersWithoutExpense: User[] = users.filter((u) => !Object.entries(summary)?.some((s) => u.id === Number(s?.[0])));

	return (() => {
		const maxAmount = Math.max(...Object.values(summary).map((v) => Math.abs(v)), 1);

		return (
			<section className="flex flex-col gap-2">
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
			</section>
		);
	})();
}
