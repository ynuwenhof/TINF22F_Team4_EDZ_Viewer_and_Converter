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

  constructor(private route: ActivatedRoute) {}

  ngOnInit(): void {
    this.route.params.subscribe(params => {
      const componentID = params['componentID'];
      const partID = +params['partID'];

      // REPLACE WITH API REQUEST

      this.loading = true;
      setTimeout(() => {
        this.loading = false;
        this.component = {
          id: componentID,
          part: partID,
          partCount: 5,
        };
      }, 400);
    })
  }
}
