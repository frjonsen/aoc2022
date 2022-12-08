#[derive(Debug, Clone)]
struct SupportFrom(i8, i8);

#[derive(Debug, Eq, PartialEq)]
struct SupportFromAll {
    top: i8,
    left: i8,
    right: i8,
    bottom: i8,
}

fn parse_grid(input: &str) -> Vec<Vec<i8>> {
    // This will create a grid using yx-grid rather than a normal xy-grid,
    // but it makes no difference to the puzzle
    input
        .trim()
        .lines()
        .map(str::trim)
        .map(|line| line.chars().map(|c| c as i8 - 48).collect::<Vec<i8>>())
        .collect()
}

fn count_trees<'a>(current_tree: &i8, tree_direction: impl IntoIterator<Item = &'a i8>) -> u64 {
    let mut seen_trees = 0;
    for tree in tree_direction.into_iter() {
        seen_trees += 1;
        if tree >= current_tree {
            break;
        }
    }

    seen_trees
}

fn find_best_view(grid: &Vec<Vec<i8>>) -> u64 {
    let grid_height = grid.len();
    let grid_width = grid[0].len();
    let mut best_view = 0;

    for y in 1..grid_height {
        for x in 1..grid_width {
            let current_tree_height = &grid[y][x];
            let left_trees = count_trees(current_tree_height, grid[y][..x].iter().rev());
            let right_trees = count_trees(current_tree_height, &grid[y][(x + 1)..]);
            let down_trees = grid.iter().skip(y + 1).map(|c| &c[x]);
            let down_trees = count_trees(current_tree_height, down_trees);
            let above_trees = grid.iter().take(y).rev().map(|c| &c[x]);
            let above_trees = count_trees(current_tree_height, above_trees);

            let current_view = left_trees * right_trees * down_trees * above_trees;
            if current_view > best_view {
                best_view = current_view;
            }
        }
    }

    best_view
}

fn main() {
    let input = include_str!("../../input.txt");
    let grid = parse_grid(input);
    let best_view = find_best_view(&grid);

    println!("{best_view}")
}

#[cfg(test)]
mod tests {
    use crate::{count_trees, find_best_view, parse_grid};

    const SAMPLE_GRID: &'static str = "30373
                                       25512
                                       65332
                                       33549
                                       35390";

    const UNEVEN_SAMPLE: &'static str = "3037
                                         2551
                                         6533
                                         3354
                                         3539";

    #[test]
    fn test_parse_grid() {
        let res = parse_grid(SAMPLE_GRID);
        assert_eq!(5, res[1][1]);
    }

    #[test]
    fn test_parse_uneven_grid() {
        let res = parse_grid(UNEVEN_SAMPLE);
        assert_eq!(1, res[1][3]);
    }

    #[test]
    fn test_find_best_view() {
        let grid = parse_grid(SAMPLE_GRID);
        let best_view = find_best_view(&grid);

        assert_eq!(8, best_view);
    }

    #[test]
    fn test_count_trees() {
        let trees = &[3, 3];
        let seen_trees = count_trees(&5, trees);
        assert_eq!(2, seen_trees);
    }

    #[test]
    fn test_count_trees_blocked() {
        let trees = &[3, 5, 3];
        let seen_trees = count_trees(&5, trees);
        assert_eq!(2, seen_trees);
    }
}
