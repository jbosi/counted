import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';




const routes: Routes = [
	{
		path: '',
		loadComponent: () => import('./project.component').then(m => m.ProjectComponent)
	}
]

@NgModule({
	imports: [
		RouterModule.forChild(routes),
	]
})
export class ProjectModule { }
