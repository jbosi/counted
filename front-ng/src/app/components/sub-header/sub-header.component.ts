import { Component, OnInit } from '@angular/core';
import { AvatarModule } from 'primeng/avatar';
import { ButtonModule } from 'primeng/button';
import { AvatarInitialsComponent, IUser, UsersHttpClient } from '../../modules';
import { AddUsersModalComponent } from './modals';
import { CommonModule } from '@angular/common';

@Component({
	selector: 'app-sub-header',
	templateUrl: './sub-header.component.html',
	styleUrls: ['./sub-header.component.scss'],
	standalone: true,
	imports: [
		ButtonModule,
		AvatarModule,
		AddUsersModalComponent,
		AvatarInitialsComponent,
		CommonModule
	],
})
export class SubHeaderComponent implements OnInit {
	public users: IUser[] = [];
	constructor(
		private readonly usersHttpClient: UsersHttpClient
	) {}

	async ngOnInit(): Promise<void> {
		this.users = await this.usersHttpClient.getAsync();
	}

	public async onUsersAddedAsync(): Promise<void> {
		this.users = await this.usersHttpClient.getAsync();
	}
}
