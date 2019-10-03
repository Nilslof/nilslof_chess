#[derive(Copy, Clone, Debug)]
enum GameState {
    Normal,
    Check,
    CheckMate,
}

impl GameState {}

#[derive(Copy, Clone, Debug)]
pub struct Game {
    pub board: [[Option<Piece>; 8]; 8],
    pub turn: Colour,
    pub possible_pessant: Option<(usize, usize)>,
}

impl Game {
    pub fn new() -> Game {
        let mut new_board: [[Option<Piece>; 8]; 8] = [[Option::None; 8]; 8];

        let rook1: Piece = Piece::new(Colour::White, PieceType::Rook);
        new_board[0][0] = Option::Some(rook1);
        let knight1: Piece = Piece::new(Colour::White, PieceType::Knight);
        new_board[1][0] = Option::Some(knight1);
        let bishop1: Piece = Piece::new(Colour::White, PieceType::Bishop);
        new_board[2][0] = Option::Some(bishop1);
        let queen: Piece = Piece::new(Colour::White, PieceType::Queen);
        new_board[3][0] = Option::Some(queen);
        let king: Piece = Piece::new(Colour::White, PieceType::King);
        new_board[4][0] = Option::Some(king);
        let bishop2: Piece = Piece::new(Colour::White, PieceType::Bishop);
        new_board[5][0] = Option::Some(bishop2);
        let knight2: Piece = Piece::new(Colour::White, PieceType::Knight);
        new_board[6][0] = Option::Some(knight2);
        let rook2: Piece = Piece::new(Colour::White, PieceType::Rook);
        new_board[7][0] = Option::Some(rook2);

        for j in 0..8 {
            let p: Piece = Piece::new(Colour::White, PieceType::Pawn);
            new_board[j][1] = Option::Some(p)
        }
        for i in 2..6 {
            for j in 0..8 {
                new_board[j][i] = Option::None
            }
        }
        for j in 0..8 {
            let p: Piece = Piece::new(Colour::Black, PieceType::Pawn);
            new_board[j][6] = Option::Some(p)
        }

        let rook1: Piece = Piece::new(Colour::Black, PieceType::Rook);
        new_board[0][7] = Option::Some(rook1);
        let knight1: Piece = Piece::new(Colour::Black, PieceType::Knight);
        new_board[1][7] = Option::Some(knight1);
        let bishop1: Piece = Piece::new(Colour::Black, PieceType::Bishop);
        new_board[2][7] = Option::Some(bishop1);
        let queen: Piece = Piece::new(Colour::Black, PieceType::Queen);
        new_board[3][7] = Option::Some(queen);
        let king: Piece = Piece::new(Colour::Black, PieceType::King);
        new_board[4][7] = Option::Some(king);
        let bishop2: Piece = Piece::new(Colour::Black, PieceType::Bishop);
        new_board[5][7] = Option::Some(bishop2);
        let knight2: Piece = Piece::new(Colour::Black, PieceType::Knight);
        new_board[6][7] = Option::Some(knight2);
        let rook2: Piece = Piece::new(Colour::Black, PieceType::Rook);
        new_board[7][7] = Option::Some(rook2);

        Game {
            board: new_board,
            turn: Colour::White,
            possible_pessant: Option::None,
        }
    }

    pub fn get_board(&self) -> [[Option<Piece>; 8]; 8] {
        let b = self.board.clone();
        return b;
    }

