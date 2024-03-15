import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import { FileUploadComponent } from './pages/file-upload/file-upload.component';

const routes: Routes = [
  { path: 'upload', component: FileUploadComponent, pathMatch: 'full' },
  { path: '**',   redirectTo: '/upload', pathMatch: 'full' },
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
