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
import { HttpClientModule } from '@angular/common/http';
import { FileViewerComponent } from './pages/file-viewer/file-viewer.component';
import { AccordionComponent } from './shared/accordion/accordion.component';
import { CUSTOM_ELEMENTS_SCHEMA } from '@angular/core';

@NgModule({
  declarations: [
    AppComponent,
    FileUploadComponent,
    NavigationComponent,
    DashboardComponent,
    CardComponent,
    ViewerComponent,
    FileViewerComponent,
    AccordionComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    RouterOutlet,
    HttpClientModule
  ],
  providers: [],
  bootstrap: [AppComponent],
  schemas: [CUSTOM_ELEMENTS_SCHEMA]
})
export class AppModule { }
