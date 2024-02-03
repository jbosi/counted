import { CommonModule } from '@angular/common';
import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { AddProjectModalComponent, CardComponent } from './components';
import { ProjectsComponent } from './projects.component';

import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { AvatarModule } from 'primeng/avatar';
import { AvatarGroupModule } from 'primeng/avatargroup';
import { ButtonModule } from 'primeng/button';
import { CardModule } from 'primeng/card';
import { DialogModule } from 'primeng/dialog';
import { InputTextModule } from 'primeng/inputtext';
import { MultiSelectModule } from 'primeng/multiselect';
import { ProgressBarModule } from 'primeng/progressbar';
import { SpeedDialModule } from 'primeng/speeddial';
import { TagModule } from 'primeng/tag';


const routes: Routes = [
	{
		path: '',
		pathMatch: 'full',
		component: ProjectsComponent
	},
	{
		path: ':projectId',
		loadChildren: () => import('./pages/project').then(m => m.ProjectModule)
	}
]

@NgModule({
	imports: [
		CommonModule,
		RouterModule.forChild(routes),
		CardModule,
		AvatarModule,
		AvatarGroupModule,
		ProgressBarModule,
		TagModule,
		SpeedDialModule,
		ButtonModule,
		DialogModule,
		InputTextModule,
		FormsModule,
		ReactiveFormsModule,
		MultiSelectModule,
		ProjectsComponent,
		CardComponent,
		AddProjectModalComponent
	]
})
export class ProjectsModule { }
