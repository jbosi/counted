import { Component, EventEmitter, Input, OnInit, Output, inject } from '@angular/core';
import { FormControl, FormGroup, FormsModule, ReactiveFormsModule } from '@angular/forms';
import { ICreatableProject, IUser, ProjectsHttpClient } from '../../../../modules';
import { ButtonModule } from 'primeng/button';
import { MultiSelectModule } from 'primeng/multiselect';
import { InputTextModule } from 'primeng/inputtext';
import { DialogModule } from 'primeng/dialog';

@Component({
    selector: 'app-add-project-modal',
    templateUrl: './add-project-modal.component.html',
    styleUrls: ['./add-project-modal.component.scss'],
    imports: [DialogModule, FormsModule, ReactiveFormsModule, InputTextModule, MultiSelectModule, ButtonModule]
})
export class AddProjectModalComponent implements OnInit {
	private readonly projectsHttpClient = inject(ProjectsHttpClient);

	@Output() public projectAdded = new EventEmitter<void>();
	@Input() public users: IUser[] = [];

	public form = {} as FormGroup;

	public display: boolean = false;

	ngOnInit(): void {
		this.form = new FormGroup({
			projectName: new FormControl(),
			selectedUserIds: new FormControl()
		})
	}

	public showDialog(): void {
		this.display = true;
	}

	public async onSubmitAsync(): Promise<void> {
		const candidate = {
			name: this.form?.value?.projectName,
			users: this.form?.value?.selectedUserIds
		} as any as ICreatableProject // TODO

		await this.projectsHttpClient.createAsync(candidate);

		this.display = false;
		this.projectAdded.next();
	}
}
