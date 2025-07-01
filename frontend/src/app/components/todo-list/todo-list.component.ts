import { Component, OnInit } from '@angular/core';
import { Todo, TodoService } from '../../services/todo.service';

@Component({
  selector: 'app-todo-list',
  templateUrl: './todo-list.component.html',
  styleUrls: ['./todo-list.component.css']
})
export class TodoListComponent implements OnInit {
  todos: Todo[] = [];

  constructor(private todoService: TodoService) { }

  ngOnInit(): void {
    this.getTodos();
  }

  getTodos(): void {
    this.todoService.getTodos()
      .subscribe(todos => {
        this.todos = todos;
        console.log('Fetched todos:', todos);
      });
  }

  addTodo(todo: Todo): void {
    this.todoService.addTodo(todo)
      .subscribe(newTodo => {
        this.todos.push(newTodo);
        this.getTodos(); // Refresh the list to get the ID from the backend
      });
  }

  updateTodo(todo: Todo): void {
    this.todoService.updateTodo(todo)
      .subscribe(() => this.getTodos());
  }

  deleteTodo(id: number): void {
    this.todoService.deleteTodo(id)
      .subscribe(() => {
        this.todos = this.todos.filter(t => t.id !== id);
      });
  }

  toggleCompleted(todo: Todo): void {
    todo.completed = !todo.completed;
    this.updateTodo(todo);
  }
}