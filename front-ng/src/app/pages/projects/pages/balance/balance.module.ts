import { CommonModule } from '@angular/common';
import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';



const routes: Routes = [
	{
		path: '',
		loadComponent: () => import('./components').then(m => m.BalanceComponent)
	}
]

@NgModule({
	imports: [
		CommonModule,
		RouterModule.forChild(routes),
	]
})
export class BalanceModule { }
