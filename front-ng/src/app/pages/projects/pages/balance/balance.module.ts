import { CommonModule } from '@angular/common';
import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { BalanceComponent } from './components';


const routes: Routes = [
	{
		path: '',
		component: BalanceComponent
	}
]

@NgModule({
	imports: [
		CommonModule,
		RouterModule.forChild(routes),
	]
})
export class BalanceModule { }
