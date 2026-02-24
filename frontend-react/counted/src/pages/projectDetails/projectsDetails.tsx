import { useCallback, useContext, useEffect, useMemo, useRef, useState } from 'react';
import { useLoaderData, useLocation } from 'react-router';
import { AppHeader } from '../../components/appHeader';
import { Loading } from '../../components/loading';
import { AddExpenseModal } from '../../components/modals/expense/addExpenseModal';
import type { AddExpenseModalForm } from '../../components/modals/expense/addExpenseModal';
import { EditProjectModal } from '../../components/modals/project/editProjectModal';
import { UserSelectionDialog } from '../../components/modals/user/userSelectionDialog';
import { SummaryCard } from '../../components/summaryCard';
import { CountedLocalStorageContext } from '../../contexts/localStorageContext';
import { ProjectUsersContext } from '../../contexts/projectUsersContext';
import { useExpensesByProjectId, useExpenseSummary } from '../../hooks/useExpenses';
import { usePaymentsByProjectId } from '../../hooks/usePayments';
import { useDeleteProject, useProject, useUpdateProjectStatus } from '../../hooks/useProjects';
import type { ReimbursementSuggestion } from '../../types/summary.model';
import { getProjectUserIdFromLocalstorage } from '../../utils/get-project-from-localstorage';
import { openDialog } from '../../utils/open-dialog';
import { ExpensesUserSection } from './components/expensesUserSection';
import { ExpenseDropdownSettings } from './components/expenseDropdownSettings';
import { ExpenseList } from './components/expenseList';
import { ProjectSummary } from './components/projectSummary';
import { ReimbursementSuggestions } from './components/reimbursementSuggestions';
import { DropdownAction } from '../../components/dropdowns/dropdownAction';
import { BurgerIcon } from '../../shared/icons/burgerIcon';
import { Dropdown } from '../../components/dropdowns/dropdown';
import { getPickerFormattedDate } from '../../utils/date';

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
	const { mutate: deleteProject } = useDeleteProject();
	const { mutate: updateStatus } = useUpdateProjectStatus(projectId);
	const location = useLocation();

	const { countedLocalStorage } = useContext(CountedLocalStorageContext);
	const storedUserId = getProjectUserIdFromLocalstorage(countedLocalStorage, projectId);

	const projectStatus = project.data?.status ?? 'ongoing';

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
	const [expenseInitialValues, setExpenseInitialValues] = useState<Partial<AddExpenseModalForm> | undefined>(undefined);

	const closeProjectDialog = () => {
		setIsProjectDialogOpen(false);
		projectDialogRef.current?.close();
	};

	const closeExpenseDialog = () => {
		setIsExpenseDialogOpen(false);
		setExpenseInitialValues(undefined);
		expenseDialogRef.current?.close();
	};

	const openReimbursementExpense = useCallback(
		(suggestion: ReimbursementSuggestion) => {
			const debtor = users!.find((u) => u.id === suggestion.userIdDebtor)!;
			const payer = users!.find((u) => u.id === suggestion.userIdPayer)!;

			setExpenseInitialValues({
				type: 'Transfer',
				name: `Remboursement ${debtor.name} vers ${payer.name}`,
				totalAmount: suggestion.amount,
				date: getPickerFormattedDate(new Date()),
				payers: users!.map((u) => ({
					amount: u.id === debtor.id ? suggestion.amount : 0,
					shares: u.id === debtor.id ? 1 : 0,
					isChecked: u.id === debtor.id,
					user: u,
				})),
				debtors: users!.map((u) => ({
					amount: u.id === payer.id ? suggestion.amount : 0,
					shares: u.id === payer.id ? 1 : 0,
					isChecked: u.id === payer.id,
					user: u,
				})),
			});
			openDialog(setIsExpenseDialogOpen, expenseDialogRef);
		},
		[users, expenseDialogRef],
	);

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
		<div className="overflow-auto app-container p-4 max-w-md">
			{project.data ? (
				<>
					<AppHeader title={project.data?.name ?? ''} backButtonRoute="..">
						<Dropdown id="AppHeaderId" icon={<BurgerIcon />}>
							<DropdownAction onEdit={() => openDialog(setIsProjectDialogOpen, projectDialogRef)} onDelete={() => deleteProject(projectId)} />
							{projectStatus === 'ongoing' && (
								<>
									<li>
										<button type="button" className="btn btn-warning btn-soft" onClick={() => updateStatus('closed')}>
											Cloturer
										</button>
									</li>
									<li>
										<button type="button" className="btn btn-neutral btn-soft" onClick={() => updateStatus('archived')}>
											Archiver
										</button>
									</li>
								</>
							)}
							{projectStatus === 'closed' && (
								<>
									<li>
										<button type="button" className="btn btn-success btn-soft" onClick={() => updateStatus('ongoing')}>
											Réouvrir
										</button>
									</li>
									<li>
										<button type="button" className="btn btn-neutral btn-soft" onClick={() => updateStatus('archived')}>
											Archiver
										</button>
									</li>
								</>
							)}
							{projectStatus === 'archived' && (
								<li>
									<button type="button" className="btn btn-success btn-soft" onClick={() => updateStatus('ongoing')}>
										Réouvrir
									</button>
								</li>
							)}
							<li>
								<button
									type="button"
									className="btn btn-ghost"
									onClick={async () => {
										const clipboardItem = new ClipboardItem({ 'text/plain': `https://counted.fr${location.pathname}` });
										await navigator.clipboard.write([clipboardItem]);
									}}
								>
									Copier le lien
								</button>
							</li>
						</Dropdown>
					</AppHeader>
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
					{isExpenseDialogOpen && (
						<AddExpenseModal
							modalId={'addExpenseModal'}
							projectId={projectId}
							users={users ?? []}
							dialogRef={expenseDialogRef}
							closeDialogFn={closeExpenseDialog}
							restrictToTransfer={projectStatus === 'closed'}
							initialValues={expenseInitialValues}
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
					<ExpensesUserSection users={users ?? []} />

					<SummaryCard projectId={projectId} globalTotal={globalTotal()} />

					<div role="tablist" className="tabs tabs-box justify-between bg-base-300">
						<a role="tab" className={`tab text-xs ${activeTab === 'ExpensesList' ? 'tab-active' : ''}`} onClick={() => setActiveTab('ExpensesList')}>
							Dépenses
						</a>

						<a role="tab" className={`tab text-xs ${activeTab === 'Summary' ? 'tab-active' : ''}`} onClick={() => setActiveTab('Summary')}>
							Equilibre
						</a>

						<a
							role="tab"
							className={`tab text-xs ${activeTab === 'ReimbursementSuggestions' ? 'tab-active' : ''}`}
							onClick={() => setActiveTab('ReimbursementSuggestions')}
						>
							Remboursements
						</a>
					</div>

					{activeTab === 'ExpensesList' ? (
						<>
							<ExpenseDropdownSettings showMyDebtsState={[showMyDebts, setShowMyDebts]} showMyPaymentsState={[showMyPayments, setShowMyPayments]} />
							<ExpenseList expenses={filteredExpenses} />

							{(users?.length ?? 0) > 0 && projectStatus !== 'archived' && (
								<button
									type="button"
									className="btn btn-circle btn-lg sticky bottom-0 self-center mt-6 btn-soft"
									onClick={() => openDialog(setIsExpenseDialogOpen, expenseDialogRef)}
								>
									+
								</button>
							)}
						</>
					) : activeTab === 'ReimbursementSuggestions' ? (
						<ReimbursementSuggestions
							reimbursementSuggestions={projectSummary.data?.reimbursementSuggestions}
							users={users}
							onReimburse={projectStatus !== 'archived' ? openReimbursementExpense : undefined}
						/>
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
