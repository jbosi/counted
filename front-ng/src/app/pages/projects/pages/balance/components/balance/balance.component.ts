import { NgFor } from '@angular/common';
import { Component, inject, OnInit, signal } from '@angular/core';
import { AvatarInitialsComponent, RouterParamService } from '@hcount/modules';
import { IBalanceViewModel } from '../../models';
import { BalanceApplication } from './balance.application';

@Component({
  selector: 'app-balance',
  standalone: true,
  imports: [AvatarInitialsComponent, NgFor],
  templateUrl: './balance.component.html',
  styleUrl: './balance.component.scss'
})
export class BalanceComponent implements OnInit {
	private readonly balanceApplication = inject(BalanceApplication);
	private readonly routerParamService = inject(RouterParamService);
	public userBalances = signal<IBalanceViewModel | null>(null);

	async ngOnInit(): Promise<void> {
		const projectId = this.routerParamService.getParam('projectId');

		if (projectId == null) {
			console.error('projectId could not be null');
			return;
		}

		this.userBalances.set(await this.balanceApplication.getBalanceAsync(projectId));
	}
}
