import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { ArchiveInformation } from '../interfaces/archive-information';
import { PackageInformation } from '../interfaces/package-information';

@Injectable({
  providedIn: 'root'
})
export class BackendService {

  constructor(private http: HttpClient) { }

  getAllArchives(): Observable<string[]> {
    return this.http.get<string[]>('/assets/allArchives.json');
  }

  getArchive(hash: string): Observable<ArchiveInformation> {
    return this.http.get<ArchiveInformation>('/assets/archives.json');
  }

  getPackage(hash:string, index: number): Observable<PackageInformation> {
    return this.http.get<PackageInformation>('/assets/packages.json');
  }
}
