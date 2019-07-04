// trasladamos las coordenadas del tablero (col, fila) al tablero interno que es 0x88
// la estructura es [col][fila]
pub const COORDS_088: [[i32; 8]; 8] = [
    [0, 16, 32, 48, 64, 80, 96, 112],
    [1, 17, 33, 49, 65, 81, 97, 113],
    [2, 18, 34, 50, 66, 82, 98, 114],
    [3, 19, 35, 51, 67, 83, 99, 115],
    [4, 20, 36, 52, 68, 84, 100, 116],
    [5, 21, 37, 53, 69, 85, 101, 117],
    [6, 22, 38, 54, 70, 86, 102, 118],
    [7, 23, 39, 55, 71, 87, 103, 119],
];

pub const COORDS_088_FLIPPED: [[i32; 8]; 8] = [
    [119, 103, 87, 71, 55, 39, 23, 7],
    [118, 102, 86, 70, 54, 38, 22, 6],
    [117, 101, 85, 69, 53, 37, 21, 5],
    [116, 100, 84, 68, 52, 36, 20, 4],
    [115, 99, 83, 67, 51, 35, 19, 3],
    [114, 98, 82, 66, 50, 34, 18, 2],
    [113, 97, 81, 65, 49, 33, 17, 1],
    [112, 96, 80, 64, 48, 32, 16, 0]
];

// tabla para convertir casilla 0x88 a algebraica
pub const ALGEBRA: [&str; 120] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", "", "", "", "", "", "", "", "",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "", "", "", "", "", "", "", "",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", "", "", "", "", "", "", "", "",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", "", "", "", "", "", "", "", "",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", "", "", "", "", "", "", "", "",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", "", "", "", "", "", "", "", "",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", "", "", "", "", "", "", "", "",
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8"
];

// tabla para convertir casilla 0x88 del tablero interno a las casillas del tablero grafico
// la casilla grafica 8x8 de codifica as: ((columna) << 8) | (fila);
// el movimiento se define como:  move = ((origen) << 16) | (destino); 
// La comparacion para marcar en el tablero grafico las casillas origen y destino es:
// ( s == (move1) >> 16 || s == (move1) & 0xFFFF )
pub const DIBUJA_CASILLA: [i32; 120] = [
    0, 256, 512, 768, 1024, 1280, 1536, 1792, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 257, 513, 769, 1025, 1281, 1537, 1793, 0, 0, 0, 0, 0, 0, 0, 0,
    2, 258, 514, 770, 1026, 1282, 1538, 1794, 0, 0, 0, 0, 0, 0, 0, 0,
    3, 259, 515, 771, 1027, 1283, 1539, 1795, 0, 0, 0, 0, 0, 0, 0, 0,
    4, 260, 516, 772, 1028, 1284, 1540, 1796, 0, 0, 0, 0, 0, 0, 0, 0,
    5, 261, 517, 773, 1029, 1285, 1541, 1797, 0, 0, 0, 0, 0, 0, 0, 0,
    6, 262, 518, 774, 1030, 1286, 1542, 1798, 0, 0, 0, 0, 0, 0, 0, 0,
    7, 263, 519, 775, 1031, 1287, 1543, 1799
];



pub fn load_svgs () -> [[rsvg::Handle; 6]; 2] {
    let arr_piezas: [[rsvg::Handle; 6]; 2] = [
        [
            rsvg::Handle::new_from_file("pieces/merida/b_p.svg").unwrap(),
            rsvg::Handle::new_from_file("pieces/merida/b_n.svg").unwrap(),
            rsvg::Handle::new_from_file("pieces/merida/b_b.svg").unwrap(),
            rsvg::Handle::new_from_file("pieces/merida/b_r.svg").unwrap(),
            rsvg::Handle::new_from_file("pieces/merida/b_q.svg").unwrap(),
            rsvg::Handle::new_from_file("pieces/merida/b_k.svg").unwrap(),
        ],
        [
            rsvg::Handle::new_from_file("pieces/merida/w_p.svg").unwrap(),
            rsvg::Handle::new_from_file("pieces/merida/w_n.svg").unwrap(),
            rsvg::Handle::new_from_file("pieces/merida/w_b.svg").unwrap(),
            rsvg::Handle::new_from_file("pieces/merida/w_r.svg").unwrap(),
            rsvg::Handle::new_from_file("pieces/merida/w_q.svg").unwrap(),
            rsvg::Handle::new_from_file("pieces/merida/w_k.svg").unwrap(),
        ]
    ];
    arr_piezas
}
/*
pub fn load_svgs(dir: &str) {
	//uint len = strlen(dir) + 8; // e.g.: "w_k.svg\0"
	//char str[len];
	let piece_letters[&str; 6] = ["p", "n", "b", "r", "q", "k"];
	let side_letters[&str; 2] = ["b", "w"];

	for i in 0..2 {
		let side: &str = side_letters[i];

		for j in 0..piece_letters.len() {   //(uint j = 0; piece_letters[j] != '\0'; j++) {
			//sprintf(str, "%s%c_%c.svg", dir, side, piece_letters[j]);
            let nombre_pieza = format!("{}{}_{}.svg", dir, side, piece_letters[j]);
			//piece_images[i][j] = rsvg_handle_new_from_file(str, err);
			//if (*err != NULL)
			//	return;
		}
	}
}
*/