    pub fn get_allowed_moves(&mut self, file: usize, rank: usize) -> Vec<(MoveType, usize, usize)> {
        let mut allowed_moves: Vec<(MoveType, usize, usize)> = Vec::new();
        match self.board[file][rank].unwrap().piece_type {
            PieceType::King => {
                for i in 0..3 {
                    for j in 0..3 {
                        if file + 1 >= i
                            && rank + 1 >= j
                            && !((file + 1 - i == 0) && (rank + 1 - j == 0))
                            && 0 <= file + 1 - i  && file + 1 - i <= 7
                            && 0 <= rank + 1 - j  && rank + 1 - j <= 7
                        {
                            if self.can_move(
                                MoveType::Normal,
                                file,
                                rank,
                                file + 1 - i,
                                rank + 1 - j,
                            ) {
                                {
                                    allowed_moves.push((
                                        MoveType::Normal,
                                        (file + 1 - i),
                                        (rank + 1 - j),
                                    ));
                                }
                            }
                        }
                    }
                }
                allowed_moves.append(&mut self.get_castle_moves(file, rank));
            }
            PieceType::Queen => {
                allowed_moves.append(&mut self.get_diagonal_direction_moves(file, rank));
                allowed_moves.append(&mut self.get_straight_direction_moves(file, rank));
            }
            PieceType::Bishop => {
                allowed_moves.append(&mut self.get_diagonal_direction_moves(file, rank));
            }
            PieceType::Knight => {
                for i in 0..8 {
                    for j in 0..8 {
                        let mut file_difference: usize = if i > file { i - file } else { file - i };
                        let mut rank_difference: usize = if j > rank { j - rank } else { rank - j };

                        if (file_difference == 1 && rank_difference == 2)
                            || (file_difference == 2 && rank_difference == 1)
                        {
                            if self.can_move(MoveType::Normal, file, rank, i, j) {
                                allowed_moves.push((MoveType::Normal, i, j));
                            }
                        }
                    }
                }
            }
            PieceType::Rook => {
                allowed_moves.append(&mut self.get_straight_direction_moves(file, rank));
            }
            PieceType::Pawn => {
                let colour = self.board[file][rank].unwrap().colour.clone();
                let mut i: usize = 2;
                match colour {
                    Colour::White => {                    }
                    Colour::Black => {
                        i = 0;
                    }
                }

                println!("the value of i is: {}", i);

                if self.can_move(MoveType::Normal, file, rank, file, rank - 1 + i)
                && self.board[file][rank - 1 + i].is_none(){

                    allowed_moves.push((MoveType::Normal, file, rank - 1 + i));
                    if self.board[file][rank - 1 + i].is_none()
                        && self.can_move(MoveType::Normal, file, rank, file, rank - 1 + i)
                        && rank == (12 - 5*i)/2
                    {
                        allowed_moves.push((MoveType::Normal, file, rank + 2 * i - 2));
                    }
                }

                if file < 7 && self.can_move(MoveType::Normal, file, rank, file + 1, rank - 1 + i) {
                    if self.board[file+1][rank - 1 + i].is_some(){
                        allowed_moves.push((MoveType::Normal, file + 1, rank - 1 + i));
                    }

                }

                if file > 0 && self.can_move(MoveType::Normal, file, rank, file - 1, rank - 1 + i) {
                    if self.board[file-1][rank - 1 + i].is_some(){
                        allowed_moves.push((MoveType::Normal, file - 1, rank - 1 + i));
                    }

                }

                allowed_moves.append(&mut self.get_pessant_move(file, rank));

                allowed_moves.append(&mut self.get_promotion_move(file, rank));
            }
        }
        return allowed_moves;
    }

