use std::io::Read;

use dorpsgek::Tune;
use dorpsgek_movegen::Board;
use revad::tape::Tape;

fn main() {
    let mut weights = [0.0; 780];
    let mut m_t = [0.0; 780];
    let mut v_t = [0.0; 780];

    weights[0] = 100.0;
    weights[1] = 300.0;
    weights[2] = 300.0;
    weights[3] = 500.0;
    weights[4] = 900.0;

    weights[6] = 100.0;
    weights[7] = 300.0;
    weights[8] = 300.0;
    weights[9] = 500.0;
    weights[10] = 900.0;

    let boards = {
        let mut boards = Vec::new();
        let mut s = String::new();
        let mut f = std::fs::File::open("ccrl4040_shuffled_5M.epd").unwrap();
        f.read_to_string(&mut s).unwrap();

        for line in s.lines() {
            boards.push(Board::from_fen(line).unwrap());
        }
        boards
    };

    for epoch in 0..500 {
        let tape = Tape::new();
        let mut tune = Tune::new(&tape);
        tune.set_state(&tape, &weights, &m_t, &v_t);

        tune.tune(&tape, &boards, epoch);

        if epoch % 10 == 0 {
            tune.dump();
        }

        let s = tune.get_state();
        weights = s.0;
        m_t = s.1;
        v_t = s.2;
    }
}
