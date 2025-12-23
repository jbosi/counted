import { useRef, useState, type RefObject } from 'react';
import { useLoaderData } from 'react-router';
import { AppHeader } from '../../components/appHeader';
import { SummaryCard } from '../../components/summaryCard';
import { useExpensesByProjectId, useExpenseSummary } from '../../hooks/useExpenses';
import { useProject } from '../../hooks/useProjects';
import { useUsersByProjectId } from '../../hooks/useUsers';
import { ExpensesUserSection } from '../expenses/expensesUserSection';
import { ExpenseList } from '../../components/expenseList';
import { ExpenseBarChartComponent } from '../expenses/expensesBarChart';
import { AddExpenseModal } from '../../components/modals/addExpenseModal';

interface ProjectDetailsProps {
	projectId: string;
}

type ActiveTab = 'ExpensesList' | 'Summary';

export const ProjectDetails = () => {
	const props: ProjectDetailsProps = useLoaderData();
	const project = useProject(props.projectId);
	const users = useUsersByProjectId(props.projectId);
	const expenses = useExpensesByProjectId(props.projectId);
	const summary = useExpenseSummary(props.projectId);
	const dialogRef = useRef<HTMLDialogElement>(null);

	const [activeTab, setActiveTab] = useState<ActiveTab>('ExpensesList');
	const [expenseModalOpen, setExpenseModalOpen] = useState(false);

	const globalTotal = expenses?.data?.reduce((acc, e) => acc + e.amount, 0) ?? 0;

	return (
		<div className="container overflow-auto app-container w-96 bg-base-200 p-4 max-w-md rounded-xl flex flex-col">
			{project ? (
				<AppHeader title={project.data?.name ?? ''} backButtonRoute="/projects" />
			) : (
				<div className="flex justify-center">
					<span className="loading loading-spinner loading-m" />
				</div>
			)}

			{users && expenses ? (
				<>
					<ExpensesUserSection id={props.projectId} users={users.data ?? []} />

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

							{(users.data?.length ?? 0) > 0 && (
								<>
									<button
										type="button"
										className="btn btn-circle btn-outline btn-lg sticky bottom-0 self-center mt-6"
										onClick={() => (dialogRef as RefObject<HTMLDialogElement>).current.showModal()}
									>
										+
									</button>

									<AddExpenseModal modalId={'addExpenseModal'} projectId={props.projectId} users={users.data ?? []} dialogRef={dialogRef} />
								</>
							)}
						</>
					) : (
						<>
							{summary.data ? (
								(() => {
									const maxAmount = Math.max(...Object.values(summary.data).map((v) => Math.abs(v)), 1);

									return (
										<section className="flex flex-col gap-2">
											{Object.entries(summary.data).map(([userIdStr, amount]) => {
												const userId = Number(userIdStr);
												const user = users.data?.find((u) => u.id === userId);
												if (!user) return null;

												return <ExpenseBarChartComponent key={userId} user={user} summaryAmount={amount} maxAmount={maxAmount} />;
											})}
										</section>
									);
								})()
							) : (
								<></>
							)}
						</>
					)}
				</>
			) : (
				<div className="flex justify-center mt-20">
					<span className="loading loading-spinner loading-xl" />
				</div>
			)}
		</div>
	);
};
