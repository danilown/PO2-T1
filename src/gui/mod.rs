use gtk;
use gtk::prelude::*;

pub fn launch () {

    gtk::init().unwrap_or_else( |_| panic!("Não iniciou gtk!"));

    let builder = gtk::Builder::new_from_string(include_str!("po2.ui"));

    let janela :gtk::Window = builder.get_object("Janela").unwrap();

    janela.show_all();

    janela.connect_delete_event(|_, _| { // trata o fechamento da janela
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
