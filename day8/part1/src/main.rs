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
        .map(|line| line.chars().map(|h| h as i8 - 48).collect::<Vec<i8>>())
        .collect()
}

fn build_max_supported_grid(grid: &Vec<Vec<i8>>) -> Vec<Vec<SupportFromAll>> {
    let mut max_supported_grid_top_left: Vec<Vec<SupportFrom>> = Vec::new();
    for (y, line) in grid.iter().enumerate() {
        let mut max_supported_line = Vec::<SupportFrom>::new();
        for (x, _) in line.iter().enumerate() {
            let support_from_x = if x == 0 {
                -1
            } else {
                let tree_left = &grid[y][x - 1];
                let support_at_left = &max_supported_line[x - 1].1;
                *tree_left.max(support_at_left)
            };
            let support_from_y = if y == 0 {
                -1
            } else {
                let tree_above = &grid[y - 1][x];
                let support_at_above = &max_supported_grid_top_left[y - 1][x].0;
                *tree_above.max(support_at_above)
            };
            max_supported_line.push(SupportFrom(support_from_y, support_from_x));
        }
        max_supported_grid_top_left.push(max_supported_line);
    }

    let grid_width = grid[0].len();
    let grid_height = grid.len();
    let mut max_supported_grid_bottom_right: Vec<Vec<SupportFrom>> = (0..grid_height)
        .map(|_| (0..grid_height).map(|_| SupportFrom(0, 0)).collect())
        .collect();
    for (y, line) in grid.iter().rev().enumerate() {
        for (x, _) in line.iter().rev().enumerate() {
            let d_x = grid_width - x - 1;
            let d_y = grid_height - y - 1;
            let support_from_x = if x == 0 {
                -1
            } else {
                let right_tree = &grid[d_y][d_x + 1];
                let support_at_right = &max_supported_grid_bottom_right[d_y][d_x + 1].1;
                *right_tree.max(support_at_right)
            };
            let support_from_y = if y == 0 {
                -1
            } else {
                let tree_below = &grid[d_y + 1][d_x];
                let support_at_below = &max_supported_grid_bottom_right[d_y + 1][d_x].0;
                *tree_below.max(support_at_below)
            };
            let max_supported_line = &mut max_supported_grid_bottom_right[d_y];
            max_supported_line[d_x] = SupportFrom(support_from_y, support_from_x);
        }
    }

    let mut support_from_all = Vec::<Vec<SupportFromAll>>::new();
    for y in 0..grid_height {
        let mut line = Vec::<SupportFromAll>::new();
        for x in 0..grid_width {
            let top_left = &max_supported_grid_top_left[y][x];
            let bottom_right = &max_supported_grid_bottom_right[y][x];
            let support = SupportFromAll {
                left: top_left.1,
                top: top_left.0,
                bottom: bottom_right.0,
                right: bottom_right.1,
            };
            line.push(support);
        }
        support_from_all.push(line);
    }

    support_from_all
}

fn calc_visible_trees(input: &str) -> usize {
    let grid = parse_grid(input);
    let support_grid = build_max_supported_grid(&grid);

    let grid_height = grid.len();
    let grid_width = grid[0].len();

    let mut visible_trees = 0;
    for y in 0..grid_height {
        for x in 0..grid_width {
            let current_point = &grid[y][x];
            let support_at_point = &support_grid[y][x];
            if [
                support_at_point.bottom,
                support_at_point.left,
                support_at_point.top,
                support_at_point.right,
            ]
            .iter()
            .min()
            .unwrap()
                < current_point
            {
                visible_trees += 1;
            }
        }
    }

    visible_trees
}

fn main() {
    let input = include_str!("../../input.txt");
    let visible_trees = calc_visible_trees(input);

    println!("{visible_trees}")
}

#[cfg(test)]
mod tests {
    use crate::{build_max_supported_grid, calc_visible_trees, parse_grid, SupportFromAll};

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
    fn test_build_max_support_grid_uneven() {
        let res = parse_grid(UNEVEN_SAMPLE);
        let supported_grid = build_max_supported_grid(&res);

        let point = &supported_grid[1][1];
        assert_eq!(
            SupportFromAll {
                left: 2,
                top: 0,
                right: 5,
                bottom: 5
            },
            *point
        );
    }

    #[test]
    fn test_build_max_supported_grid() {
        let res = parse_grid(SAMPLE_GRID);
        let supported_grid = build_max_supported_grid(&res);

        let point = &supported_grid[0][1];
        assert_eq!(
            SupportFromAll {
                left: 3,
                top: -1,
                right: 7,
                bottom: 5
            },
            *point
        );

        let point = &supported_grid[2][2];
        assert_eq!(
            SupportFromAll {
                left: 6,
                top: 5,
                bottom: 5,
                right: 3
            },
            *point
        );
    }

    #[test]
    fn test_calc_trees() {
        let visible_trees = calc_visible_trees(SAMPLE_GRID);
        assert_eq!(21, visible_trees);
    }
}