    pub fn move_piece(
        &mut self,
        move_type: MoveType,
        current_file: usize,
        current_rank: usize,
        new_file: usize,
        new_rank: usize,
    ) {
        let mut p = self.board[current_file][current_rank].unwrap();

        match move_type {
            MoveType::Castle => {
                if new_file == 2 {
                    let p = self.board[0][current_file].unwrap();
                    self.board[0][current_file] = Option::None;
                    self.board[3][new_rank] = Option::None;
                    self.board[3][new_rank] = Option::Some(p);
                } else {
                    let p = self.board[7][current_file].unwrap();
                    self.board[7][current_file] = Option::None;
                    self.board[6][new_rank] = Option::None;
                    self.board[6][new_rank] = Option::Some(p);
                }
                self.possible_pessant = Option::None
            }
            MoveType::Promotion(piece_type) => {
                p.piece_type = piece_type;
                self.possible_pessant = Option::None
            }
            MoveType::Pessant => {
                self.board[self.possible_pessant.unwrap().0][self.possible_pessant.unwrap().1] =
                    Option::None;
                self.possible_pessant = Option::None
            }
            MoveType::Normal => match p.piece_type {
                PieceType::Pawn => {
                    if (current_rank == 1 || current_rank == 6) && (new_rank == 3 || new_rank == 5)
                    {
                        self.possible_pessant = Option::Some((new_file, new_rank));
                    }
                }
                _ => {}
            },
        }

        self.board[current_file][current_rank] = Option::None;
        self.board[new_file][new_rank] = Option::Some(p);
        self.next_turn();
    }

    pub fn get_turn(&self) -> Colour {
        let t = self.turn.clone();
        return t;
    }

    fn can_move(
        &mut self,
        move_type: MoveType,
        file: usize,
        rank: usize,
        new_file: usize,
        new_rank: usize,
    ) -> bool {

        if self.board[file][rank].is_none(){
            return false
        }



        if self.board[new_file][new_rank].is_none(){
            if !self.is_in_check_after_move(move_type, file, rank, new_file, new_rank) {

                return true;

            }
        }
        else{

            if !self.board[new_file][new_rank]
                .unwrap()
                .colour
                .equals(&self.board[file][rank].unwrap().colour){

                if !self.is_in_check_after_move(move_type, file, rank, new_file, new_rank) {
                    return true;
                }
            }
        }
        return false
    }
    //not done
    fn check_win() -> Option<Colour> {
        return Option::None;
    }

    fn next_turn(&mut self) {
        match self.turn {
            Colour::White => {
                self.turn = Colour::Black;
            }
            Colour::Black => {
                self.turn = Colour::White;
            }
        }
    }
    //not done
    fn is_threatened(
        &mut self,
        board: [[Option<Piece>; 8]; 8],
        colour: Colour,
        file: usize,
        rank: usize,
    ) -> bool {
        for i in 0..8 {
            for j in 0..8 {
                if board[i][j].is_some() {
                    match board[i][j].unwrap().piece_type {
                        PieceType::King => {
                            let mut file_difference: usize =
                                if i > file { i - file } else { file - i };
                            let mut rank_difference: usize =
                                if j > rank { j - rank } else { rank - j };
                            if file_difference < 2
                                && rank_difference < 2
                                && !board[i][j].unwrap().colour.equals(&colour)
                            {
                                return true;
                            }
                        }
                        PieceType::Queen => {}
                        PieceType::Bishop => {}
                        PieceType::Knight => {
                            let mut file_difference: usize =
                                if i > file { i - file } else { file - i };
                            let mut rank_difference: usize =
                                if j > rank { j - rank } else { rank - j };

                            if (file_difference == 2 && rank_difference == 1)
                                || (file_difference == 1 && rank_difference == 2)
                                && !board[i][j].unwrap().colour.equals(&colour)
                            {
                                return true;
                            }
                        }
                        PieceType::Rook => {}
                        PieceType::Pawn => match board[i][j].unwrap().colour {
                            Colour::White => {
                                if j + 1 == rank {
                                    let mut file_difference: usize =
                                        if i > file { i - file } else { file - i };
                                    if file_difference == 1
                                        && !board[i][j].unwrap().colour.equals(&colour)
                                    {
                                        return true;
                                    }
                                }
                            }
                            Colour::Black => {
                                if rank + 1 == j {
                                    let mut file_difference: usize =
                                        if i > file { i - file } else { file - i };
                                    if file_difference == 1
                                        && !board[i][j].unwrap().colour.equals(&colour)
                                    {
                                        return true;
                                    }
                                }
                            }
                        },
                    }
                }
            }
        }
        return false;
    }

