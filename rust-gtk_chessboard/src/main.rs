/*
    This program is a clon of https://github.com/orodley/chessboard ,
    but written in rust.
    
    The internal logic of chess is based on https://github.com/calvicius/calvichess
    
    It is done for rust-gtk learning purposes only
*/

extern crate gtk;
extern crate rsvg;
extern crate cairo;
#[macro_use]
extern crate lazy_static;
extern crate mut_static;

use std::process;
use std::env;

mod ui;


use ui::inicia_gui;
#[allow(unused)]
use ui::ajedrez;



fn main() {
    if gtk::init().is_err() {
        eprintln!("No se ha podido iniciar la aplicacion GTK");
        process::exit(1);
    }
    
    let mut current_board: ajedrez::Tablero = ajedrez::Tablero::init();;
    
    if env::args().len() > 1 {
        // suponemos que hemos pasado una partida PGN como parametro
        // TODO all
    }
    else {
        let fen_valida = ajedrez::setup_inicio(&mut current_board);
        if !fen_valida {
            eprintln!("La FEN de inicio no es válida");
            process::exit(1);
        }
    }
    
    inicia_gui::setup_gui(current_board);
    gtk::main();
}
