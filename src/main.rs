use std::{fs, io, path::Path};
use std::io::Write;
use chrono::{DateTime,Utc};
//use chrono_tz::Asia::Taipei;
use clearscreen;
use serde::{Deserialize, Serialize};
//libray to process the struct
//derive ma hoa user duoi dang don gian nhat
//tao struct de su ly chuooi nhap
#[derive(Debug, Serialize, Deserialize)]
struct ToDo {
    date: DateTime<Utc>,
    text:String,
    done: bool,
}


impl ToDo{
    fn new(text: String) -> Self{
        ToDo{
            date:Utc::now(),
            text,
            done:false
        }
    }
}

fn add_todo(todos: &mut Vec<ToDo>, new_text: String){
    todos.push(ToDo::new(new_text.clone())); 
    println!("add: {}",new_text);
}

fn remove_task(todos: &mut Vec<ToDo>, index: usize){
    if index > 0 && index <= todos.len(){
        let remove = todos.remove(index - 1 );  
        println!("Index {} have been remove",remove.text);
    } else {
        println!("Task not exsit");
    }
}

fn mark_done(todos: &mut Vec<ToDo>, index: usize){
    if index > 0 && index <= todos.len() {
        todos[index - 1].done = true ;
        println!("The task {} has finish ",index);
    }else {
        println!("Index Unvalided");
    }
}

fn edit_part(todos: &mut Vec<ToDo> ,index: usize, new_text: String){
    if index > 0 && index <= todos.len(){
        todos[index-1].text = new_text.clone();
        println!("Edited to index {},{}", index, new_text);
    }else {
        println!("Unvalid Index");
    }
}

fn list_todos(todos: &Vec<ToDo>){
    if todos.is_empty(){    
        println!("Your todo list is empty.");
        return;
    }

    println!("To do list:");
    for (i,to_do)in todos.iter().enumerate(){
        let status = if to_do.done{"[V]"}else{"[]"};
        println!("{} {} {}",status,i+1,to_do.text)
    }
}

//what is this part do ??
fn save_task(todos: &mut Vec<ToDo>,path: &Path) -> io::Result<()>{
    let json_data = serde_json::to_string_pretty(todos)?;
    fs::write(path, json_data)?;
    Ok(())
}

fn load_todos(path: &Path) -> io::Result<Vec<ToDo>>{
    if path.exists() {
        let json_data = fs::read_to_string(path)?;
        let todos = serde_json::from_str(&json_data)?;
        Ok(todos)
    } else {
        Ok(Vec::new())
    }
}

fn main(){
    let todo_file_path ="todos.json";
    let mut todos = load_todos(Path::new(todo_file_path))
    .unwrap_or_else(|err|{
        eprintln!("Error loading todo:{}", err);
        Vec::new()
    });

    loop{
        println!("\nCommands: add, list(ls), done, remove(rm), edit, save(sv), quit, clear");
        print!(">");
        io::stdout().flush().unwrap();


        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let part: Vec<&str> = input.trim().splitn(2, ' ').collect();
        let command = part[0];

        match command{
            "add" => {
                if part.len() > 1 { 
                    add_todo(&mut todos, part[1].to_string());
                } else {
                    println!("Usage: add <task>");
                }
            }
            "ls" => {
                list_todos(&todos);
            }
            "list" => {
                list_todos(&todos);
            }

            "done" => {
                if part.len() > 1{
                    if let Ok(index) = part[1].parse::<usize>(){
                        mark_done(&mut todos, index);
                    }else {
                        println!("Invalit Index. Plese use the number");
                    }
                } else {
                    println!("Usage: done <index>");
                }
            }
            "rm" => {
                if part.len() > 1{ //
                    if let Ok(index) = part[1].parse::<usize>(){
                        remove_task(&mut todos, index);
                    } else {
                        println!("invalid index, use the number");
                    } 
                } else {
                    println!("Usage: rm <index>");
                }
            }
            "remove" => {
                if part.len() > 1{ //
                    if let Ok(index) = part[1].parse::<usize>(){
                        remove_task(&mut todos, index);
                    } else {
                        println!("invalid index, use the number");
                    } 
                } else {
                    println!("Usage: rm <index>");
                }
            }
            "edit" => {
                if part.len() > 1 {
                    let edit_parts: Vec<&str> = part[1].splitn(2,' ').collect();
                    if edit_parts.len() > 1 {
                        if let Ok(index) = edit_parts[0].parse::<usize>(){
                            let new_text = edit_parts[1].to_string();
                            edit_part(&mut todos, index, new_text);
                        } else {
                            println!("Unvalid index, Please use number");   
                        }
                    } else {
                        println!("Usage: edit <index> <new task>");
                    }
                    
                } else {
                    println!("Usage: edit <index> <new task>");
                }
            }
            "sv" => {
                match save_task(&mut todos, Path::new(todo_file_path)){
                    Ok(_) => println!("To do list save successfully"),
                    Err(e) => eprint!("Error saving todos:{}",e),
                }
            }
            "save" => {
                match save_task(&mut todos, Path::new(todo_file_path)){
                    Ok(_) => println!("To do list save successfully"),
                    Err(e) => eprint!("Error saving todos:{}",e),
                }
            }
            "quit" => {
                println!("Good bye!!");
                break;
            }
            "clear" => {
                clearscreen::clear().expect("Fail to clear screen");
                println!("Screen cleared.");
            }
            _ => {
                println!("Unknown Command!");
            }
        }   
    }    
}
