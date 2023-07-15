import { CommonModule } from '@angular/common';
import { ChangeDetectionStrategy, ChangeDetectorRef, Component, Input, OnInit } from '@angular/core';

const AVAILABLE_COLORS = [
	'#1abc9c', '#2ecc71', '#3498db', '#9b59b6', '#34495e', '#16a085', '#27ae60', '#2980b9',
	'#8e44ad', '#2c3e50', '#f1c40f', '#e67e22', '#e74c3c', '#95a5a6', '#f39c12', '#d35400', '#c0392b', '#bdc3c7', '#7f8c8d'
];
 

@Component({
	selector: 'app-avatar-initials',
	templateUrl: './avatar-initials.component.html',
	styleUrls: ['./avatar-initials.component.scss'],
	standalone: true,
	imports: [CommonModule],
	changeDetection: ChangeDetectionStrategy.OnPush
})
export class AvatarInitialsComponent implements OnInit {
	@Input() public userName: string = '';
	@Input() public size: number = 30;
	
	public backGroundColor: string = '#1abc9c';
	public initials: string = '';

	constructor(private readonly cdr: ChangeDetectorRef) {}

	ngOnInit(): void {
		const chars = this.userName?.split(' ')?.length > 1 ? this.userName?.split(' ') : this.userName;

		this.initials = chars?.[0]?.charAt(0)?.toUpperCase() + (chars?.[1]?.charAt(0)?.toUpperCase() ?? '');
		const charIndex = this.initials?.charCodeAt(0) - 65;
		const colorIndex = charIndex % 19;
		this.backGroundColor = AVAILABLE_COLORS?.[colorIndex];
		this.cdr.markForCheck();
	}
}
