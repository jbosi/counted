import { useEffect, useState } from 'react';
import { ErrorBoundary } from 'react-error-boundary';
import { Outlet, useParams } from 'react-router';
import { ErrorFallback } from '../components/errorFallback';
import { Loading } from '../components/loading';
import { ProjectUsersContext } from '../contexts/projectUsersContext';
import { useUsersByProjectId } from '../hooks/useUsers';
import type { User } from '../types/users.model';

export function ProjectLayout() {
	const [projectUsers, setProjectUsers] = useState<User[]>();
	const { projectId } = useParams();

	const { data, isLoading } = useUsersByProjectId(projectId); // TODO : can i remove undefined ?

	useEffect(() => {
		setProjectUsers(data);
	}, [data]);

	return (
		<ProjectUsersContext value={{ projectUsers, setProjectUsers }}>
			{isLoading ? (
				<Loading />
			) : (
				<ErrorBoundary FallbackComponent={ErrorFallback}>
					<Outlet />
				</ErrorBoundary>
			)}
		</ProjectUsersContext>
	);
}
