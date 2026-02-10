import { useContext, useRef, useState, type Dispatch, type RefObject, type SetStateAction } from 'react';
import { useNavigate, useParams } from 'react-router';
import { AppHeader } from '../../components/appHeader';
import { Avatar } from '../../components/avatar';
import { Loading } from '../../components/loading';
import { EditExpenseModal } from '../../components/modals/expense/editExpenseModal';
import { ProjectUsersContext } from '../../contexts/projectUsersContext';
import { useDeleteExpense, useExpense } from '../../hooks/useExpenses';
import { usePaymentsByExpenseId } from '../../hooks/usePayments';
import type { Expense } from '../../types/expenses.model';
import type { PaymentViewModel } from '../../types/payments.model';
import type { User } from '../../types/users.model';

export function PaymentPage() {
	const { expenseId, projectId } = useParams<string>();
	const { data: expense } = useExpense(parseInt(expenseId ?? '0', 10)); // TODO: handle error

	const { projectUsers } = useContext(ProjectUsersContext);
	const { mutate } = useDeleteExpense(projectId ?? '');
	const navigate = useNavigate();
	const [isModalOpen, setIsModalOpen] = useState(false);

	const expenseDialogRef = useRef<HTMLDialogElement>(null);

	const onDeleteExpense = () => {
		if (expense === undefined) {
			return;
		}

		mutate(expense?.id);
		navigate('..');
	};

	const openModal = () => {
		setIsModalOpen(true);
		setTimeout(() => {
			expenseDialogRef.current?.showModal();
		}, 100);
	};

	return (
		<div className="container overflow-auto app-container p-4 max-w-md">
			<AppHeader title={expense?.name} backButtonRoute=".." onDelete={onDeleteExpense} onEdit={() => openModal()} />
			<div className="container p-4 max-w-md rounded-xl flex flex-col">
				{expense == null || projectUsers == null ? (
					<Loading />
				) : (
					<PaymentList expense={expense} projectUsers={projectUsers} expenseDialogRef={expenseDialogRef} isModalOpen={isModalOpen} setIsModalOpen={setIsModalOpen} />
				)}
			</div>
		</div>
	);
}

interface PaymentListProps {
	expense: Expense;
	projectUsers: User[];
	expenseDialogRef: RefObject<HTMLDialogElement | null>;
	isModalOpen: boolean;
	setIsModalOpen: Dispatch<SetStateAction<boolean>>;
}

function PaymentList({ expense, projectUsers, expenseDialogRef, isModalOpen, setIsModalOpen }: PaymentListProps) {
	const { data: payments } = usePaymentsByExpenseId(expense.id);
	const paymentsViewModel: PaymentViewModel[] = (payments ?? []).map((p) => ({
		amount: p.amount,
		createdAt: p.createdAt,
		expenseId: p.expenseId,
		id: p.id,
		isDebt: p.isDebt,
		user: projectUsers?.find((pu) => pu.id === p.userId),
	}));

	const closeModal = () => {
		setIsModalOpen(false);
		expenseDialogRef.current?.close();
	};

	const payers = paymentsViewModel.filter((p) => !p.isDebt);
	const debtors = paymentsViewModel.filter((p) => p.isDebt);

	return (
		<>
			<section className="flex flex-col gap-3">
				<div>
					<h2 className="text-left">Répartition du paiement</h2>
					{payers?.map((payment) => (
						<PaymentItem payment={payment} key={payment.id} />
					))}
				</div>
				<div>
					<h2 className="text-left">Répartition de la dette</h2>
					{debtors?.map((payment) => (
						<PaymentItem payment={payment} key={payment.id} />
					))}
				</div>
			</section>
			{isModalOpen && (
				<EditExpenseModal
					dialogRef={expenseDialogRef}
					modalId={'EditExpenseModal'}
					expense={expense}
					projectId={expense.projectId}
					users={projectUsers}
					payments={payments ?? []}
					closeDialogFn={closeModal}
				/>
			)}
		</>
	);
}

interface PaymentItemProps {
	payment: PaymentViewModel;
}

function PaymentItem({ payment }: PaymentItemProps) {
	return (
		<div className="flex items-center gap-4 p-3 hover:bg-base-200 rounded-lg transition-colors">
			{/* Name */}
			<div className="flex-1 min-w-0 flex-row flex items-center gap-3">
				<Avatar name={payment.user?.name ?? '?'} />
				<p className="font-semibold text-base-content truncate">
					{payment.user?.name ?? '?'}
					{payment.isDebt ? ' doit' : ' a payé'}
				</p>
			</div>
			{/* Amount */}
			<div className="text-right">
				<p className="font-bold text-lg text-base-content">{payment.amount} €</p>
			</div>
		</div>
	);
}
