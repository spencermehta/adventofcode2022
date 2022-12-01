fn main() {
    //let input = include_str!("test_input.txt");
    let input = include_str!("input.txt");
    let split_input = input.lines();
    let mut calorie_lists: Vec<Vec<isize>> = vec![];
    calorie_lists.push(vec![]);
    for inp in split_input {
        if inp.is_empty() {
            calorie_lists.push(vec![]);
            continue;
        }
        let len = calorie_lists.len();
        calorie_lists[len - 1].push(inp.parse().unwrap());
    }

    let mc = get_calories_for_max_n_elves(&calorie_lists, 3);
    println!("{:?}", mc);
}

fn get_calories_for_max_n_elves(elfs_items: &[Vec<isize>], n: usize) -> isize {
    let mut calories = get_all_calories(elfs_items);
    calories.sort_by(|a, b| b.cmp(a));
    calories.truncate(n);
    calories.iter().sum()
}

fn get_all_calories(elfs_items: &[Vec<isize>]) -> Vec<isize> {
    elfs_items.iter().map(|x| sum_indiv_calories(x)).collect()
}

fn sum_indiv_calories(items: &[isize]) -> isize {
    items.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn sums_indiv_calories() {
        let items = vec![1000, 2000, 3000];
        assert_eq!(6000, sum_indiv_calories(&items));

        let items = vec![4000];
        assert_eq!(4000, sum_indiv_calories(&items));

        let items = vec![5000, 6000];
        assert_eq!(11000, sum_indiv_calories(&items));

        let items = vec![7000, 8000, 9000];
        assert_eq!(24000, sum_indiv_calories(&items));
    }

    #[test]
    fn sums_all_calories() {
        let all_items = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
        ];

        assert_eq!(vec![6000, 4000, 11000, 24000], get_all_calories(&all_items));
    }

    #[test]
    fn gets_calories() {
        let all_items = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
        ];

        assert_eq!(24000, get_calories_for_max_n_elves(&all_items, 1));
    }
}
