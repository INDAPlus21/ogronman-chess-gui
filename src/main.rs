use ggez::{conf, event, graphics, ContextBuilder, Context, GameError, GameResult};
use std::{path, env, collections::HashMap};
use alholmbe_chess::{ Game, GameState, Colour, Piece };
use std::{thread, time};


/// A chess board is 8x8 tiles.
const GRID_SIZE: i16 = 8;
/// Sutible size of each tile.
const GRID_CELL_SIZE: (i16, i16) = (90, 90);

/// Size of the application window.
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE as f32 * GRID_CELL_SIZE.1 as f32,
);

// GUI Color representations
const BLACK: graphics::Color = graphics::Color::new(228.0/255.0, 196.0/255.0, 108.0/255.0, 1.0);
const DARKYELLOW: graphics::Color = graphics::Color::new(240.0/255.0, 210.0/255.0, 90.0/255.0, 1.0);
const LIGHTYELLOW: graphics::Color = graphics::Color::new(255.0/255.0, 210.0/255.0, 90.0/255.0, 1.0);
const WHITE: graphics::Color = graphics::Color::new(188.0/255.0, 140.0/255.0, 76.0/255.0, 1.0);

const REALBLACK: graphics::Color = graphics::Color::new(0.0/255.0, 0.0/255.0, 0.0/255.0, 1.0);


/// GUI logic and event implementation structure. 
struct AppState {
    sprites: HashMap<(u8, u8), graphics::Image>,
    board: Vec<Vec<u8>>,
    game: Game,
    turn: u8,
    current_turn: String,
    current_piece: Vec<u8>

    // Save piece positions, which tiles has been clicked, current colour, etc...
}

impl AppState {
    /// Initialise new application, i.e. initialise new game and load resources.
    fn new(ctx: &mut Context) -> GameResult<AppState> {
        
        let state = AppState {
            sprites: AppState::load_sprites(ctx),
            board: Vec::with_capacity(64),
            game: Game::new(),
            turn: 8,
            current_turn: "White".to_string(),
            current_piece: vec![0,0,64],
        };

        Ok(state)
    }

    /// Loads chess piese images into vector.
    fn load_sprites(ctx: &mut Context) -> HashMap<(u8, u8), graphics::Image> {

        [
            ((16, 6), "/black_king.png".to_string()),
            ((16, 5), "/black_queen.png".to_string()),
            ((16, 4), "/black_rook.png".to_string()),
            ((16, 1), "/black_pawn.png".to_string()),
            ((16, 3), "/black_bishop.png".to_string()),
            ((16, 2), "/black_knight.png".to_string()),
            ((8, 6), "/white_king.png".to_string()),
            ((8, 5), "/white_queen.png".to_string()),
            ((8, 4), "/white_rook.png".to_string()),
            ((8, 1), "/white_pawn.png".to_string()),
            ((8, 3), "/white_bishop.png".to_string()),
            ((8, 2), "/white_knight.png".to_string())
        ]
            .iter()
            .map(|(_piece, _path)| {
                (*_piece, graphics::Image::new(ctx, _path).unwrap())
            })
            .collect::<HashMap<(u8, u8), graphics::Image>>()
    }

    fn create_board(&mut self) -> () {

        for _i in 0..64 {
            self.board.push(vec![0,0]);
        }

        self.board[0] = vec![8,4];
        self.board[7] = vec![8,4];
        self.board[1] = vec![8,2];
        self.board[6] = vec![8,2];
        self.board[2] = vec![8,3];
        self.board[5] = vec![8,3];
        self.board[3] = vec![8,5];
        self.board[4] = vec![8,6];

        self.board[8] = vec![8,1];
        self.board[9] = vec![8,1];
        self.board[10] = vec![8,1];
        self.board[11] = vec![8,1];
        self.board[12] = vec![8,1];
        self.board[13] = vec![8,1];
        self.board[14] = vec![8,1];
        self.board[15] = vec![8,1];


        self.board[56] = vec![16,4];
        self.board[63] = vec![16,4];
        self.board[57] = vec![16,2];
        self.board[62] = vec![16,2];
        self.board[58] = vec![16,3];
        self.board[61] = vec![16,3];
        self.board[59] = vec![16,5];
        self.board[60] = vec![16,6];

        self.board[48] = vec![16,1];
        self.board[49] = vec![16,1];
        self.board[50] = vec![16,1];
        self.board[51] = vec![16,1];
        self.board[52] = vec![16,1];
        self.board[53] = vec![16,1];
        self.board[54] = vec![16,1];
        self.board[55] = vec![16,1];

        println!("Board is set up");


    }   

