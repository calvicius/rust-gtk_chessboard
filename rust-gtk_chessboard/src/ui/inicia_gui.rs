use gtk::prelude::*;
use mut_static::MutStatic;  // https://github.com/tyleo/mut_static  // https://riptutorial.com/rust/example/29321/safe-static-mut-with-mut-static
use rsvg;

use super::dialogos;
#[allow(unused)]
use super::ajedrez;
use super::utils;


#[derive(Clone)]
pub struct Variables {
    pub board_size: i32,
    pub board_flipped: bool,
    pub default_square_size: i32,
    pub drag_source: i16,
    pub casilla_desde: i16,
    pub casilla_hasta: i16,
    pub mouse_x : f64,
    pub mouse_y : f64,
}

#[allow(unused)]
impl Variables {
    pub fn init()-> Variables{
        let board_size = 8;
        let board_flipped = false;
        let default_square_size = 50;
        let drag_source = 999;  //un valor imposible. solo para incializar una casilla nula
        let casilla_desde = 999;
        let casilla_hasta = 999;
        let mouse_x = 0.0;
        let mouse_y = 0.0;
        Variables {
            board_size,
            board_flipped,
            default_square_size,
            drag_source,
            casilla_desde,
            casilla_hasta,
            mouse_x,
            mouse_y,
        }
    }
    pub fn set_casilla_desde(&mut self, value: i16) {
        self.casilla_desde = value
    }
    pub fn get_casilla_desde(&self) -> i16 {
        self.casilla_desde
    }
    pub fn set_casilla_hasta(&mut self, value: i16) {
        self.casilla_hasta = value
    }
    pub fn get_casilla_hasta(&self) -> i16 {
        self.casilla_hasta
    }
    pub fn set_drag_source(&mut self, value: i16) {
        self.drag_source = value
    }
    pub fn get_drag_source(&self) -> i16 {
        self.drag_source
    }
}


lazy_static! {
    static ref VARIABLES: MutStatic<Variables> = MutStatic::new();
}

#[derive(Clone)]
pub struct TableroGrafico {
    pub tablero_g: ajedrez::Tablero,
    pub last_move: (String, String, String, String, String),
}

impl TableroGrafico {
    pub fn init(tablero_g: ajedrez::Tablero) -> TableroGrafico{
        let last_move = ("None".to_string(), 
                        "None".to_string(), 
                        "None".to_string(), 
                        "None".to_string(), 
                        "None".to_string());
        TableroGrafico {
            tablero_g,
            last_move,
        }
    }
}

lazy_static! {
    static ref TABLERO_G: MutStatic<TableroGrafico> = MutStatic::new();
}


