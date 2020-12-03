fn main() {
    let entries = helper::read_file_strings().unwrap();

    part1(&entries);
    part2(&entries);
}

fn part1(entries: &Vec<String>) {
    let hit_trees = get_hit_trees_for_slope(entries, &(3, 1));
    println!("Part 1: {} trees hit.", hit_trees);
}

fn part2(entries: &Vec<String>) {
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let hit_trees_per_slope: usize = slopes
        .iter()
        .map(|s| get_hit_trees_for_slope(entries, s))
        .fold(1, |acc, x| acc * x);

    println!("Part 2: {}.", hit_trees_per_slope);
}

fn get_hit_trees_for_slope(entries: &Vec<String>, diff: &(usize, usize)) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut hit_trees: usize = 0;

    while y < entries.len() {
        let normalized_x = if x >= entries[y].len() {
            x % entries[y].len()
        } else {
            x
        };
        let symbol = entries[y].chars().nth(normalized_x).unwrap();

        if symbol == '#' {
            hit_trees += 1;
        }

        x += diff.0;
        y += diff.1;
    }

    println!("Slope {} {} hit {} trees.", diff.0, diff.1, hit_trees);

    return hit_trees;
}