    fn is_in_check(&mut self, board: [[Option<Piece>; 8]; 8], colour: Colour) -> bool {
        for i in 0..8 {
            for j in 0..8 {
                if self.board[i][j].is_some()
                    && self.board[i][j]
                    .unwrap()
                    .piece_type
                    .equals(&PieceType::King)
                    && self.board[i][j].unwrap().colour.equals(&colour)
                {
                    return self.is_threatened(board, colour, i, j);
                }
            }
        }
        false
    }

    fn is_in_check_after_move(
        &mut self,
        move_type: MoveType,
        current_file: usize,
        current_rank: usize,
        new_file: usize,
        new_rank: usize,
    ) -> bool {
        //  println!("file: {} rank: {}", current_file, current_rank);
        let mut board = self.board.clone();

        let mut p = board[current_file][current_rank].unwrap();

        match move_type {
            MoveType::Castle => {
                if new_file == 2 {
                    let p = board[0][current_file].unwrap();
                    board[0][current_file] = Option::None;
                    board[3][new_rank] = Option::None;
                    board[3][new_rank] = Option::Some(p);
                } else {
                    let p = board[7][current_file].unwrap();
                    board[7][current_file] = Option::None;
                    board[6][new_rank] = Option::None;
                    board[6][new_rank] = Option::Some(p);
                }
            }
            MoveType::Promotion(piece_type) => {
                p.piece_type = piece_type;
            }
            MoveType::Pessant => {
                board[self.possible_pessant.unwrap().0][self.possible_pessant.unwrap().1] =
                    Option::None;
            }
            _ => {}
        }

        board[current_file][current_rank] = Option::None;
        board[new_file][new_rank] = Option::Some(p);

        return self.is_in_check(
            board,
            self.board[current_file][current_rank]
                .unwrap()
                .colour
                .clone(),
        );
    }

    fn get_castle_moves(&mut self, file: usize, rank: usize) -> Vec<(MoveType, usize, usize)> {
        let mut castle_moves: Vec<(MoveType, usize, usize)> = Vec::new();

        let mut r = 0;
        match self.board[file][rank].unwrap().colour {
            Colour::Black => {}
            _ => r = 7,
        }
        if file == 5 && rank == r {
            if self.board[0][r].is_some()
                && !self.board[0][r].unwrap().has_moved
                && !self.board[file][rank].unwrap().has_moved
            {
                for i in 2..4 {
                    if self.board[i][r].is_some() {
                        break;
                    }
                }
                for i in 2..5 {
                    if self.is_threatened(
                        self.board.clone(),
                        self.board[5][r].unwrap().colour.clone(),
                        i,
                        r,
                    ) {
                        break;
                    }
                }
                castle_moves.push((MoveType::Castle, 2, r))
            }
            if self.board[7][r].is_some()
                && !self.board[7][r].unwrap().has_moved
                && !self.board[file][rank].unwrap().has_moved
            {
                for i in 5..7 {
                    if self.board[i][r].is_some() {
                        break;
                    }
                }
                for i in 4..7 {
                    if self.is_threatened(
                        self.board.clone(),
                        self.board[5][r].unwrap().colour.clone(),
                        i,
                        r,
                    ) {
                        break;
                    }
                }
                castle_moves.push((MoveType::Castle, 6, r))
            }
        }
        return castle_moves;
    }

    fn get_pessant_move(&mut self, file: usize, rank: usize) -> Vec<(MoveType, usize, usize)> {
        let mut pessant_move: Vec<(MoveType, usize, usize)> = Vec::new();

        let colour = self.board[file][rank].unwrap().colour.clone();
        let i: usize = 2;
        match colour {
            Colour::White => {}
            Colour::Black => {
                let i: usize = 0;
            }
        }

        if self.possible_pessant.is_some() {
            if self.possible_pessant.unwrap().1 == rank {
                if self.possible_pessant.unwrap().0 == file + 1
                    || self.possible_pessant.unwrap().0 + 1 == file
                {
                    if self.can_move(
                        MoveType::Pessant,
                        file,
                        rank,
                        self.possible_pessant.unwrap().0,
                        rank - 1 + i,
                    ) {
                        pessant_move.push((
                            MoveType::Pessant,
                            self.possible_pessant.unwrap().0,
                            rank - 1 + i,
                        ));
                    }
                }
            }
        }
        return pessant_move;
    }

