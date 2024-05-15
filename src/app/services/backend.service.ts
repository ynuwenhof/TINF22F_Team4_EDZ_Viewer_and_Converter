import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable, from, map, switchMap, tap, toArray } from 'rxjs';
import { ArchiveInformation } from '../interfaces/archive-information';
import { PackageInformation } from '../interfaces/package-information';
import { ExplorerItem } from '../interfaces/explorer-item';

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

    return this.http.post<String>(this.apiUrl + 'samples/', formData);
  }

  getExplorerLevel(id: string, path: string): Observable<any> {
    const fileExtension = path.split('.').pop();

    if (fileExtension === 'xml' || fileExtension === 'ema')
      return this.http.get<any>(this.apiUrl + 'samples/' + id + '/blob' + path, { responseType: 'text' as 'json' });

    if (fileExtension === 'jpg' || fileExtension === 'png')
      return this.http.get<any>(this.apiUrl + 'samples/' + id + '/blob' + path, { responseType: 'blob' as 'json' });

    return this.http.get<any>(this.apiUrl + 'samples/' + id + '/blob' + path, { responseType: 'json' });
  }

  getFullUrl(id: string, path: string): string {
    return this.apiUrl + 'samples/' + id + '/blob' + path;
  }

  getDownloadLink(id: string): string {
    return this.apiUrl + 'samples/' + id + '/aasx';
  }
}