    fn convert_string_vector(&mut self, moves:&Vec<String>) -> Vec<Vec<u8>> {
        let mut int_vector:Vec<Vec<u8>> = Vec::new();
        for _i in 0..moves.len(){
            int_vector.push(self.str_to_u8(moves[_i].clone()));
        }
        return int_vector;
    }

    fn update_board(&mut self, fen:String) -> () { 

        let mut fen_string:String = "".to_string();

        for _c in fen.chars(){
            if _c.is_whitespace(){
                break;
            }else{
                fen_string.push(_c);
            }
        }

        let mut file:usize = 0;
        let mut rank:usize = 7;

        for _i in 0..64 {
            self.board[_i] = vec![0,0];
        }

        for c in fen_string.chars(){
            if c == '/'{
                file = 0;
                rank -= 1;
            }else{
                if c.is_numeric() {
                    let num:u32 = *c.to_digit(10).get_or_insert(0);
                    file += num as usize;
                }else{
                    let mut piece_color = 16;
                    if c.is_uppercase() {
                        piece_color = 8; 
                    }

                    let piece_type = AppState::piece_from_symbol(c.to_ascii_lowercase());
                    self.board[rank*8+file] = vec![piece_color, piece_type];
                    file += 1;
                }
            }
        }

    }

    fn piece_from_symbol(c:char) -> u8 {
        let mut _s = c.to_string();
        _s = _s.chars().map(|_s| match _s {      
            'p' => "1",  
            'n' => "2", 
            'b' => "3",
            'r' => "4",
            'q' => "5",
            'k' => "6",
            _ => "0"
        }).collect();
        let piece:u8 = _s.parse::<u8>().unwrap(); //Gör om bokstäver till siffror som kan motsvara till brädet t.ex.

        return piece;
    }

    fn u8_to_str(num:u8) -> String {

        //println!("{}", num);
        let mut pos_str:String = "".to_string();
        for _rows in 0..8 {
            for _file in 0..8{
                if num == _rows*8+_file {
                    //println!("Rows is {}", _rows);
                    //println!("File is {}", _file);
                    if _file == 0 {
                        pos_str.push_str("a");
                    }else if _file == 1 {
                        pos_str.push_str("b");
                    }else if _file == 2 {
                        pos_str.push_str("c");
                    }else if _file == 3 {
                        pos_str.push_str("d");
                    }else if _file == 4 {
                        pos_str.push_str("e");
                    }else if _file == 5 {
                        pos_str.push_str("f");
                    }else if _file == 6 {
                        pos_str.push_str("g");
                    }else if _file == 7 {
                        pos_str.push_str("h");
                    }   
                    pos_str.push_str(&((_rows+1).to_string()));
                }
            }
        }
        return pos_str;
    }
     
