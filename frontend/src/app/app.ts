import { AsyncPipe } from '@angular/common';
import { HttpClient, provideHttpClient } from '@angular/common/http';
import { Component, inject, resource } from '@angular/core';
import { DomSanitizer } from '@angular/platform-browser';
import { RouterOutlet } from '@angular/router';
import { map } from 'rxjs';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, AsyncPipe],
  templateUrl: './app.html',
  styleUrl: './app.scss'
})
export class App {

  http = inject(HttpClient)
  sanitizer = inject(DomSanitizer)
  protected title = 'k8s-todo-frontend';

  img$ = this.http.get('/api/pic', {responseType: 'blob'})
  .pipe(map((imgBlob) => {
    return this.sanitizer.bypassSecurityTrustUrl(URL.createObjectURL(imgBlob))
  }))
}
