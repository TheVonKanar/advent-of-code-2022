use super::{get_puzzle, Info, Solution, Solver};

pub(super) fn resolve(solver: &Solver, solution: &mut Solution, _: &mut Info) {
    let puzzle = get_puzzle(solver.index);

    let mut curr_path: Vec<String> = Vec::new();
    let mut dirs: Vec<String> = vec!["/".to_string()];
    let mut files: Vec<(String, usize)> = Vec::new();
    let mut used_space: usize = 0;

    for line in puzzle.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts[0] == "$" {
            if parts[1] == "cd" {
                if parts[2] == ".." {
                    curr_path.pop();
                } else {
                    if parts[2] == "/" {
                        curr_path.clear();
                        curr_path.push("/".to_string());
                    }

                    curr_path.push(parts[2].to_string());
                }
            }
        } else {
            let curr_dir = curr_path.join("/");
            if parts[0] == "dir" {
                dirs.push(format!("{}/{}", curr_dir, parts[1]));
            } else {
                let size: usize = parts[0].parse().unwrap();
                files.push((curr_dir, size));
                used_space += size;
            }
        }
    }

    let mut size_0 = 0;
    let mut size_1 = usize::MAX;
    let size_1_target = 30000000 - (70000000 - used_space);

    for dir in dirs {
        let mut size = 0;
        for file in &files {
            if file.0.contains(&dir) {
                size += file.1;
            }
        }

        if size <= 100000 {
            size_0 += size;
        }

        if size >= size_1_target && size < size_1 {
            size_1 = size;
        }
    }

    solution.0 = size_0.to_string();
    solution.1 = size_1.to_string();
}

#[test]
fn test_07() {
    let (solver, mut solution, mut info) = super::create_solver_bundle(7, resolve);
    (solver.resolve)(&solver, &mut solution, &mut info);
    assert_eq!(solution.0, "1770595");
    assert_eq!(solution.1, "2195372");
}
