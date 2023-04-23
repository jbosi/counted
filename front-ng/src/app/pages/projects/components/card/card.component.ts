import { Component, Input } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { IProjectCardViewModel } from '../../projects.component';
import { AvatarModule } from 'primeng/avatar';
import { NgFor } from '@angular/common';
import { AvatarGroupModule } from 'primeng/avatargroup';
import { TagModule } from 'primeng/tag';
import { ProgressBarModule } from 'primeng/progressbar';
import { SharedModule } from 'primeng/api';
import { CardModule } from 'primeng/card';

@Component({
    selector: 'app-card',
    templateUrl: './card.component.html',
    styleUrls: ['./card.component.scss'],
    standalone: true,
    imports: [CardModule, SharedModule, ProgressBarModule, TagModule, AvatarGroupModule, NgFor, AvatarModule]
})
export class CardComponent {
	@Input() public project = {} as IProjectCardViewModel;

	constructor(
		private readonly router: Router,
		private readonly activatedRoute: ActivatedRoute
	) {}

	public onCardClick(): void {
		this.router.navigate([`${this.project.id}`], { relativeTo: this.activatedRoute });
	}
}
