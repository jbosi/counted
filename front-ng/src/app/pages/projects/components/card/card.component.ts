
import { Component, Input } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { AvatarInitialsComponent } from '@hcount/modules';
import { SharedModule } from 'primeng/api';
import { AvatarGroupModule } from 'primeng/avatargroup';
import { CardModule } from 'primeng/card';
import { ProgressBarModule } from 'primeng/progressbar';
import { TagModule } from 'primeng/tag';
import { IProjectCardViewModel } from '../../projects.component';

@Component({
	selector: 'app-card',
	templateUrl: './card.component.html',
	styleUrls: ['./card.component.scss'],
	standalone: true,
	imports: [CardModule, SharedModule, ProgressBarModule, TagModule, AvatarGroupModule, AvatarInitialsComponent]
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
