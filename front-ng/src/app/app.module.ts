import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AvatarModule } from 'primeng/avatar';
import { AvatarGroupModule } from 'primeng/avatargroup';
import { CardModule } from 'primeng/card';
import { ProgressBarModule } from 'primeng/progressbar';
import { TagModule } from 'primeng/tag';
import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';

@NgModule({
	declarations: [
		AppComponent
	],
	imports: [
		BrowserModule,
		AppRoutingModule,
		CardModule,
		AvatarModule,
		AvatarGroupModule,
		ProgressBarModule,
		TagModule
	],
	providers: [],
	bootstrap: [AppComponent]
})
export class AppModule { }
