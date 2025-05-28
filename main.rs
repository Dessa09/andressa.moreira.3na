use std::collections::LinkedList;

fn main() {
    //Criar a lista encardeada
    let mut lista: LinkedList<i32> = LinkedList::new();

    // Inserindo elementos desejados a lsita
    lista.push_back(10);
    lista.push_back(20);
    lista.push_front(5);
    println!("Lista após inserções: {:?}", lista);

    // Removendo elementos desejados da lsita
    let removido_inicio = lista.pop_front();
    println!("Removido do início: {:?}", removido_inicio);
    println!("Lista agora: {:?}", lista);


    let removido_fim = lista.pop_back();
    println!("Removido do fim: {:?}", removido_fim);
    println!("Lista final: {:?}", lista);

    // Printar os elementos restantes da lista
    println!("Itens restantes na lista:");
    for item in &lista {
        println!("{}", item);
    }
}
