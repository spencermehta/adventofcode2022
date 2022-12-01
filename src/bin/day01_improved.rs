fn main() {
    let input = include_str!("input.txt");

    let elves = input.split("\n\n");

    let mut elf_sums: Vec<usize> = elves
        .map(|elf| {
            elf.split('\n')
                .flat_map(|num| num.parse::<usize>())
                .sum::<usize>()
        })
        .collect();

    elf_sums.sort_by(|a, b| b.cmp(a));

    println!("{}", elf_sums.iter().take(3).sum::<usize>());
}