pub fn setup_gui(current_board: ajedrez::Tablero) {
    /* iniciamos variables publicas */
    // Llamo a .set en el MutStatic para poner datos dentro de él. Aqui solo lo inicializo.
    let _obj = VARIABLES.set(Variables::init()).unwrap();
    // creamos el interno del tablero grafico
    let _obj = TABLERO_G.set(TableroGrafico::init(current_board)).unwrap();
    
    let glade_src = include_str!("chessboard.ui");
    let builder = gtk::Builder::new_from_string(glade_src);
    
    let window: gtk::Window = builder.get_object("main_window").expect("No se puede abrir la ventana principal");  //gtk_builder_get_object(builder, "main_window");
    // Programamos lo que hacer cuando se pulsa el boton de salida
    window.connect_delete_event(move |_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    
    let open_pgn_item: gtk::MenuItem = builder.get_object("open_pgn_menu_item").expect("No se puede crear el open_pgn_menu_item");
    gtk::WidgetExt::set_tooltip_markup(&open_pgn_item, Some("the implementation is not complete"));
    let weak_window = window.downgrade();
    open_pgn_item.connect_activate(move |mitem| {
        let window = match weak_window.upgrade() {
            Some(window) => window,
            None => return,
        };
        dialogos::open_pgn_callback(mitem, &window);
    });
    
    let go_end_button: gtk::ToolButton = builder.get_object("go_end_button").expect("No se puede crear el go_end_button");
    gtk::WidgetExt::set_tooltip_markup(&go_end_button, Some("Opción no activa"));
    go_end_button.connect_clicked(move |_btn| {
        // llamar a go_end_button_click_callback
    });
    
    let go_next_button: gtk::ToolButton = builder.get_object("go_next_button").expect("No se puede crear el go_next_button");
    gtk::WidgetExt::set_tooltip_markup(&go_next_button, Some("Opción no activa"));
    go_next_button.connect_clicked(move |_btn| {
        // llamar a muestra_tablero::go_next_button_click_callback
    });
    
    let go_back_button: gtk::ToolButton = builder.get_object("go_back_button").expect("No se puede crear el go_back_button");
    gtk::WidgetExt::set_tooltip_markup(&go_back_button, Some("Opción no activa"));
    go_back_button.connect_clicked(move |_btn| {
        // llamar a go_back_button_click_callback
    });
    
    let go_start_button: gtk::ToolButton = builder.get_object("go_start_button").expect("No se puede crear el go_start_button");
    gtk::WidgetExt::set_tooltip_markup(&go_start_button, Some("Opción no activa"));
    go_start_button.connect_clicked(move |_btn| {
        // llamar a go_start_button_click_callback
    });
    
    let board_display: gtk::DrawingArea = builder.get_object("board_drawing_area").expect("No se puede crear el board_drawing_area");
    gtk::WidgetExtManual::add_events(&board_display, 
                gdk::EventMask::POINTER_MOTION_MASK | 
                gdk::EventMask::BUTTON_PRESS_MASK | 
                gdk::EventMask::BUTTON_RELEASE_MASK);
    
    
    board_display.connect_draw(move |widget, ctx| {
        board_draw_callback(widget, ctx);
        Inhibit(false)
    });
    
    board_display.connect_button_press_event ( move |widget, event| {
        board_mouse_down_callback(widget, event);
        Inhibit(false)
    });
    
    board_display.connect_button_release_event(move |widget, event| {
        board_mouse_up_callback(widget, event);
        Inhibit(false)
    });
    
    board_display.connect_motion_notify_event(move |widget, event| {
        board_mouse_move_callback(widget, event);
        Inhibit(false)
    });
    
    let flip_button: gtk::ToolButton = builder.get_object("flip_board_button").expect("No se puede crear el open_pgn_menu_item");
    gtk::WidgetExt::set_tooltip_markup(&flip_button, Some("Da la vuelta al tablero"));
    flip_button.connect_clicked(move |_btn| {
        {
            let mut var = VARIABLES.write().unwrap();
            var.board_flipped = !var.board_flipped;
        }
        board_display.queue_draw();
    });
    
    window.show_all();
}


pub fn board_draw_callback(widget: &gtk::DrawingArea, 
                        ctx: &cairo::Context) {
    // A menos que el ancho / alto del área de dibujo sea exactamente un múltiplo de 8, 
    // habrá algo de espacio sobrante. Queremos que el tablero esté completamente centrado, 
    // por lo que rellenamos la mitad del espacio restante.
    // Confiamos en que el widget sea perfectamente cuadrado en estos cálculos, 
    // ya que esto se cumple mediante su contenedor aspect frame.
    // Esto debería ser divisible por 2 para no dejar un espacio de un píxel.
    let highlight_line_width: f64 = 4.0;
    
    let var; 
    {
        var = VARIABLES.read().unwrap();   // method read is inmutable, method write is mutable
    }
    let tab; 
    {
        tab = TABLERO_G.read().unwrap();
    }
    
    let piezas_svg = utils::load_svgs();
    let square_size = get_square_size(widget);
    let leftover_space = widget.get_allocated_width() - square_size * var.board_size;
    let padding = leftover_space / 2;
    cairo::Context::translate(ctx, padding as f64, padding as f64);
    
    // Color light squares one-by-one
    let tamano = var.board_size;
    let flipped = var.board_flipped;
    
	cairo::Context::set_line_width(ctx, 0.0);
    for file in 0..tamano {
        let x: i32;
        if flipped {
            x = tamano - file - 1;
        }
        else {
            x = file;
        }
        
        for rank in (0..tamano).rev() {
            let y: i32;
            if flipped {
                y = tamano - rank - 1;
            } else {
                y = rank;
            }
            if (y + x) % 2 == 0 {
                // casillas negras
                cairo::Context::set_source_rgb(ctx, 0.450980, 0.537255, 0.713725);
                ctx.rectangle(0.0, 0.0, square_size as f64, square_size as f64);
				ctx.fill();
            }
            else {
                // casillas blancas
                cairo::Context::set_source_rgb(ctx, 0.952941, 0.952941, 0.952941);
                ctx.rectangle(0.0, 0.0, square_size as f64, square_size as f64);
				ctx.fill();
            }
            
            // Highlight the source and target squares of the last move
			let last_move = tab.last_move.clone();
            let sq_orig = var.get_casilla_desde();
            let sq_dest = var.get_casilla_hasta();
            // las casillas del tablero 8x8. Ver el array DIBUJA_CASILLA en utils
            let s = ((x) << 8) | (y);
            
            if sq_orig != 999 && sq_dest != 999 {
                let mov_tab = ((utils::DIBUJA_CASILLA[sq_orig as usize]) << 16) | (utils::DIBUJA_CASILLA[sq_dest as usize]);
                if last_move.0 != "None" &&
                        ( s == (mov_tab) >> 16 || s == (mov_tab) & 0xFFFF ) {
                    cairo::Context::set_source_rgb(ctx, 0.225, 0.26, 0.3505);
                    cairo::Context::set_line_width(ctx, highlight_line_width);
                    cairo::Context::translate(ctx, highlight_line_width / 2.0, highlight_line_width / 2.0);
                    cairo::Context::rectangle(ctx, 0.0, 0.0, square_size as f64 - highlight_line_width,
                            square_size as f64 - highlight_line_width);
                    cairo::Context::stroke(ctx);

                    cairo::Context::set_line_width(ctx, 1.0);
                    cairo::Context::translate(ctx, -highlight_line_width / 2.0, -highlight_line_width / 2.0);
                }
            }
            let clon_piezas = piezas_svg.clone();
            let pieza_interna;
            if !flipped {
                pieza_interna = tab.tablero_g.board_array[utils::COORDS_088[file as usize][rank as usize] as usize];
            }
            else {
                pieza_interna = tab.tablero_g.board_array[utils::COORDS_088_FLIPPED[file as usize][rank as usize] as usize];
            }
            if pieza_interna != 0 {
                draw_piece(&ctx, pieza_interna, square_size, clon_piezas);
            }
            
            cairo::Context::translate(ctx, 0.0, square_size as f64);
        }
        
        cairo::Context::translate(ctx, square_size as f64, (-square_size * var.board_size) as f64);
    }
    //drop(var);
    //{
    let var = VARIABLES.read().unwrap(); 
    if var.drag_source != 999 {
        cairo::Context::identity_matrix(ctx);
        cairo::Context::translate(ctx, padding as f64 + var.mouse_x - (square_size / 2) as f64,
                padding as f64 + var.mouse_y - (square_size / 2) as f64);
        
        let pieza_interna = tab.tablero_g.board_array[var.drag_source as usize];
        
        if pieza_interna != 0 {
            draw_piece(ctx, pieza_interna, square_size, piezas_svg);
        }
    }
    //}
    
}


pub fn board_mouse_down_callback(widget: &gtk::DrawingArea, event: &gdk::EventButton) {
	let e = event;
    
    let tab; 
    {
        tab = TABLERO_G.read().unwrap();   // method read is inmutable, method write is mutable
    }
    
    if e.get_button() == 1 {
        let (x, y) = event.get_position();
        let casilla_088 = board_coords_to_square(widget, x, y);
        
        if tab.tablero_g.board_array[casilla_088 as usize] != 0 {   // hay alguna pieza
            let mut var; 
            {
                var = VARIABLES.write().unwrap();   // method read is inmutable, method write is mutable
            }
            var.drag_source = casilla_088 as i16;
        }
    }
}


// Intenta mover una pieza si actualmente estamos arrastrando una.
pub fn board_mouse_up_callback(widget: &gtk::DrawingArea, event: &gdk::EventButton) {
	let e = event;

    if e.get_button() == 1 {
        let drag_origen: i16; 
        {
            let var = VARIABLES.read().unwrap(); 
            drag_origen = var.drag_source;
        }
        if drag_origen != 999 && drag_origen != -1{
            let (x, y) = e.get_position();
            let drag_target = board_coords_to_square(widget, x, y);
            let drag_source = drag_origen;
            
            // ahora necesitamos convertir las casillas 0x88 a algebraico "a2", "b5"...
            let origen = utils::ALGEBRA[drag_source as usize];
            let destino = utils::ALGEBRA[drag_target as usize];
            
            let mut posic_actual;
            {
                let tab = TABLERO_G.read().unwrap();
                posic_actual = tab.tablero_g.clone();
            }
            let movim = (origen, destino, "Q"); // promote always to queen
            let result = ajedrez::mueve_algebra(&mut posic_actual, movim);
            if result.0 != "None" {
                {
                    let mut obj = TABLERO_G.write().unwrap();
                    obj.tablero_g = posic_actual.clone();
                    obj.last_move = result.clone();
                    let pgn_txt = ajedrez::pgn(&mut posic_actual);
                    println!("\n{}", pgn_txt);
                }
                
                {
                    let mut obj = VARIABLES.write().unwrap();
                    obj.set_casilla_desde(drag_source);
                    let destino = drag_target as i16;
                    obj.set_casilla_hasta(destino);
                }
                
            }
            
            {
                let mut obj = VARIABLES.write().unwrap();
                obj.set_drag_source(999);
            }
            
            widget.queue_draw();
        }
    }
}


// Redraw if we're dragging a piece
pub fn board_mouse_move_callback(widget: &gtk::DrawingArea, event: &gdk::EventMotion) {
	let e = event;
    
    let (x, y) = gdk::EventMotion::get_position(e); // -> (f64, f64)
    
	// e->x and e->y are relative to the window, but we want coordinates
	// relative to the chessboard drawing area.
	// So we figure out where it is, and add that on.
    let padre = gtk::WidgetExt::get_toplevel(widget).unwrap();
    let resultado = gtk::WidgetExt::translate_coordinates(&padre, widget, 0, 0).unwrap();
    
    let mouse_x = x + (resultado.0 * -1) as f64;
    let mouse_y = y + (resultado.1 * -1) as f64;
    
    {
        let mut obj = VARIABLES.write().unwrap(); 
        obj.mouse_x = mouse_x;
        obj.mouse_y = mouse_y;
        if obj.drag_source != 999 {
            widget.queue_draw();
        }
    }
}


// ========= funciones auxiliares ================
pub fn get_square_size(board: &gtk::DrawingArea) -> i32 {
	let width: i32  = board.get_allocated_width();
	let height: i32 = board.get_allocated_height();
    
    let var; 
    {
        var = VARIABLES.read().unwrap(); 
    }
	let max_square_width = width / var.board_size;
	let max_square_height = height / var.board_size;
    
    if max_square_width < max_square_height {
        return max_square_width;
    }
    max_square_height
}

pub fn draw_piece(cr: &cairo::Context, pieza: i16, size: i32, piece_images: [[rsvg::Handle; 6]; 2]) {
    let var; 
    {
        var = VARIABLES.read().unwrap();
    }
    let mut indice_player: usize = 0;
    if pieza < 0 {
        // el jugador es con negras y esta relacionado con los indices de utils::load_svgs()
        indice_player = 0;
    }
    else if pieza > 0 {
        indice_player = 1;
    }
    let mut indice_pieza: usize = 0;
    match pieza.abs() {
        1 => indice_pieza = 5,  // es el rey
        2 => indice_pieza = 4,  // es la dama
        3 => indice_pieza = 3,  // es la torre
        4 => indice_pieza = 2,  // es el alfil
        5 => indice_pieza = 1,  // es el caballo
        6 => indice_pieza = 0,  // es el peon
        _ => (),
    };
    
	let piece_image = &piece_images[indice_player][indice_pieza];

	// 0.025 is a bit of a magic number. It's basically just the factor by
	// which the pieces must be scaled in order to fit correctly with the
	// default square size. We then scale that based on how big the squares
	// actually are.
	let scale: f64 = 0.025 * size as f64 / var.default_square_size as f64;
	cairo::Context::scale(cr, scale, scale);

    rsvg::HandleExt::render_cairo(piece_image, cr);

	cairo::Context::scale(cr, 1.0 / scale, 1.0 / scale);
}

pub fn board_coords_to_square(drawing_area: &gtk::DrawingArea, x: f64, y: f64) -> i32 {
    let var; 
    {
        var = VARIABLES.read().unwrap();
    }
	let square_size = get_square_size(drawing_area);
	let mut board_x = (x / square_size as f64) as i32;
	let mut board_y = (y / square_size as f64) as i32;
    
	if !var.board_flipped {
		board_y = var.board_size - 1 - board_y;
	} else {
		board_x = var.board_size - 1 - board_x;
	}
    
    let casilla_088 = utils::COORDS_088[board_x as usize][board_y as usize];
	
    casilla_088
}