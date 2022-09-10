pub mod post_actions {
    use crate::{
        actions::post::EOF,
        database::{create_post, establish_connection},
        models::Post,
        schema::posts::dsl::{posts, published},
    };
    use diesel::prelude::*;
    use std::{
        env,
        io::{stdin, Read},
    };

    pub fn show() {
        let connection = &mut establish_connection();

        let results = posts
            .filter(published.eq(true))
            .limit(5)
            .load::<Post>(connection)
            .expect("error loading posts");

        if results.len() == 0 {
            println!("Vazio! Utilize a Opção 2 para criar um novo post");
            return;
        }
        for post in results {
            println!("ID: {} - {}", post.id, post.title);
            println!("--------------");
            println!("{}", post.body);
        }
    }

    pub fn create() {
        let connection = &mut establish_connection();

        println!("Titulo do Post: ");
        let mut title = String::new();
        stdin()
            .read_line(&mut title)
            .expect("erro em obter o title");
        let title = &title[..(title.len() - 1)];

        println!(
            "Escreva o conteudo do post {} (pressione {} para finalizar)",
            title, EOF
        );
        let mut body = String::new();
        stdin().read_to_string(&mut body).unwrap();

        let _ = create_post(connection, &title, &body);

        println!("\nCriado com sucesso o post: {}", title);
        return;
    }

    pub fn publish() {
        let connection = &mut establish_connection();

        println!("forneça o Id do post:");
        let mut id = String::new();

        stdin().read_line(&mut id).expect("erro ao obter o id");

        let id = id.trim().parse::<i32>().expect("erro em realizar o parse");
        let _ = diesel::update(posts.find(id))
            .set(published.eq(true))
            .execute(connection)
            .unwrap();
    }
    pub fn show_posts_drafts() {
        let connection = &mut establish_connection();

        let results = posts
            .filter(published.eq(false))
            .limit(5)
            .load::<Post>(connection)
            .expect("error loading posts");

        if results.len() == 0 {
            println!("Vazio! Utilize a Opção 2 para criar um novo post");
            return;
        }
        for post in results {
            println!("ID: {} - {}", post.id, post.title);
            println!("--------------");
            println!("{}\n", post.body);
        }
    }
    pub fn delete() {
        let id = env::args()
            .nth(2)
            .expect("Necessario adicionar um id")
            .parse::<i32>()
            .expect("não foi possivel fazer o parse do argumento");

        let connection = &mut establish_connection();

        let num_deleted = diesel::delete(posts.find(id))
            .execute(connection)
            .expect("não foi possivel deletar o post");

        println!("Deletado {} posts", num_deleted);
    }
}
#[cfg(not(windows))]
pub const EOF: &str = "CTRL+D";