    fn move_piece(&mut self, pos: u8) -> () {
        let from = AppState::u8_to_str(self.current_piece[2]);
        let to = AppState::u8_to_str(pos);
        let mut move_pos:String = "".to_string();
        move_pos.push_str(&from);
        move_pos.push_str(" ");
        move_pos.push_str(&to);
        let testS = from.clone();
        //println!("From posistion {}", from);
        //println!("To posistion {}", to);
        let result = self.game.make_move(from, to);

        

        if  result != None {

            self.board[pos as usize][0] = self.board[self.current_piece[2]as usize][0];
            self.board[pos as usize][1] = self.board[self.current_piece[2] as usize][1];
            self.board[self.current_piece[2]as usize][0] = 0;
            self.board[self.current_piece[2]as usize][1] = 0;

            self.current_piece = vec![0,0,64];
            if(self.turn == 8){
                self.turn = 16;
                self.current_turn = "Black".to_string();
            }else{
                self.turn = 8;
                self.current_turn = "White".to_string();
            }
        }
        //println!("{}",self.game.get_fen());

    }

    fn str_to_u8(&mut self, pos:String) -> Vec<u8> {
        let mut _c:String = pos.chars().nth(0).unwrap().to_string();
        _c = _c.chars().map(|_c| match _c {      
                'a' => "0", 
                'b' => "1", 
                'c' => "2",
                'd' => "3",
                'e' => "4",
                'f' => "5",
                'g' => "6",
                'h' => "7",
                _ => "0"
            }).collect();

        let mut _s:String  = pos.chars().nth(1).unwrap().to_string();

        _s = _s.chars().map(|_s| match _s {      
            '1' => "0", 
            '2' => "1", 
            '3' => "2",
            '4' => "3",
            '5' => "4",
            '6' => "5",
            '7' => "6",
            '8' => "7",
            _ => "0"
        }).collect();
        let _pos1:u8 = _c.parse::<u8>().unwrap();
        let _pos2:u8 = _s.parse::<u8>().unwrap();
        return vec![_pos1,_pos2];
    }

    fn get_square(&mut self, x: f32, y: f32) -> u8 {

        let mut pos:u8 = 64;

        if x < 0.0 || x > SCREEN_SIZE.0 {
            return 0;
        }
        if y < 0.0 || y > SCREEN_SIZE.1 {
            return 0;
        }

        for rows in 0..8 {
            for file in 0..8 {
                if x <= (rows*GRID_CELL_SIZE.0+GRID_CELL_SIZE.0) as f32 && x > (rows*GRID_CELL_SIZE.0) as f32{
                    if y <= (file*GRID_CELL_SIZE.1+GRID_CELL_SIZE.1) as f32 && y > (file*GRID_CELL_SIZE.1) as f32{
                        pos = (file*8+rows) as u8;
                    }
                }
            }
        }
        return pos;
    }

}

impl event::EventHandler<GameError> for AppState {

    /// For updating game logic, which front-end doesn't handle.
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    /// Draw interface, i.e. draw game board
    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        // clear interface with gray background colour
        graphics::clear(ctx, [0.5, 0.5, 0.5, 1.0].into());

        // create text representation
        let state_text = graphics::Text::new(
                graphics::TextFragment::from(format!("     Game is:\n    {:?}.", self.game.get_game_state())
            )
            .scale(graphics::PxScale     { x: 30.0, y: 30.0 }));

        // get size of text
        let text_dimensions = state_text.dimensions(ctx);
        // create background rectangle with white coulouring
        let background_box = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(),
            graphics::Rect::new((SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 as f32 - 8.0,
                                (SCREEN_SIZE.0 - text_dimensions.h as f32) / 2f32 as f32,
                                text_dimensions.w as f32 + 16.0, text_dimensions.h as f32),
                                [1.0, 1.0, 1.0, 1.0].into()
        )?;

        // draw background
        graphics::draw(ctx, &background_box, graphics::DrawParam::default()).expect("Failed to draw background.");

        // draw grid
        for _row in 0..8 {
            for _col in 0..8 {

                // draw tile
                let rectangle = graphics::Mesh::new_rectangle(ctx, 
                    graphics::DrawMode::fill(), 
                    graphics::Rect::new_i32(
                        _col * GRID_CELL_SIZE.0 as i32,
                        _row * GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ), match _col % 2 {
                        0 => 
                            if _row % 2 == 0 { WHITE } 
                            else { BLACK },
                        _ => 
                            if _row % 2 == 0 { BLACK } 
                            else { WHITE },
                    }).expect("Failed to create tile.");
                graphics::draw(ctx, &rectangle, graphics::DrawParam::default()).expect("Failed to draw tiles.");

            }
        }

