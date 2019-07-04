use gtk::prelude::*;


// Carga una partida nueva desde un fichero PGN
// No está impentado. Falta ller el fichero y procesarlo
pub fn open_pgn_callback(_file: &gtk::MenuItem , win_padre: &gtk::Window ) {
    let dialog = gtk::FileChooserDialog::new(Some("Abrir PGN"), 
                Some(win_padre), 
                gtk::FileChooserAction::Open);
    dialog.add_buttons(&[
                ("Abrir", gtk::ResponseType::Ok.into()),
                ("Cancelar", gtk::ResponseType::Cancel.into())
            ]);
    let mut file = None; 
    // añadir esto si queremos un filtro
    let filter = gtk::FileFilter::new();
    //filter.add_mime_type("chess/pgn");
    filter.add_pattern("*.pgn");
    filter.set_name("PGN chess file");
    dialog.add_filter(&filter);
    
    dialog.set_current_folder_uri("./partidas_pgn/"); 
    
    let result = dialog.run();
    if result == gtk::ResponseType::Ok.into() {
        file = dialog.get_filename();
    }
    
    match file {
        Some(file) => {
                    println!("{:?}", file);
                    // TODO abrir convertir la partida pgn a "current_game"
                    },
        None => {println!("None");},
    }
    dialog.destroy();
}
