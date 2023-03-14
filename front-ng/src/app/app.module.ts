import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { AvatarModule } from 'primeng/avatar';
import { AvatarGroupModule } from 'primeng/avatargroup';
import { ButtonModule } from 'primeng/button';
import { CardModule } from 'primeng/card';
import { DialogModule } from 'primeng/dialog';
import { ProgressBarModule } from 'primeng/progressbar';
import { SpeedDialModule } from 'primeng/speeddial';
import { TagModule } from 'primeng/tag';
import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { AddProjectModalComponent, CardComponent, HeaderComponent } from './components';
import { FetchHttpClient } from './modules';
import { InputTextModule } from 'primeng/inputtext';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { MultiSelectModule } from 'primeng/multiselect';

@NgModule({
	declarations: [
		AppComponent,
		HeaderComponent,
		CardComponent,
		AddProjectModalComponent
	],
	imports: [
		BrowserModule,
		BrowserAnimationsModule,
		AppRoutingModule,
		CardModule,
		AvatarModule,
		AvatarGroupModule,
		ProgressBarModule,
		TagModule,
		SpeedDialModule,
		ButtonModule,
		DialogModule,
		InputTextModule,
		FormsModule,
		ReactiveFormsModule,
		MultiSelectModule
	],
	providers: [FetchHttpClient],
	bootstrap: [AppComponent]
})
export class AppModule { }
