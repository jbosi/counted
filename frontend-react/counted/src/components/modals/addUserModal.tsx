import { type RefObject } from 'react';
import { useForm, type SubmitHandler } from 'react-hook-form';
import { useAddUser } from '../../hooks/useUsers';

export interface AddUserModalProps {
	modalId: string;
	dialogRef: RefObject<HTMLDialogElement | null>;
	projectId: string;
}

interface AddUserModalForm {
	name: string;
}

export function AddUserModal({ dialogRef, modalId, projectId }: AddUserModalProps) {
	const {
		register,
		formState: { errors },
		getValues,
	} = useForm<AddUserModalForm>();
	const { error, isPending, isError, mutate } = useAddUser();

	const onSubmit: SubmitHandler<AddUserModalForm> = (data) => {
		mutate({ name: data.name, projectId });
		dialogRef.current?.close();
	};

	return (
		<>
			<dialog ref={dialogRef} id={modalId} className="modal">
				<div className="modal-box flex gap-3 flex-col">
					<button type="button" className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" onClick={() => dialogRef.current?.close()}>
						✕
					</button>
					<h1>Ajouter un Utilisateur</h1>
					<form
						className="ml-4 mr-4"
						onSubmit={(e) => {
							e.preventDefault();
							onSubmit(getValues());
						}}
					>
						<div className="flex flex-col gap-3">
							<label className="label">Nom</label>
							<input className="input w-full" {...register('name', { required: true, maxLength: 100 })} />
							{errors.name && <span>Ce champ est requis</span>}

							{isPending && <span>Enregistrement…</span>}
							{isError && <span className="text-error">{(error as Error).message}</span>}
						</div>

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
