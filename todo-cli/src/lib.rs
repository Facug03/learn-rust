use std::fmt::Display;

pub struct Todos {
  todos: Vec<Todo>,
}

impl Todos {
  pub fn new() -> Todos {
    Todos { todos: vec![] }
  }

  pub fn add(&mut self, text: &str) {
    let id = self.todos.len() + 1;

    self.todos.push(Todo::new(id, text.to_string()));
  }

  pub fn remove(&mut self, id: usize) -> Result<Todo, &str> {
    if let Some(index) = self.todos.iter().position(|todo| todo.id == id) {
      let todo_removed = self.todos.remove(index);
      Ok(todo_removed)
    } else {
      Err("Id given does not exists")
    }
  }

  pub fn done(&mut self, id: usize) -> Result<(), &str> {
    let todo = self
      .todos
      .iter_mut()
      .find(|todo| todo.id == id)
      .expect("Todo given does not exists");

    todo.done();

    Ok(())
  }

  pub fn list(&self) -> &Vec<Todo> {
    &self.todos
  }
}

#[derive(Debug)]
pub struct Todo {
  id: usize,
  text: String,
  status: TodoStatus,
}

#[derive(Debug)]
pub enum TodoStatus {
  Done,
  Created,
}

impl Display for TodoStatus {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      TodoStatus::Done => write!(f, "Done"),
      TodoStatus::Created => write!(f, "Created"),
    }
  }
}

impl Todo {
  fn new(id: usize, text: String) -> Todo {
    Todo {
      id,
      text,
      status: TodoStatus::Created,
    }
  }

  fn done(&mut self) {
    self.status = TodoStatus::Done;
  }

  pub fn id(&self) -> usize {
    self.id
  }

  pub fn text(&self) -> &str {
    &self.text
  }

  pub fn status(&self) -> &TodoStatus {
    &self.status
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_add_todo() {
    let mut todos = Todos::new();
    let todo_to_add = "make dinner";

    todos.add(todo_to_add);

    assert_eq!(todos.list().len(), 1);
    assert_eq!(todos.list()[0].text(), todo_to_add)
  }

  #[test]
  fn should_add_multiple_todos() {
    let mut todos = Todos::new();

    todos.add("sleep");
    todos.add("workout");

    assert_eq!(todos.list().len(), 2);
  }

  #[test]
  fn should_remove_todo() {
    let mut todos = Todos::new();
    let todo_to_add = "make dinner";

    todos.add(todo_to_add);
    todos.remove(1).expect("Imposible to fail");

    assert_eq!(todos.list().len(), 0);
  }

  #[test]
  fn remove_wrong_id() {
    let mut todos = Todos::new();
    let id_to_remove = 1;
    let result = todos.remove(id_to_remove);

    assert_eq!(result.unwrap_err(), "Id given does not exists")
  }

  #[test]
  fn should_list() {
    let mut todos = Todos::new();

    todos.add("work");
    todos.add("go home");

    assert_eq!(todos.list()[0].text(), "work");
    assert_eq!(todos.list()[1].text(), "go home");
  }

  #[test]
  fn should_close_a_task() {
    let mut todos = Todos::new();

    todos.add("work");
    todos.add("go home");

    let result = todos.done(1).expect("should not fail");

    assert_eq!(result, ());
  }
}
