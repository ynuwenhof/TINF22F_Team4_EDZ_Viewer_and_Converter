import { Component } from '@angular/core';

@Component({
  selector: 'app-dashboard',
  templateUrl: './dashboard.component.html',
  styleUrl: './dashboard.component.scss'
})
export class DashboardComponent {
  loading = true;
  components: any;

  constructor() {
    setTimeout(() => {
      this.loading = false;
      this.components = [
        {
          title: "Bauteil",
          description: "Beschreigung?",
          size: "xxMB",
          date: new Date(),
          partCount: 10,
          id: "xxx"
        },
        {
          title: "Bauteil2",
          description: "Beschreigung?",
          size: "xxxMB",
          date: new Date(),
          partCount: 3,
          id: "yyy"
        },
        {
          title: "Bauteil2",
          description: "Beschreigung?",
          size: "xxxMB",
          date: new Date(),
          partCount: 3,
          id: "zzz"
        },
      ];

    }, 400)
  }
}
