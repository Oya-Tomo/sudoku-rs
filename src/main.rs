use solver::Solver;

mod solver;

fn main() {
    let problem: Vec<Vec<u8>> = vec![
        vec![8, 0, 4, 7, 6, 0, 1, 0, 0],
        vec![0, 6, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 2, 0, 0, 0, 9],
        vec![0, 0, 0, 0, 0, 8, 0, 1, 0],
        vec![7, 0, 5, 4, 0, 0, 8, 0, 0],
        vec![3, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 6, 0, 0, 0, 0, 0],
        vec![5, 0, 6, 0, 7, 0, 0, 2, 0],
        vec![0, 3, 0, 0, 0, 0, 5, 0, 0],
    ];

    let mut board = Solver::new(problem);
    board.solve();

    for answer in board.answers.iter() {
        for row in answer.iter() {
            println!("{:?}", row);
        }
        println!();
    }
}
