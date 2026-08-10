use std::io;
use todo_cli::Todos;

fn main() {
  let mut todos = Todos::new();

  loop {
    let mut input = String::new();

    io::stdin()
      .read_line(&mut input)
      .expect("Error reading your input");

    let input: Vec<&str> = input.split(' ').map(|text| text.trim()).collect();

    if input.len() < 2 {
      println!("Not enough arguments");
      continue;
    }

    if input[0] != "todo" {
      println!("Unknown command {}", input[0]);
      println!("Use todo command");
      continue;
    }

    let action = input[1];

    match action {
      "add" => {
        let action_to_perform = get_action_to_perfom(&input);

        if let Some(action_to_perform) = action_to_perform {
          todos.add(&action_to_perform);
          println!("Added succesfully!");
        }
        continue;
      }
      "remove" => {
        if let None = is_there_action_in_input(&input) {
          continue;
        }

        let id = input[2].parse::<usize>();

        match id {
          Ok(id) => match todos.remove(id) {
            Ok(todo) => {
              println!("Removed todo: {}", todo.text());
            }
            Err(err) => {
              println!("{err}");
            }
          },
          Err(_) => {
            println!("Id should be a number");
            continue;
          }
        }

        continue;
      }
      "done" => {
        if let None = is_there_action_in_input(&input) {
          continue;
        }

        let id = input[2].parse::<usize>();

        match id {
          Ok(id) => match todos.done(id) {
            Ok(_) => {
              println!("Todo completed!");
            }
            Err(err) => {
              println!("{err}");
            }
          },
          Err(_) => {
            println!("Id should be a number");
            continue;
          }
        }
      }
      "list" => {
        let list_of_todos = todos.list();

        if list_of_todos.len() == 0 {
          println!("There is no todo's");
          continue;
        }

        list_of_todos
          .into_iter()
          .for_each(|todo| println!("{}: {} - {}", todo.id(), todo.text(), todo.status()));
      }
      action_given => {
        println!("Action {} not recognized", action_given);
        continue;
      }
    }
  }
}

fn get_action_to_perfom(input: &Vec<&str>) -> Option<String> {
  match is_there_action_in_input(input) {
    Some(_) => Some(input[2..].join(" ")),
    None => None,
  }
}

fn is_there_action_in_input(input: &Vec<&str>) -> Option<()> {
  if input.len() < 3 {
    println!("Not enough arguments");
    return None;
  }

  Some(())
}
