import { Component } from '@angular/core';
import { BackendService } from '../../services/backend.service';
import { Router } from '@angular/router';

@Component({
  selector: 'app-file-upload',
  templateUrl: './file-upload.component.html',
  styleUrl: './file-upload.component.scss'
})
export class FileUploadComponent {
  canLeave = true;

  constructor(private backend: BackendService, private router: Router) {
    window.addEventListener('beforeunload', (event) => {
      if (!this.canLeave) {
        event.returnValue = 'Die Datei Lädt noch hoch!';
      }
    });
  }

  fileHandler(e: Event) {
    this.canLeave = false;
    
    const fileList = (e.target as HTMLInputElement).files!;

    if (fileList.length < 1) {
      return;
    }

    let file: File = fileList[0];

    this.backend.uploadFile(file).subscribe(() => {
      this.canLeave = true;
      this.router.navigate(['/dashboard']);
    });
  }
}
