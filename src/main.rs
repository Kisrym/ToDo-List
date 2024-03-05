mod todo;
use todo::{ToDoList, Tarefa};
use std::io;

fn main() {
    println!("============= To-do List =============");
    let mut lista_tarefas: ToDoList = ToDoList::new();

    loop {
        println!("Escolha a opção:\n[ 1 ] Mostrar tarefas\n[ 2 ] Adicionar tarefa\n[ 3 ] Deletar tarefa\n[ 4 ] Completar tarefa\n[ 5 ] Sair");

        let mut input: String = String::new();

        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse::<u8>().unwrap_or_else(|erro| {
            println!("Número inteiro inválido ({})", erro);
            10
        }) {
            1 => lista_tarefas.print_tasks(),
            2 => {
                println!("Qual é o título da tarefa?");
                let mut titulo = String::new();
                io::stdin().read_line(&mut titulo).unwrap();

                println!("Qual é a descrição da tarefa? (Não obrigatório)");
                let mut descricao = String::new();
                io::stdin().read_line(&mut descricao).unwrap();

                lista_tarefas.add_task(Tarefa::new(titulo.trim().to_string(), if &descricao != "" {Some(descricao.trim().to_string())} else {None}));
            }
            3 => {
                lista_tarefas.print_tasks();
                println!("Qual tarefa você quer deletar?");
                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();

                match lista_tarefas.delete_task(index.trim().parse::<u8>().unwrap() as usize) {
                    Ok(()) => println!("Tarefa deletada com sucesso!"),
                    Err(p) => println!("Erro ao deletar a tarefa: {}", p)
                    };
            }
            4 => {
                lista_tarefas.print_tasks();
                println!("Qual tarefa você completou?");
                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();

                match lista_tarefas.complete_task(index.trim().parse::<u8>().unwrap() as usize) {
                    Ok(()) => println!("Tarefa completada com sucesso!"),
                    Err(p) => println!("Erro ao completar a tarefa: {}", p)
                    };
            }
            5 => break,
            _ => ()
        }
    }
}