import { Component, Output, EventEmitter } from '@angular/core';
import { Todo } from '../../services/todo.service';

@Component({
  selector: 'app-todo-form',
  templateUrl: './todo-form.component.html',
  styleUrls: ['./todo-form.component.css']
})
export class TodoFormComponent {
  newTodoTitle: string = '';
  @Output() addTodo = new EventEmitter<Todo>();

  onSubmit(): void {
    if (this.newTodoTitle.trim()) {
      const newTodo: Todo = {
        title: this.newTodoTitle.trim(),
        completed: false
      };
      this.addTodo.emit(newTodo);
      this.newTodoTitle = '';
    }
  }
}