import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ArchiveInformation } from '../interfaces/archive-information';
import { PackageInformation } from '../interfaces/package-information';

@Injectable({
  providedIn: 'root'
})
export class BackendService {
  apiUrl = 'https://rplan.tethy.xyz/';

  constructor(private http: HttpClient) { }

  getAllArchives(): Observable<ArchiveInformation[]> {
    return this.http.get<ArchiveInformation[]>(this.apiUrl + 'samples/');
  }

  getArchive(id: string): Observable<ArchiveInformation> {
    return this.http.get<ArchiveInformation>(this.apiUrl + 'samples/' + id);
  }

  getPackage(id:string, index: number): Observable<PackageInformation> {
    return this.http.get<PackageInformation>(this.apiUrl + 'samples/' + id + '/packages/' + index);
  }

  uploadFile(file: File): Observable<String> {
    const formData = new FormData();
    formData.append('file', file);

    return this.http.post<String>(this.apiUrl + 'samples/', formData, { 
      headers: new HttpHeaders({
        'Content-Type': 'multipart/form-data',
        'Accept': 'application/json'
      })
    });
  }
}
