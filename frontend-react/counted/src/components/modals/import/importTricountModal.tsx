import { useState, type RefObject } from 'react';
import { useImportTricount } from '../../../hooks/useImportTricount';
import { ModalFooter } from '../shared/modalFooter';

interface ImportTricountModalProps {
	dialogRef: RefObject<HTMLDialogElement | null>;
	modalId: string;
	closeDialogFn: () => void;
}

export function ImportTricountModal({ dialogRef, modalId, closeDialogFn }: ImportTricountModalProps) {
	const [tricountKey, setTricountKey] = useState('');
	const { mutateAsync, isPending, isError, error } = useImportTricount();

	const handleSubmit = async () => {
		if (!tricountKey.trim()) return;
		await mutateAsync({ tricountKey: tricountKey.trim() });
		closeDialogFn();
	};

	return (
		<dialog ref={dialogRef} id={modalId} className="modal">
			<div className="modal-box flex gap-3 flex-col">
				<button type="button" className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2" onClick={closeDialogFn}>
					✕
				</button>
				<h1>Importer depuis Tricount</h1>

				<label className="label">Clé ou URL Tricount</label>
				<input
					className="input w-full"
					placeholder="Ex: AbCdEf123 ou https://tricount.com/..."
					value={tricountKey}
					onChange={(e) => setTricountKey(e.target.value)}
					onKeyDown={(e) => {
						if (e.key === 'Enter') handleSubmit();
					}}
				/>
				<p className="text-xs text-base-content/60">
					Vous pouvez trouver la clé dans l'URL de votre tricount (ex: tricount.com/<strong>AbCdEf123</strong>)
				</p>

				{isPending && (
					<div className="flex items-center gap-2">
						<span className="loading loading-spinner loading-sm" />
						<span>Import en cours...</span>
					</div>
				)}

				{isError && <div className="text-error text-sm">{(error as Error)?.message ?? 'Erreur lors de l`import'}</div>}

				<ModalFooter closeDialogFn={closeDialogFn} disabled={isPending || !tricountKey.trim()} handleSubmit={handleSubmit} isLoading={isPending} submitName="Importer" />
			</div>
		</dialog>
	);
}
