
pub fn sort(unsorted: Vec<i32>) -> Vec<i32> {

    if unsorted.len() <= 1 {
        return unsorted;
    }

    let mut sorted = unsorted.clone();

    let split: usize = unsorted.len() / 2;

    let mut lhs: Vec<i32> = unsorted[..split].to_vec();
    let mut rhs: Vec<i32> = unsorted[split..].to_vec();
    lhs = sort(lhs.clone());
    rhs = sort(rhs.clone());
    
    let (mut i, mut j, mut k) = (0, 0, 0);

    // while both work, choose lhs first
    while i < lhs.len() && j < rhs.len() {
        if lhs[i] < rhs[j] {
            sorted[k] = lhs[i];
            k += 1;
            i += 1;
        } else {
            sorted[k] = rhs[j];
            k += 1;
            j += 1;
        }
    }
    // while rhs works
    while j < rhs.len() {
        sorted[k] = rhs[j];
        k += 1;
        j += 1;
    }
    // while lhs works
    while i < lhs.len() {
        sorted[k] = lhs[i];
        k += 1;
        i += 1;
    }

    sorted
}