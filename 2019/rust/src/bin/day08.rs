const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn count(layer: &[u32], pixel: u32) -> usize {
    layer.iter().filter(|&&p| p == pixel).count()
}

fn color(image: &[u32], pos: usize) -> u32 {
    *image
        .iter()
        .skip(pos)
        .step_by(WIDTH * HEIGHT)
        .filter(|&&p| p != 2)
        .next()
        .unwrap()
}

fn main() {
    let image: Vec<u32> = include_str!("../../../in/day08.in")
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let best_layer = image
        .chunks(WIDTH * HEIGHT)
        .min_by_key(|l| count(l, 0))
        .unwrap();

    println!("Part 1: {}", count(best_layer, 1) * count(best_layer, 2));

    let pixels: Vec<char> = (0..(WIDTH * HEIGHT))
        .map(|pos| color(&image, pos))
        .map(|n| if n == 0 { ' ' } else { '#' })
        .collect();

    println!("Part 2:");
    pixels
        .chunks(WIDTH)
        .map(|c| c.iter().collect::<String>())
        .for_each(|c| println!("{:?}", c));
}