        // Create line between info and board
        let info_background = graphics::Mesh::new_rectangle(ctx, 
            graphics::DrawMode::fill(), 
            graphics::Rect::new_i32(
                (GRID_CELL_SIZE.0 *8) as i32,
                    0 as i32,
                    300 as i32,
                    (GRID_CELL_SIZE.0 *8) as i32,
                    ), {BLACK}).expect("Failed to create tile.");
            
        // draw line
        graphics::draw(ctx, &info_background, graphics::DrawParam::default()).expect("Failed to draw background.");

        // Create line between info and board
        let board_line = graphics::Mesh::new_rectangle(ctx, 
            graphics::DrawMode::fill(), 
            graphics::Rect::new_i32(
                (GRID_CELL_SIZE.0 *8) as i32,
                0 as i32,
                15 as i32,
                (GRID_CELL_SIZE.0 *8) as i32,
            ), {REALBLACK}).expect("Failed to create tile.");
    
        // draw line
        graphics::draw(ctx, &board_line, graphics::DrawParam::default()).expect("Failed to draw background.");


        // create text representation
        let information_text = graphics::Text::new(
            graphics::TextFragment::from(format!("Information: ")
            )
            .scale(graphics::PxScale     { x: 30.0, y: 30.0 }));

        let text_dimensions_info = information_text.dimensions(ctx);

        let mut turn_text = graphics::Text::new(
            graphics::TextFragment::from(format!("Current player:\n     {}", self.current_turn)
            )
            .scale(graphics::PxScale     { x: 30.0, y: 30.0 }));


        let text_dimensions_turn = turn_text.dimensions(ctx);


        if self.current_piece[2] < 64 {
            let mut possible_moves = self.game.get_possible_moves(AppState::u8_to_str(self.current_piece[2]));
            let draw_pos = self.convert_string_vector(possible_moves.get_or_insert(vec![]));
            for _i in 0..draw_pos.len(){
                let selectedRect = graphics::Mesh::new_rectangle(ctx, 
                    graphics::DrawMode::fill(), 
                    graphics::Rect::new_i32(
                        draw_pos[_i][0] as i32 * GRID_CELL_SIZE.0 as i32,
                        draw_pos[_i][1] as i32 * GRID_CELL_SIZE.1 as i32,
                        GRID_CELL_SIZE.0 as i32,
                        GRID_CELL_SIZE.1 as i32,
                    ), match draw_pos[_i][0] % 2 {
                        0 => 
                            if (draw_pos[_i][1]+1) % 2 == 0 { DARKYELLOW } 
                            else { LIGHTYELLOW },
                        _ => 
                            if (draw_pos[_i][1]+1) % 2 == 0 { LIGHTYELLOW } 
                            else { DARKYELLOW },
                    }).expect("Failed to create tile.");
                    graphics::draw(ctx, &selectedRect, graphics::DrawParam::default()).expect("Failed to draw tiles.");
            }
        }



        for _rows in 0..8 {
            for _file in 0..8 {
            //draw piece
            if self.board[_rows*8 + _file as usize][0] != 0{

                if self.current_piece[2] == (_rows*8 + _file) as u8{
                    graphics::draw(ctx, self.sprites.get(&(self.board[_rows*8 + _file as usize][0], self.board[_rows*8 + _file  as usize][1])).unwrap(), graphics::DrawParam::default()
                    .scale([2.0, 2.0])  // Tile size is 90 pixels, while image sizes are 45 pixels.
                    .dest(
                        [_file as f32 * GRID_CELL_SIZE.0 as f32, _rows as f32 * GRID_CELL_SIZE.1 as f32 - 12 as f32],
                        )
                    ).expect("Failed to draw piece.");
                }else{
                    graphics::draw(ctx, self.sprites.get(&(self.board[_rows*8 + _file as usize][0], self.board[_rows*8 + _file  as usize][1])).unwrap(), graphics::DrawParam::default()
                    .scale([2.0, 2.0])  // Tile size is 90 pixels, while image sizes are 45 pixels.
                    .dest(
                        [_file as f32 * GRID_CELL_SIZE.0 as f32, _rows as f32 * GRID_CELL_SIZE.1 as f32],
                        )
                    ).expect("Failed to draw piece.");
                    }
                }
            }
        }