    fn get_promotion_move(&mut self, file: usize, rank: usize) -> Vec<(MoveType, usize, usize)> {
        let mut promotion_move: Vec<(MoveType, usize, usize)> = Vec::new();

        let colour = self.board[file][rank].unwrap().colour.clone();

        if (rank == 1 && colour.equals(&Colour::Black))
            || (rank == 6 && colour.equals(&Colour::White))
        {
            let new_rank = if colour.equals(&Colour::White) {
                rank + 1
            } else {
                rank - 1
            };

            if self.can_move(
                MoveType::Promotion(PieceType::Queen),
                file,
                rank,
                file,
                new_rank,
            ) {
                promotion_move.push((MoveType::Promotion(PieceType::Queen), file, new_rank));
            }
        }

        return promotion_move;
    }

    fn get_single_direction_straight_moves(
        &mut self,
        colour: Colour,
        file: usize,
        rank: usize,
        direction: usize,
    ) -> Vec<(MoveType, usize, usize)> {
        let mut single_direction_moves: Vec<(MoveType, usize, usize)> = Vec::new();

        let mut f: isize = 0;
        let mut r: isize = 0;
        let i_file = file as isize;
        let i_rank = rank as isize;

        match direction {
            1 => f = -1,
            2 => r = -1,
            3 => f = 1,
            4 => r = 1,
            _ => {}
        }

        for i in 1..(9 - file) {
            let i = i as isize;
            let new_file = (i_file + (i * f)) as usize;
            let new_rank = (i_rank + (i * r)) as usize;

            if new_file < 0 || new_file > 7 {
                continue;
            }
            if new_rank < 0 || new_rank > 7 {
                continue;
            }

            if self.board[new_file][new_rank].is_some() {
                if !self.board[new_file][new_rank]
                    .unwrap()
                    .colour
                    .equals(&colour)
                {
                    if !self.is_in_check_after_move(
                        MoveType::Normal,
                        file,
                        rank,
                        new_file,
                        new_rank,
                    ) {
                        single_direction_moves.push((MoveType::Normal, new_file, new_rank));
                    }
                }
                break;
            } else {
                if !self.is_in_check_after_move(MoveType::Normal, file, rank, new_file, new_rank) {
                    single_direction_moves.push((MoveType::Normal, new_file, new_rank));
                }
            }
        }
        return single_direction_moves;
    }

    fn get_single_direction_diagonal_moves(
        &mut self,
        colour: Colour,
        file: usize,
        rank: usize,
        direction: usize,
    ) -> Vec<(MoveType, usize, usize)> {
        let mut single_direction_moves: Vec<(MoveType, usize, usize)> = Vec::new();
        let mut n: isize = 1;

        let mut f: isize = 0;
        let mut r: isize = 0;
        let i_file = file as isize;
        let i_rank = rank as isize;

        match direction {
            1 => {
                f = -1;
                r = -1
            }
            2 => {
                f = 1;
                r = -1
            }
            3 => {
                f = -1;
                r = 1
            }
            4 => {
                f = 1;
                r = 1
            }
            _ => {}
        }

        while 4 * (f + 1) - f * (i_file + 1 + (n * f)) > 0
            && 4 * (r + 1) - r * (i_rank + (n * r)) > 0
            {
                let new_file = (i_file + (n * f)) as usize;
                let new_rank = (i_rank + (n * r)) as usize;

                if self.board[new_file][new_rank].is_some() {
                    if !self.board[new_file][new_rank]
                        .unwrap()
                        .colour
                        .equals(&colour)
                    {
                        if !self.is_in_check_after_move(
                            MoveType::Normal,
                            file,
                            rank,
                            new_file,
                            new_rank,
                        ) {
                            single_direction_moves.push((MoveType::Normal, new_file, new_rank));
                        }
                    }
                    break;
                } else {
                    if !self.is_in_check_after_move(MoveType::Normal, file, rank, new_file, new_rank) {
                        single_direction_moves.push((MoveType::Normal, new_file, new_rank));
                    }
                }
                n += 1;
            }
        return single_direction_moves;
    }

