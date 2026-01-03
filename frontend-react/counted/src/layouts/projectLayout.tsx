import { useEffect, useState } from 'react';
import { ProjectUsersContext } from '../contexts/projectUsersContext';
import type { User } from '../types/users.model';
import { Outlet, useParams } from 'react-router';
import { useUsersByProjectId } from '../hooks/useUsers';
import { Loading } from '../components/loading';

export function ProjectLayout() {
	const [projectUsers, setProjectUsers] = useState<User[]>();
	const { projectId } = useParams();

	const { data, isLoading } = useUsersByProjectId(projectId); // TODO : can i remove undefined ?

	useEffect(() => {
		setProjectUsers(data);
	}, [data]);

	return <ProjectUsersContext value={{ projectUsers, setProjectUsers }}>{isLoading ? <Loading /> : <Outlet />}</ProjectUsersContext>;
}
