import { CommonModule } from '@angular/common';
import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { ProjectComponent } from './project.component';

import { AvatarModule } from 'primeng/avatar';
import { AvatarGroupModule } from 'primeng/avatargroup';
import { ButtonModule } from 'primeng/button';
import { CardModule } from 'primeng/card';
import { ExpenseComponent } from './components';


const routes: Routes = [
	{
		path: '',
		component: ProjectComponent
	}
]

@NgModule({
    imports: [
        CommonModule,
        RouterModule.forChild(routes),
        CardModule,
        AvatarModule,
        AvatarGroupModule,
        ButtonModule,
        ProjectComponent,
        ExpenseComponent
    ]
})
export class ProjectModule { }
