import { HttpClient } from '@angular/common/http';
import { inject, Injectable } from '@angular/core';
import { Observable } from 'rxjs';

export interface Todo {
  id: string;
  value: string;
}

export interface CreateTodo {
  value: string;
}

@Injectable({
  providedIn: 'root'
})
export class TodosService {
  httpClient = inject(HttpClient);

  getAll(): Observable<Todo[]> {
    return this.httpClient.get<Todo[]>("/api/todos");
  }

  create(todo: CreateTodo) {
    return this.httpClient.post<Todo>("/api/todos", todo);
  }

}
