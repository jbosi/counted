import { type RefObject } from 'react';
import { useForm, type SubmitHandler } from 'react-hook-form';
import { useAddUser } from '../../hooks/useUsers';

export interface AddUserModalProps {
	modalId: string;
	dialogRef: RefObject<HTMLDialogElement | null>;
	projectId: string;
}

export interface AddUserModalForm {
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
						className="flex flex-col gap-3"
						onSubmit={(e) => {
							e.preventDefault();
							onSubmit(getValues());
						}}
					>
						<label>Nom</label>
						<input {...register('name', { required: true, maxLength: 100 })} />
						{errors.name && <span>Ce champ est requis</span>}

						{isPending && <span>Enregistrement…</span>}
						{isError && <span className="text-error">{(error as Error).message}</span>}

						<button className="btn btn-primary" type="submit">
							Enregistrer
						</button>
					</form>
				</div>
			</dialog>
		</>
	);
}