        // draw text with dark gray colouring and center position
        graphics::draw(ctx, &state_text, graphics::DrawParam::default().color([0.0, 0.0, 0.0, 1.0].into())
            .dest(ggez::mint::Point2 {
                x: ((SCREEN_SIZE.0 - text_dimensions.w as f32) / 2f32 + (SCREEN_SIZE.0 +300.0)/2f32  ) as f32,
                y: (text_dimensions.h as f32+250.0) / 2f32 as f32,
            })).expect("Failed to draw text.");
    
        // draw text with dark gray colouring and center position
        graphics::draw(ctx, &information_text, graphics::DrawParam::default().color([0.0, 0.0, 0.0, 1.0].into())
        .dest(ggez::mint::Point2 {
            x: ((SCREEN_SIZE.0 - text_dimensions_info.w as f32) / 2f32 + (SCREEN_SIZE.0 +300.0)/2f32  ) as f32,
            y: (text_dimensions_info.h as f32+30.0) / 2f32 as f32,
        })).expect("Failed to draw text.");

        // draw text with dark gray colouring and center position
        graphics::draw(ctx, &turn_text, graphics::DrawParam::default().color([0.0, 0.0, 0.0, 1.0].into())
            .dest(ggez::mint::Point2 {
                x: ((SCREEN_SIZE.0 - text_dimensions_turn.w as f32) / 2f32 + (SCREEN_SIZE.0 +300.0)/2f32  ) as f32,
                y: (text_dimensions_turn.h as f32+90.0) / 2f32 as f32,
            })).expect("Failed to draw text.");

        // render updated graphics
        graphics::present(ctx).expect("Failed to update graphics.");

        Ok(())
    }


    /// Update game on mouse click
    fn mouse_button_up_event(&mut self, ctx: &mut Context, button: event::MouseButton, x: f32, y: f32) {
        if button == event::MouseButton::Left {
            let pos = self.get_square(x,y);
            //println!("Clicked piece is: {:#?}",self.board[pos as usize]);
            if self.turn == self.board[pos as usize][0] {
                //println!("Changing selected piece");
                self.current_piece[0] = self.board[pos as usize][0];
                self.current_piece[1] = self.board[pos as usize][1];
                self.current_piece[2] = pos;
                
            }else{
                if self.current_piece[0] != 0 {
                    self.move_piece(pos);
                }
            }

            self.update(ctx);
        }
    }
}



pub fn main() -> GameResult {

    let resource_dir = path::PathBuf::from("./resources");

    let context_builder = ContextBuilder::new("schack", "Oskar")
        .add_resource_path(resource_dir)        // Import image files to GGEZ
        .window_setup(
            conf::WindowSetup::default()  
                .title("Schnack")                // Set window title "Schack"
                .icon("/icon.png")              // Set application icon
        )
        .window_mode(
            conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0+300.0, SCREEN_SIZE.1) // Set window dimensions
                .resizable(false)               // Fixate window size
        ).modules(conf::ModuleConf::default().audio(false));
    let (mut contex, mut event_loop) = context_builder.build().expect("Failed to build context.");
    

    let mut state = AppState::new(&mut contex).expect("Failed to create state.");
    state.create_board();
    event::run(contex, event_loop, state)       // Run window event loop
}