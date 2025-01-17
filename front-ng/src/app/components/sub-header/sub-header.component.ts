
import { Component, OnInit, inject } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { ButtonModule } from 'primeng/button';
import { AvatarInitialsComponent, IUser, RouterParamService, UsersHttpClient } from '../../modules';
import { AddUsersModalComponent } from './modals';

@Component({
    selector: 'app-sub-header',
    templateUrl: './sub-header.component.html',
    styleUrls: ['./sub-header.component.scss'],
    imports: [
        ButtonModule,
        AvatarModule,
        AddUsersModalComponent,
        AvatarInitialsComponent
    ]
})
export class SubHeaderComponent implements OnInit {
	private readonly usersHttpClient = inject(UsersHttpClient);
	private readonly routerParamService = inject(RouterParamService);

	public users: IUser[] = [];

	async ngOnInit(): Promise<void> {
		const project_id: string | undefined = this.routerParamService.getParam('projectId');
		if (project_id == null) {
			console.error('project_id should not be null');
			return;
		}

		this.users = await this.usersHttpClient.getUsersByProjectIdAsync(project_id);
	}

	public async onUsersAddedAsync(): Promise<void> {
		this.users = await this.usersHttpClient.getAsync();
	}
}
