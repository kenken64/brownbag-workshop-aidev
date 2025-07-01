import { Component, Input, Output, EventEmitter } from '@angular/core';
import { Todo } from '../../services/todo.service';

@Component({
  selector: 'app-todo-item',
  templateUrl: './todo-item.component.html',
  styleUrls: ['./todo-item.component.css']
})
export class TodoItemComponent {
  @Input() todo!: Todo;
  @Output() updateTodo = new EventEmitter<Todo>();
  @Output() deleteTodo = new EventEmitter<number>();
  @Output() toggleCompleted = new EventEmitter<Todo>();

  isEditing = false;
  editedTitle: string = '';

  onDelete(): void {
    if (this.todo.id !== undefined) {
      this.deleteTodo.emit(this.todo.id);
    }
  }

  onToggleCompleted(): void {
    this.toggleCompleted.emit(this.todo);
  }

  onEdit(): void {
    this.isEditing = true;
    this.editedTitle = this.todo.title;
  }

  onSaveEdit(): void {
    if (this.todo.id !== undefined) {
      this.updateTodo.emit({ ...this.todo, title: this.editedTitle });
      this.isEditing = false;
    }
  }

  onCancelEdit(): void {
    this.isEditing = false;
  }
}