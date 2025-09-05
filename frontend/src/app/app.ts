import { AsyncPipe } from '@angular/common';
import { HttpClient } from '@angular/common/http';
import { Component, inject } from '@angular/core';
import { DomSanitizer } from '@angular/platform-browser';
import { map } from 'rxjs';
import { CreateTodo, TodosService } from './core/services/todos.service';
import { FormControl, ReactiveFormsModule } from '@angular/forms';

@Component({
  selector: 'app-root',
  imports: [AsyncPipe, ReactiveFormsModule],
  templateUrl: './app.html',
  styleUrl: './app.scss'
})
export class App {
  todosService = inject(TodosService)
  http = inject(HttpClient)
  sanitizer = inject(DomSanitizer)
  protected title = 'k8s-todo-frontend';

  control = new FormControl<string>('');

  img$ = this.http.get('/api/pic', {responseType: 'blob'})
  .pipe(map((imgBlob) => {
    return this.sanitizer.bypassSecurityTrustUrl(URL.createObjectURL(imgBlob))
  }))

  todos$ = this.todosService.getAll();

  onSubmit() {
    if (!this.control.valid) {
      return;
    }
    const value = this.control.value?.trim() ?? '';
    if(value.length === 0) {
      return
    }

    this.todosService.create({value} satisfies CreateTodo).subscribe();
  }
}
