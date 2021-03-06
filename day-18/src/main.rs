fn main() {
    println!("\nPuzzle:");
    process(PUZZLE_INPUT);
    //println!("\nExample:");
    //process(EXAMPLE_INPUT);
}

fn process(input: &str) {
    println!("input.len(): {}", input.len());
    let mut map = generate_map(input);
    println!("first char: {}", map[0][0]);
    let rows = map.len();
    let columns = map[0].len();
    println!("char size: {} x {}\n", rows, columns);
    print_map(&map, "Initial state");



    let mut odd_map = empty_map(rows, columns);
    for step in 1 ..= 1000 {
        if step % 2 == 1 {
            evolve_map(&map, &mut odd_map);
        } else {
            evolve_map(&odd_map, &mut map);
        }
        let wood = count_kinds(if step % 2 == 0 {&map} else {&odd_map}, '|');
        let lumberyards = count_kinds(if step % 2 == 0 {&map} else {&odd_map}, '#');

        let resources = wood * lumberyards;
        println!("step: {}, resources: {}", step, resources);
    }

    print_map(&map, "After 1000000000 minutes");

    let wood = count_kinds(&map, '|');
    let lumberyards = count_kinds(&map, '#');

    println!("wood: {}, lumberyards: {}", wood, lumberyards);
    println!("total resources: {}", wood * lumberyards);
}

fn evolve_map(map: &Vec<Vec<char>>, new_map: &mut Vec<Vec<char>>) {
    let rows = map.len();
    let columns = map[0].len();
    for i in 0 .. rows {
        for j in 0 .. columns {
            new_map[i][j] = map[i][j];
            match map[i][j] {
                '.' => if search_adjacent(&map, i, j, '|') >= 3 {
                    new_map[i][j] = '|';
                },
                '#' => if search_adjacent(&map, i, j, '#') < 1 || search_adjacent(&map, i, j, '|') < 1 {
                    new_map[i][j] = '.';
                },
                '|' => if search_adjacent(&map, i, j, '#') >= 3 {
                    new_map[i][j] = '#';
                },
                _ => unreachable!()
            }
        }
    }
}

fn count_kinds(map: &Vec<Vec<char>>, kind: char) -> usize {
    let rows = map.len();
    let columns = map[0].len();
    let mut total = 0;
    for i in 0 .. rows {
        for j in 0 .. columns {
            if map[i][j] == kind {
                total += 1;
            }
        }
    }
    total
}

fn print_map(map: &Vec<Vec<char>>, text: &str) {
    println!("{}: ", text);
    map.iter()
        .map(|vec| vec.iter().collect::<String>())
        .for_each(|string| println!("{}", string));
    println!("");
}

fn search_adjacent(map: &Vec<Vec<char>>, i: usize, j: usize, searched: char) -> usize {
    let rows = map.len();
    let columns = map[0].len();
    let mut found = 0;
    if i > 0 && j > 0 && map[i - 1][j - 1] == searched { found += 1; }
    if i > 0 && map[i - 1][j + 0] == searched { found += 1; }
    if i > 0 && j < (columns - 1) && map[i - 1][j + 1] == searched { found += 1; }

    if j > 0 && map[i + 0][j - 1] == searched { found += 1; }
    //if map[i + 0][j + 0] == searched { found += 1; }
    if j < (columns - 1) && map[i + 0][j + 1] == searched { found += 1; }

    if i < (rows - 1) && j > 0 && map[i + 1][j - 1] == searched { found += 1; }
    if i < (rows - 1) && map[i + 1][j + 0] == searched { found += 1; }
    if i < (rows - 1) && j < (columns - 1) && map[i + 1][j + 1] == searched { found += 1; }

    found
}

fn empty_map(rows: usize, columns: usize) -> Vec<Vec<char>> {
    let mut map = Vec::new();
    for i in 0 .. rows {
        map.push(Vec::new());
        for j in 0 .. columns {
            map[i].push('.');
        }
    }
    map
}