    fn get_straight_direction_moves(
        &mut self,
        file: usize,
        rank: usize,
    ) -> Vec<(MoveType, usize, usize)> {
        let mut straight_direction_moves: Vec<(MoveType, usize, usize)> = Vec::new();

        let mut colour = self.board[file][rank].unwrap().colour;

        straight_direction_moves
            .append(&mut self.get_single_direction_straight_moves(colour, file, rank, 1));
        straight_direction_moves
            .append(&mut self.get_single_direction_straight_moves(colour, file, rank, 2));
        straight_direction_moves
            .append(&mut self.get_single_direction_straight_moves(colour, file, rank, 3));
        straight_direction_moves
            .append(&mut self.get_single_direction_straight_moves(colour, file, rank, 4));

        return straight_direction_moves;
    }

    fn get_diagonal_direction_moves(
        &mut self,
        file: usize,
        rank: usize,
    ) -> Vec<(MoveType, usize, usize)> {
        let mut diagonal_direction_moves: Vec<(MoveType, usize, usize)> = Vec::new();

        let mut colour = self.board[file][rank].unwrap().colour;

        diagonal_direction_moves
            .append(&mut self.get_single_direction_diagonal_moves(colour, file, rank, 1));
        diagonal_direction_moves
            .append(&mut self.get_single_direction_diagonal_moves(colour, file, rank, 2));
        diagonal_direction_moves
            .append(&mut self.get_single_direction_diagonal_moves(colour, file, rank, 3));
        diagonal_direction_moves
            .append(&mut self.get_single_direction_diagonal_moves(colour, file, rank, 4));

        return diagonal_direction_moves;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MoveType {
    Normal,
    Castle,
    Pessant,
    Promotion(PieceType),
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    colour: Colour,
    pub piece_type: PieceType,
    has_moved: bool,
}

impl Piece {
    fn new(colour: Colour, piece_type: PieceType) -> Piece {
        Piece {
            colour,
            piece_type,
            has_moved: false,
        }
    }

    pub fn get_colour(&self) -> Colour {
        self.colour
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Colour {
    White,
    Black,
}

impl Colour {
    fn equals(&self, colour: &Colour) -> bool {
        match self {
            Colour::White => match colour {
                Colour::White => true,
                _ => false,
            },
            Colour::Black => match colour {
                Colour::Black => true,
                _ => false,
            },
        }
    }

    fn invert(&self) -> Colour {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

impl PieceType {
    pub fn equals(&self, piece_type: &PieceType) -> bool {
        match self {
            PieceType::King => match piece_type {
                PieceType::King => true,
                _ => false,
            },
            PieceType::Queen => match piece_type {
                PieceType::Queen => true,
                _ => false,
            },
            PieceType::Bishop => match piece_type {
                PieceType::Bishop => true,
                _ => false,
            },
            PieceType::Knight => match piece_type {
                PieceType::Knight => true,
                _ => false,
            },
            PieceType::Rook => match piece_type {
                PieceType::Rook => true,
                _ => false,
            },
            PieceType::Pawn => match piece_type {
                PieceType::Pawn => true,
                _ => false,
            },
        }
    }
}

//to be called by GUI
/*

## new -> board
## get board -> board
# get Allowed Moves (position) -> Vec<(moveType, Position)>
## move (moveType, Position, Position) -> board
surrender (Colour)
## get turn -> colour

*/
