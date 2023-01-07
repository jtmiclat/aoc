use std::fs;
pub fn part1(file: String) {
    let values: String = fs::read_to_string(file).expect("Error opening file");
    let dimensions: Vec<&str> = values.split_whitespace().map(|x| x.clone()).collect();
    let mut area: u32 = 0;
    for dims in dimensions {
        let raw_dims: Vec<u32> = dims
            .split("x")
            .map(|x| x.parse().expect("Error parsing dimension"))
            .collect();
        let h = raw_dims[0];
        let w = raw_dims[1];
        let l = raw_dims[2];

        let areas = vec![l * w, w * h, h * l];
        let total_area = 2 * areas[0] + 2 * areas[1] + 2 * areas[2] + *areas.iter().min().unwrap();
        area = area + total_area;
    }
    println!("Total area needed: {:?}", area)
}

pub fn part2(file: String) {
    let values: String = fs::read_to_string(file).expect("Error opening file");
    let dimensions: Vec<&str> = values.split_whitespace().map(|x| x.clone()).collect();
    let mut total_ribbon: u32 = 0;
    for dims in dimensions {
        let raw_dims: Vec<u32> = dims
            .split("x")
            .map(|x| x.parse().expect("Error parsing dimension"))
            .collect();
        let h = raw_dims[0];
        let w = raw_dims[1];
        let l = raw_dims[2];

        let perimeters = vec![l + w, w + h, h + l];
        let ribbon = 2 * (*perimeters.iter().min().unwrap()) + l * w * h;
        total_ribbon += ribbon;
    }
    println!("Total area needed: {:?}", total_ribbon)
}
