pub fn run() {
    println!("  ├─ Day 2 - I Was Told There Would Be No Math");

    let path = "input/advent_of_code/Y2015/D2.txt";
    let input = std::fs::read_to_string(path).unwrap();

    let boxes: Vec<(usize, usize, usize)> = input
        .lines()
        .map(|line| {
            let dimensions = line
                .split("x")
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (dimensions[0], dimensions[1], dimensions[2])
        })
        .collect();

    let mut wrap_area = 0;
    let mut bow_len = 0;

    boxes.iter().for_each(|&dimensions| {
        let volume = |(l, w, h): (usize, usize, usize)| l * w * h;
        let area = |(l, w, h): (usize, usize, usize)| 2 * l * w + 2 * w * h + 2 * h * l;

        let smallest = |(l, w, h): (usize, usize, usize)| {
            let smallest = if l <= w && l <= h {
                (l, w.min(h))
            } else if w <= h && w <= l {
                (w, l.min(h))
            } else {
                (h, w.min(l))
            };

            (smallest.0, smallest.1)
        };

        let slack = |(s1, s2): (usize, usize)| s1 * s2;
        let side_area = |(s1, s2): (usize, usize)| 2 * s1 + 2 * s2;

        let smallest_dims = smallest(dimensions);

        wrap_area += area(dimensions) + slack(smallest_dims);
        bow_len += volume(dimensions) + side_area(smallest_dims);
    });

    println!("  │  ├─ Part 1: {}", wrap_area);
    println!("  │  └─ Part 2: {}", bow_len);
}
