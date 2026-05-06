fn find_max(nub: &[i32]) -> Option<i32> {
    let mut i: usize = 0;

    if nub.get(0) == None {
        return None
    }
    let mut m = nub[0];
    while nub.get(i) != None {
        if nub[i] >= m {
            m = nub[i];
        } 
        i += 1;
    }
    return Some(m)
}

fn main() {
    let nums1: &[i32] = &[3, 9 ,7, 2, 4];
    let nums_empty: &[i32] = &[];

    match find_max(&nums1) {
    Some(n) => println!("The max number is {}", n),
    None => println!("Empty Input."),
    }

    match find_max(&nums_empty) {
    Some(n) => println!("The max number is {}", n),
    None => println!("Empty Input."),
    }
}