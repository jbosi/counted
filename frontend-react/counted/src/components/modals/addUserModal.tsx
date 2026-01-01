import { useState, type RefObject } from 'react';
import { useForm, type SubmitHandler } from 'react-hook-form';
import { useAddUser, useDeleteUser } from '../../hooks/useUsers';
import * as z from 'zod';
import { ErrorValidationCallout } from '../errorCallout';
import type { User } from '../../types/users.model';

export interface AddUserModalProps {
	modalId: string;
	dialogRef: RefObject<HTMLDialogElement | null>;
	projectId: string;
	currentUsers: User[];
}

const formSchema = z.object({
	name: z.string().min(2).max(100),
});

interface AddUserModalForm {
	name: string;
}

export function AddUserModal({ dialogRef, modalId, projectId, currentUsers }: AddUserModalProps) {
	const [addUserErrorState, addUserSetErrorState] = useState<string | null>(null);
	const [deleteUserErrorState, deleteUserSetErrorState] = useState<string | null>(null);
	const {
		register,
		formState: { errors },
		getValues,
	} = useForm<AddUserModalForm>();
	const { error: addUserError, isPending: addUserIsPending, isError: addUserIsError, mutate: addUserMutate } = useAddUser();
	const { error: deleteUserError, isPending: deleteUserIsPending, isError: deleteUserIsError, mutate: deleteUserMutate } = useDeleteUser();

	const onAddUser: SubmitHandler<AddUserModalForm> = (data) => {
		const parsedResult = formSchema.safeParse(data);

		if (parsedResult.error) {
			addUserSetErrorState(parsedResult.error.message);
			return;
		}

		addUserMutate({ name: data.name, projectId });
		dialogRef.current?.close();
	};

	const onDeleteUser = (userId: number) => {
		deleteUserMutate(userId);
	};

	return (
		<>
			<dialog ref={dialogRef} id={modalId} className="modal">
				<div className="modal-box flex gap-3 flex-col">
					<button type="button" className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" onClick={() => dialogRef.current?.close()}>
						✕
					</button>
					<h1>Ajouter un Utilisateur</h1>
					<ErrorValidationCallout errorState={addUserErrorState} /> {/* TODO, use error boundary ? */}
					<form
						className="flex flex-col gap-4 ml-4 mr-4"
						onSubmit={(e) => {
							e.preventDefault();
							onAddUser(getValues());
						}}
					>
						<div className="flex flex-col gap-3">
							<label className="label">Nom</label>
							<input className="input w-full" {...register('name', { required: true, maxLength: 100 })} />
							{errors.name && <span>Ce champ est requis</span>}

							{addUserIsPending && <span>Enregistrement…</span>}
							{addUserIsError && <span className="text-error">{(addUserError as Error).message}</span>}
						</div>
						{deleteUserErrorState && <span className="text-error">{(deleteUserError as Error).message}</span>}
						{currentUsers?.map((u) => {
							return (
								<div className="flex gap-3">
									<span className="self-center">{u.name}</span>
									<button type="button" className="btn btn-square btn-sm p-1.5 btn-soft" onClick={() => onDeleteUser(u.id)}>
										<svg
											xmlns="http://www.w3.org/2000/svg"
											width="24"
											height="24"
											viewBox="0 0 24 24"
											fill="none"
											stroke="currentColor"
											stroke-width="2"
											stroke-linecap="round"
											stroke-linejoin="round"
											className="icon icon-tabler icons-tabler-outline icon-tabler-trash"
										>
											<path stroke="none" d="M0 0h24v24H0z" fill="none" />
											<path d="M4 7l16 0" />
											<path d="M10 11l0 6" />
											<path d="M14 11l0 6" />
											<path d="M5 7l1 12a2 2 0 0 0 2 2h8a2 2 0 0 0 2 -2l1 -12" />
											<path d="M9 7v-3a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v3" />
										</svg>
									</button>
								</div>
							);
						})}
						<footer className="flex gap-1.5 mt-12 justify-end">
							<button className="btn btn-primary" type="submit">
								Enregistrer
							</button>
							<button className="btn btn-outline" type="button" onClick={() => dialogRef.current?.close()}>
								Annuler
							</button>
						</footer>
					</form>
				</div>
			</dialog>
		</>
	);
}
