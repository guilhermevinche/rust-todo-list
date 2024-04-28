use chrono::NaiveTime;
use colored::*;
use prettytable::row;
use prettytable::{Cell, Row, Table};
use std::io::{self, Write};

#[derive(Debug)]
pub enum Priority {
    Baixa,
    Media,
    Alta,
}
struct TodoList {
    pub tasks: Vec<(String, NaiveTime, Priority)>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList  {tasks: Vec::new()}
    }

    fn add_task(&mut self, task:String, time: NaiveTime, priority: Priority) {
        self.tasks.push((task, time, priority));
        println!("Tarefa adicionada com sucesso");
    }

    pub fn read_task(&mut self) -> (String, NaiveTime, Priority) {
        let mut input = String::new();

        input.clear();
        print!("Digite a sua tarefa: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let task = input.trim().to_string();

        input.clear();
        print!("Digite a hora da tarefa (formato HH:MM): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let time = NaiveTime::parse_from_str(input.trim(), "%H:%M").unwrap();

        input.clear();
        print!("Escolha a prioridade (Baixa, Media, Alta): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let priority = match input.trim().to_lowercase().as_str() {
            "baixa" => Priority::Baixa,
            "media" => Priority::Media,
            "alta" => Priority::Alta,
            _=> panic!("Prioridade Inválida"),
        };
        (task, time, priority)
    }

    fn list_tasks(&self) {
        let mut table = Table::new();
        table.add_row(row!["#", "tarefa", "Hora", "Prioridade"]);
        for (i, (task, time, priority)) in self.tasks.iter().enumerate() {
            let priority_str = match priority {
                Priority::Baixa => "baixa".green(),
                Priority::Media => "media".yellow(),
                Priority::Alta => "alta".bright_red(),
            };

            table.add_row(Row::new(vec![
                Cell::new(&(i + 1).to_string()),
                Cell::new(task),
                Cell::new(&time.format("%H:%M").to_string()),
                Cell::new(&priority_str.to_string()),
            ]));
        }
        table.printstd();
    }

    fn remove_task(&mut self, index: usize) {
        self.tasks.remove( index - 1 );
    }
}

fn main() {
    // instanciamos o TodoList
    let mut todo_list = TodoList::new();
    let mut input = String::new();

    // nosso programa vai continuar rodando até que a gente saia
    loop {
        input.clear();
        let mut table = Table::new();

        table.set_titles(row!["Menu da aplicação".green().bold()]);
        table.add_row(row!["1. Adicionar tarefa".blue()]);
        table.add_row(row!["2. Listar tarefa".blue()]);
        table.add_row(row!["3. Editar tarefa".blue()]);
        table.add_row(row!["4. Remover tarefa".blue()]);
        table.add_row(row!["5. Sair".blue()]);

        table.set_format(*prettytable::format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        table.printstd();

        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap(); //para o print! ser chamado na hora
        io::stdin().read_line(&mut input).unwrap(); //implementando o input de entrada
        
        let choice: u32 = input.trim().parse().unwrap(); //convertendo a string para um número 32bits

        match choice {
            //se o usuário digitar 1, então o programa vai executar esse bloco
            1 => {
                let (task, time, priority) = todo_list.read_task();
                todo_list.add_task(task, time, priority)
            }

            2 => todo_list.list_tasks(),

            3 => {
                //quando o usuário digitar 3, ele chama o remove_task
                input.clear();
                // o programa pede para o usuário entrar com o valor do índice da task
                print!("Digite o número da tarefa: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                //convertemos o valor de entrada para o usize - o índice do vetor
                let index: usize = input.trim().parse().unwrap();
                //chamamos o método remove_task do impl e executamos a remoção e a organização do vetor
                let (task, time, priority) = todo_list.read_task();

                todo_list.tasks[index - 1] = (task, time, priority)
            }

            4 => {
                input.clear();
                print!("Digite o número da tarefa: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                let index: usize = input.trim().parse().unwrap();
                todo_list.remove_task(index - 1);
            }

            5 => break,
            _ => continue,
        }
    }
}