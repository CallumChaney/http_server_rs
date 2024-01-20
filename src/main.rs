use server::{Route, Server};

mod response;
mod server;

const HOST: &str = "127.0.0.1";
const PORT: &str = "7878";

fn main() -> std::io::Result<()> {
    let mut server = Server::new(HOST, PORT);
    server
        .regester_route(Route::new_get("/", "index.html"))
        .unwrap();
    server.listen();
    Ok(())
}
