
pub fn sort(unsorted: Vec<i32>, range: usize) -> Vec<i32> {

    let mut sorted: Vec<i32> = unsorted.clone();

    let mut counted: Vec<i32> = vec![0; range];

    for num in sorted.clone() {
        counted[num as usize] += 1;
    }
    // sorted = std::vec::Vec::with_capacity(size);
    let mut index: usize = 0;

    for i in 0..counted.len() {
        if counted[i] > 0 {
            let temp = counted[i];
            for _ in 0..temp {
                sorted[index] = i as i32;
                index += 1;
            }
        }
    }
    
    sorted
}