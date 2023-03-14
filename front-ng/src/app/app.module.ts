import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AvatarModule } from 'primeng/avatar';
import { AvatarGroupModule } from 'primeng/avatargroup';
import { ButtonModule } from 'primeng/button';
import { CardModule } from 'primeng/card';
import { ProgressBarModule } from 'primeng/progressbar';
import { SpeedDialModule } from 'primeng/speeddial';
import { TagModule } from 'primeng/tag';
import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { CardComponent, HeaderComponent } from './components';
import { FetchHttpClient } from './modules';

@NgModule({
	declarations: [
		AppComponent,
		HeaderComponent,
		CardComponent
	],
	imports: [
		BrowserModule,
		AppRoutingModule,
		CardModule,
		AvatarModule,
		AvatarGroupModule,
		ProgressBarModule,
		TagModule,
		SpeedDialModule,
		ButtonModule
	],
	providers: [FetchHttpClient],
	bootstrap: [AppComponent]
})
export class AppModule { }
