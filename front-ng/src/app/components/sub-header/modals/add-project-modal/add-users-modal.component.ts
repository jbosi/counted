import { CommonModule } from '@angular/common';
import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { FormArray, FormControl, FormGroup, FormsModule, ReactiveFormsModule } from '@angular/forms';
import { ButtonModule } from 'primeng/button';
import { DialogModule } from 'primeng/dialog';
import { InputTextModule } from 'primeng/inputtext';
import { ICreatableUsers, IUser, UsersHttpClient } from '../../../../modules';

@Component({
	selector: 'app-add-users-modal',
	templateUrl: './add-users-modal.component.html',
	styleUrls: ['./add-users-modal.component.scss'],
	standalone: true,
	imports: [DialogModule, FormsModule, ReactiveFormsModule, InputTextModule, ButtonModule, CommonModule]
})
export class AddUsersModalComponent implements OnInit {
	@Output() public usersAdded = new EventEmitter<void>();
	@Input() public existingUsers: IUser[] = [];

	public form = new FormGroup({
		users: new FormArray<FormGroup<IUsersForm>>([
			new FormGroup({
				name: new FormControl('', { nonNullable: true })
			})
		]),
	});
	
	public users!: FormArray<FormGroup<IUsersForm>>;

	public display: boolean = false;
	constructor(
		private readonly usersHttpClient: UsersHttpClient
	) {}

	ngOnInit(): void {
		this.users = this.form.controls["users"];
	}

	public showDialog(): void {
		this.form.reset();
		this.display = true;
	}

	public async createUsersAsync(): Promise<void> {
		const rawUsers = this.form
			.getRawValue().users
			?.filter(user => user.name != null && user.name !== '');
		
		if (rawUsers == null || rawUsers?.length === 0) {
			this.display = false;
			return;
		}

		const candidates: ICreatableUsers = rawUsers
			?.map(user => ({ name: user.name }));
		
		await this.usersHttpClient.createAsync(candidates);

		this.display = false;
		this.usersAdded.next();
	}

	public addCandidateUser(): void {
		const user = new FormGroup({
			name: new FormControl('', { nonNullable: true })
		})
		this.users.push(user);
	}

	public deleteCandidateUser(userIndex: number): void {
		this.users.removeAt(userIndex);
	}

	public async deleteExistingUserAsync(userId: number): Promise<void> {
		await this.usersHttpClient.deleteAsync(userId).then(() => {
			this.existingUsers = this.existingUsers.filter(user => user.id != userId);
		});
	}
}


interface IUsersForm {
	name: FormControl<string>
}