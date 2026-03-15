import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useContext } from 'react';
import { CountedLocalStorageContext } from '../contexts/localStorageContext';
import { importService, type TricountImportRequest, type TricountImportResponse } from '../services/importService';
import type { ProjectDto } from '../types/projects.model';

export function useImportTricount() {
	const { saveProjectEntry } = useContext(CountedLocalStorageContext);
	const queryClient = useQueryClient();

	return useMutation({
		mutationFn: (request: TricountImportRequest) => importService.importTricount(request),
		onSuccess: (data: TricountImportResponse) => {
			queryClient.setQueryData(['projects'], (old: ProjectDto[]) => [...(old ?? []), data.project]);
			saveProjectEntry({ projectId: data.project.id, userId: null });
		},
	});
}
