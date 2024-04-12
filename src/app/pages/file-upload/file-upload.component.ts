import { Component } from '@angular/core';
import { BackendService } from '../../services/backend.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-file-upload',
  templateUrl: './file-upload.component.html',
  styleUrl: './file-upload.component.scss'
})
export class FileUploadComponent {

  constructor(private backend: BackendService, private router: Router) {}

  fileHandler(e: Event) {
    const fileList = (e.target as HTMLInputElement).files!;

    if (fileList.length < 1) {
      return;
    }

    let file: File = fileList[0];

    this.backend.uploadFile(file).subscribe(() => {
      console.log('File uploaded');
    });

    this.router.navigate(['/dashboard']);
  }
}
