import { NgModule } from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { FileUploadComponent } from './pages/file-upload/file-upload.component';
import { RouterOutlet } from '@angular/router';
import { NavigationComponent } from './shared/navigation/navigation.component';
import { DashboardComponent } from './pages/dashboard/dashboard.component';
import { CardComponent } from './shared/card/card.component';
import { ViewerComponent } from './pages/viewer/viewer.component';

@NgModule({
  declarations: [
    AppComponent,
    FileUploadComponent,
    NavigationComponent,
    DashboardComponent,
    CardComponent,
    ViewerComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    RouterOutlet
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
