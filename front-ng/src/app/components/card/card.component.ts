import { Component, Input } from '@angular/core';
import { IProject, IUser } from '../../modules';

@Component({
  selector: 'app-card',
  templateUrl: './card.component.html',
  styleUrls: ['./card.component.scss']
})
export class CardComponent {
	@Input() public users: IUser[] = [];
	@Input() public project = {} as IProject;
}
