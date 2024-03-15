import { Component } from '@angular/core';

@Component({
  selector: 'app-file-upload',
  templateUrl: './file-upload.component.html',
  styleUrl: './file-upload.component.scss'
})
export class FileUploadComponent {
  fileHandler(e: Event) {
    const file = (e.target as HTMLInputElement).files![0];

    console.log(file);
  }
}
