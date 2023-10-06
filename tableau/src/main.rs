fn paires (tab: &Vec<u32>) -> Vec<u32> {
    // tab.iter().filter
    let mut tmp: Vec<u32> = Vec::new();
    for y in tab.iter() {
      if y % 2 == 0 {
            tmp.push(*y);
        }
    }
    return tmp;
}

fn find_even(tab: &Vec<u32>) -> Vec<u32> {
    let a = 3;
    tab.iter()
        .filter(|&x| (x + a) % 2 == 0)
        .copied()
        .collect::<Vec<u32>>()
}

fn test(){
    
}

fn main() {
    let tab: Vec<u32> = (0..1_00).collect();
    let paires: Vec<u32> = find_even(&tab);
        println!("{:?}", paires);
}
