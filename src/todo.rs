pub struct ToDoList {
    tarefas: Vec<Tarefa>
}

pub struct Tarefa {
    titulo: String,
    descricao: String,
    concluida: bool
}

impl Tarefa {
    pub fn new(titulo: String, descricao: Option<String>) -> Self {
        Tarefa {
            titulo: titulo,
            descricao: match descricao {Some(d) => d, None => "".to_string()},
            concluida: false
        }
    }
    pub fn change_state(&mut self, concluida: bool) {
        self.concluida = concluida;
    }
}

impl ToDoList {
    pub fn new() -> Self {
        ToDoList {tarefas: Vec::new()}
    }
    pub fn add_task(&mut self, task: Tarefa) {
        self.tarefas.push(task)
    }

    pub fn delete_task(&mut self, index: usize) -> Result<(), &str> {
        if (index - 1) > self.tarefas.len() {
            return Err("Não há uma tarefa nesse índice");
        }

        self.tarefas.remove(index-1);
        Ok(())
    }

    pub fn print_tasks(&self) {
        let mut result: String = String::new();
        
        for (i, t) in self.tarefas.iter().enumerate() {
            if t.concluida {
                result.push_str(&format!("{}. ☑ {} | {}\n", i+1, t.titulo, t.descricao));
            }
            else if !t.concluida {
                result.push_str(&format!("{}. ☐ {} | {}\n", i+1, t.titulo, t.descricao));
            }
        }
        println!("{}", result);
    }

    pub fn complete_task(&mut self, index: usize) -> Result<(), &str> {
        if (index - 1) > self.tarefas.len() {
            return Err("Não há uma tarefa nesse índice");
        }

        self.tarefas.get_mut(index - 1).unwrap().change_state(true);

        Ok(())
    }
}