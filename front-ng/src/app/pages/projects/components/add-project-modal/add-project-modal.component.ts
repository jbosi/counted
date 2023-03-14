import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import { FormControl, FormGroup } from '@angular/forms';
import { ICreatableProject, IUser, ProjectsHttpClient } from '../../../../modules';

@Component({
  selector: 'app-add-project-modal',
  templateUrl: './add-project-modal.component.html',
  styleUrls: ['./add-project-modal.component.scss']
})
export class AddProjectModalComponent implements OnInit {
	@Output() public projectAdded = new EventEmitter<void>();
	@Input() public users: IUser[] = [];

	public form = {} as FormGroup;

	public display: boolean = false;
	constructor(
		private readonly projectsHttpClient: ProjectsHttpClient
	) {}

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
