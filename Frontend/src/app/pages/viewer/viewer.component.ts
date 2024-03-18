import { Component } from '@angular/core';
import { ActivatedRoute } from '@angular/router';

@Component({
  selector: 'app-viewer',
  templateUrl: './viewer.component.html',
  styleUrl: './viewer.component.scss'
})
export class ViewerComponent {
  loading = true;
  component: any;

  constructor(private activeRoute: ActivatedRoute) {
    setTimeout(() => {
      this.loading = false;
      this.component = {
        title: this.activeRoute.snapshot.paramMap.get('componentID'),
        part: this.activeRoute.snapshot.paramMap.get('partID'), // convert to number
        partCount: 5,
      };
    }, 400);

  }
}