fn generate_map(input: &str) -> Vec<Vec<char>> {
    input.split('\n').map(|string| string.chars().collect::<Vec<_>>()).collect::<Vec<_>>()
}

const EXAMPLE_INPUT: &'static str = 
".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

const PUZZLE_INPUT: &'static str =
".....#.#.####..|.##|...##.#.|.#|#..|#..||.#.#.#.|.
....#||..#.......|.#|#..|.#.|.#|..#.....##........
.#....#.##.|#....|..||#.....|..|.|.||......#|#|..#
||||||..|..#|.####.|.||#.|.|.....#..|.#.||..|...#.
#.....||#||....#|.#..#.#....|...#...||.||.....||..
...|....#........|......|.##||#.###|.#|#....|#||.|
#..|||...#....#..|##...|......#.....#.##..||..#...
.#..|....|.#.||.||#....#|#.|.||#..#..|..##|||..#||
|.....#......##....|#|.#.........|.|..|#.#..##....
...|#.#.#.....||#..|...##|#|#|.......#.....|.###..
..|....#......##...#|.|.||....|.|.||.##..|##....|.
...##....#...||......|..|....|..#.|.#....|.|#.|.|.
|.#.#...|.#|..##....##.......#....#....#...|.#.||.
.|......##..#.|.....###.|.#....#...#.|.|...#|.|...
..#.||.#....||....#....|...|.....#|.......|..#....
.|#....|..|.|..|....#..#...|....|..#|...|.|.....#.
|.#.||...|...|.##|.|.#.|##.........#......|...||..
||.#|..#..||..#......||....|..............#.......
#||...#.|#|...###...#.#..||.|...#||##......|##..|#
.#.....#..|.||..#.#...||.#..|#.......####..##|...#
#....#...|..#|..##.....||..|.##.#.|..#|...#.......
|..|...##.#..||.#...|#|#......|#...#.|.......|#|..
.#|.####|...|.#|...#.#....|#..#.##|#.|.|.#|#....##
..|..|.#||.|...|...##|..|.....#||||.#|.#.|...#....
#....#|...#...#....#..#..#.||..##.|||....||.#...|.
||..|..#.#.||.#|..#|#..#...#....#..#...|..##......
..#|.....|##|........#..#...#...#...#|..|||..#..|#
|..#..||...#.##.#.#..#..#...#....#..|#|#|..|.#.|.#
#||||#.#...#....|..|.|.|...|..........#....#|.|.#|
|....#|#.|.#|#..#|####...|.......|.||....|##..|.#.
||.||#......##.|...#...|.|#.......|..#|#|....|..|.
...#.#.|...##|#....|....|#.#.|....||#....#........
..|...#.|..|#...#..#...|#.|#.|#...#....|#.||#...|.
.|.#.#..|....###|.#.|...|....#.#.....#.|.#..|.#|..
|#|#.#.#..||.#..|.|.##..|#|...#..##|#|.|##|..|..||
...#..#...|.|...#..#.|#..|....|#..|..|.|...#......
.|||##|.....#.|#|.|....#..|.##...#||..|....#......
.#|..##.......#|..|.|.|...|.#....#.|..#...#.|...#.
...|...|....|.#..##|#.#.......|#...##|.##.#..||#..
.#.|........|.........#..|.|..|#.............||...
..|.|..|#..|..|#.|.##.|.|.|#||.||.|#.....#...#.|..
..|#|....##||...||.||..#..#|#....#.##.#||#...|||..
.##.#...#....|.|.#||....##...#.|..|..|#.|.#|#.|#..
#..|.||....#..#..#|#.||#....|..##...#.....##.|.#.#
#...|#.....#.#.|..|..|...#...##...#|#.|...##...|..
..||.|......#|#....|....||#|..|....#...##|..|..|..
.|#...#..|................#...#|#....|..|......#||
....|#...||#...#...|#....|||.#..|#...|#..##.##|.|.
##.#|..#|||...#.|.|.#......||.#.#..|..#.##..#.||.#
....|....|.###|||...|##|.#.#|||..|......|..|..||.|";