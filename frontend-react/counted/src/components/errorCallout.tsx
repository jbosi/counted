import type { FieldErrors, FieldValues } from 'react-hook-form';

type ErrorCalloutProps = { errors: FieldErrors<FieldValues> | undefined };

export function ErrorValidationCallout({ errors }: ErrorCalloutProps) {
	const errorMessages = getErrorMessages(errors);

	if (errorMessages.length === 0) {
		return null;
	}

	return (
		<div role="alert" className="alert alert-error whitespace-pre-line ml-4 mr-4">
			<svg xmlns="http://www.w3.org/2000/svg" className="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
				<path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
			</svg>
			<span>{errorMessages.join('\r\n')}</span>
		</div>
	);
}

function getErrorMessages(errors: FieldErrors<FieldValues> | undefined): string[] {
	if (!errors) return [];

	const messages: string[] = [];

	for (const [key, error] of Object.entries(errors)) {
		if (!error) {
			continue;
		}

		if (typeof error.message === 'string') {
			messages.push(`${key}: ${error.message}`);
		} else if ('root' in error && (error.root as FieldErrors[string])?.message) {
			messages.push(`${key}: ${(error.root as FieldErrors[string])!.message}`);
		} else if (typeof error === 'object' && !error.message && !error.type) {
			const nested = getErrorMessages(error as FieldErrors<FieldValues>);
			if (nested.length > 0) {
				messages.push(`${key}: ${nested[0]}`);
			}
		}
	}

	return messages;
}
