import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { FileUploadComponent } from './pages/file-upload/file-upload.component';
import { DashboardComponent } from './pages/dashboard/dashboard.component';
import { ViewerComponent } from './pages/viewer/viewer.component';
import { FileViewerComponent } from './pages/file-viewer/file-viewer.component';

const routes: Routes = [
  { path: 'upload', component: FileUploadComponent },
  { path: 'dashboard', component: DashboardComponent },
  { path: 'viewer/:id/:index', component: ViewerComponent },
  { path: 'files/:id', component: FileViewerComponent },
  { path: '**', redirectTo: '/upload' },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
