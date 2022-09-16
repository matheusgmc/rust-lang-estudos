#[derive(Debug, Clone)]
struct Node {
    data: String,
    previous: Option<Box<Node>>,
}

impl Node {
    fn new(value: String, previous: Option<Box<Node>>) -> Option<Box<Self>> {
        Some(Box::new(Node {
            data: value,
            previous,
        }))
    }
}

#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Node>>,
    size: i32,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            head: None,
            size: 0,
        }
    }

    fn insert_tail(&mut self, value: String) {
        //pode ser usado take() também, porém, torna o self.head = None até ser atribuido novamente pelo new_head
        let old_head = self.head.clone();

        let new_head = Node::new(value, old_head);

        self.head = new_head;
        self.size += 1;
    }

    fn insert_index(&mut self, value: String, index: i32) {
        match &mut self.head {
            None => println!("Lista Vazia"),
            Some(head) => {
                if index > self.size {
                    println!("Sorry, the maximum size is {}", self.size);
                    return;
                }
                // Before é o Node antes do ponto de inserção
                let mut before = head;

                for _ in 0..index - 1 {
                    before = before.previous.as_mut().unwrap();
                }

                //After é o ponto de inserção, no caso o lugar onde
                //sera adicionado o novo Node.
                let after = before.previous.take();

                // Criando o novo Node para assumir o ponto de inserção assim adicionando o after no
                // previous do novo Node
                let new_node = Node::new(value, after);

                //adicionando o novo node no previous no ponto antes da inserção.
                before.previous = new_node;
            }
        }
    }

    fn search(&mut self, index: i32) -> Option<String> {
        match &mut self.head {
            None => {
                println!("Lista Vazia");
                return None;
            }
            Some(head) => {
                if index > self.size {
                    println!("Sorry, the maximum size is {}", self.size);
                    return None;
                }
                // Before é o Node antes do ponto de inserção
                let mut before = Some(head.clone());

                for _ in 0..index {
                    before = before.unwrap().previous;
                }
                return Some(before.unwrap().data);
            }
        }
    }

    fn remove(&mut self, index: i32) {
        match &mut self.head {
            None => println!("Lista Vazia"),
            Some(head) => {
                if index > self.size {
                    println!("Sorry, the maximum size is {}", self.size);
                    return;
                }
                // Before é o Node antes do ponto de remoção
                let mut before = head;

                for _ in 0..index - 1 {
                    before = before.previous.as_mut().unwrap();
                }

                //After é o Node que sera removido, e no seu lugar
                //sera adicionado o seu previous.
                let after = before.previous.take();

                //adicionando o novo node no previous no ponto antes da inserção.
                before.previous = after.unwrap().previous;
            }
        }
    }

    fn show(&mut self) {
        match self.head {
            None => {}
            Some(_) => {
                let mut pointer = self.head.clone().unwrap();
                loop {
                    if pointer.previous.is_none() {
                        println!("{}", pointer.data);
                        break;
                    }
                    print!("{} -> ", pointer.data);
                    pointer = pointer.previous.unwrap();
                }
            }
        }
    }
}

fn main() {
    let mut linked_list = LinkedList::new();
    linked_list.insert_tail("2020".to_string());
    linked_list.insert_tail("2021".to_string());
    linked_list.insert_tail("2023".to_string());

    println!("lista ligada original");
    linked_list.show();

    println!("\nadicionado um novo Node no index 1");
    linked_list.insert_index("2022".to_string(), 1);

    linked_list.show();

    println!("\nbuscando o elemento no index 2");
    let data = linked_list.search(2);
    println!("{:?}", data);

    println!("\nremovendo o elemento no index 1");
    linked_list.remove(1);
    linked_list.show();
}
