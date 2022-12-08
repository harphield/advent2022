use std::fs::File;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), io::Error> {
    let file = File::open("input.txt")?;

    let mut grid_width = 0usize;
    let mut grid: Vec<u32> = vec![];

    for line_r in io::BufReader::new(file).lines() {
        match line_r {
            Ok(line) => {
                if grid_width == 0 {
                    grid_width = line.len();
                }

                let vek = line.chars().collect::<Vec<char>>();
                let mut vek: Vec<u32> = vek.iter().map(|v| v.to_digit(10).unwrap()).collect();
                grid.append(&mut vek);
            }
            Err(_) => break,
        }
    }

    // println!("{:?}", grid);

    let visible_count = grid
        .iter()
        .enumerate()
        .filter(|(i, _v)| is_visible(*i, &grid, &grid_width))
        .count();

    println!("Part 1: {}", visible_count);

    let best_scenic_score = grid
        .iter()
        .enumerate()
        .map(|(i, _v)| scenic_score(i, &grid, &grid_width))
        .max()
        .unwrap();

    println!("Part 2: {}", best_scenic_score);

    Ok(())
}

fn scenic_score(index: usize, grid: &[u32], grid_width: &usize) -> u32 {
    let mut los: Vec<u32> = vec![];

    // look left
    if index % grid_width != 0 {
        let mut look = index - 1;
        let mut los_local = 0;
        loop {
            los_local += 1;
            if grid[look] >= grid[index] || look % grid_width == 0 {
                los.push(los_local);
                break;
            }

            look -= 1;
        }
    } else {
        return 0;
    }

    // look right
    if index < (grid_width - 1) || ((index + 1) - grid_width) % grid_width != 0 {
        let mut look = index + 1;
        let mut los_local = 0;
        loop {
            los_local += 1;
            if grid[look] >= grid[index]
                || (&look >= grid_width && ((look + 1) - grid_width) % grid_width == 0)
            {
                los.push(los_local);
                break;
            }

            look += 1;
        }
    } else {
        return 0;
    }

    // look up
    if &index >= grid_width {
        let mut look = index - grid_width;
        let mut los_local = 0;
        loop {
            los_local += 1;
            if grid[look] >= grid[index] || &look < grid_width {
                los.push(los_local);
                break;
            }

            look -= grid_width;
        }
    } else {
        return 0;
    }

    // look down
    if index < (grid_width * grid_width) - grid_width {
        let mut look = index + grid_width;
        let mut los_local = 0;
        loop {
            los_local += 1;
            if grid[look] >= grid[index] || look >= (grid_width * grid_width) - grid_width {
                los.push(los_local);
                break;
            }

            look += grid_width;
        }
    } else {
        return 0;
    }

    los.iter().product()
}

fn is_visible(index: usize, grid: &[u32], grid_width: &usize) -> bool {
    // edges
    if index % grid_width == 0 ||                           // left edge
        &index < grid_width ||                              // top edge
        index >= (grid_width * grid_width) - grid_width ||  // bottom edge
        ((index + 1) - grid_width) % grid_width == 0
    // right edge
    {
        return true;
    }

    // internal
    // look left
    let mut look = index - 1;
    loop {
        if grid[look] >= grid[index] {
            // we found a blocker
            break;
        }
        if look % grid_width == 0 {
            return true;
        }

        look -= 1;
    }
    // look right
    let mut look = index + 1;
    loop {
        if grid[look] >= grid[index] {
            // we found a blocker
            break;
        }
        if ((look + 1) - grid_width) % grid_width == 0 {
            return true;
        }

        look += 1;
    }
    // look up
    let mut look = index - grid_width;
    loop {
        if grid[look] >= grid[index] {
            // we found a blocker
            break;
        }
        if &look < grid_width {
            return true;
        }

        look -= grid_width;
    }
    // look down
    let mut look = index + grid_width;
    loop {
        if grid[look] >= grid[index] {
            // we found a blocker
            break;
        }
        if look >= (grid_width * grid_width) - grid_width {
            return true;
        }

        look += grid_width;
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::{is_visible, scenic_score};

    #[test]
    fn test_is_visible() {
        let grid_width = 5;
        let grid: Vec<u32> = vec![
            3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
        ];

        // edges
        assert_eq!(is_visible(0, &grid, &grid_width), true);
        assert_eq!(is_visible(4, &grid, &grid_width), true);
        assert_eq!(is_visible(9, &grid, &grid_width), true);
        assert_eq!(is_visible(grid_width - 1, &grid, &grid_width), true);
        assert_eq!(
            is_visible(grid_width * grid_width - 3, &grid, &grid_width),
            true
        );

        // internal
        assert_eq!(is_visible(6, &grid, &grid_width), true);
        assert_eq!(is_visible(7, &grid, &grid_width), true);
        assert_eq!(is_visible(8, &grid, &grid_width), false);
        assert_eq!(is_visible(11, &grid, &grid_width), true);
    }

    #[test]
    fn test_scenic_score() {
        let grid_width = 5;
        let grid: Vec<u32> = vec![
            3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
        ];

        assert_eq!(scenic_score(7, &grid, &grid_width), 4);
        assert_eq!(scenic_score(17, &grid, &grid_width), 8);
    }
}
