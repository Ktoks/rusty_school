
pub fn sort(unsorted: Vec<i32>, low: usize, high: usize) -> Vec<i32> {
    if high - low <= 0 {
        return unsorted;
    }
    let mut sorted: Vec<i32> = unsorted.clone();

    let mut least_most_gt: usize = low + 1;

    for i in low..high {
        
        if sorted[i] < sorted[low] {
            let temp = sorted[i];
            sorted[i] = sorted[least_most_gt];
            sorted[least_most_gt] = temp;
            least_most_gt += 1;
        } 
    }
    let pivot: usize = least_most_gt - 1;
    
    let temp = sorted[low];
    sorted[low] = sorted[pivot];
    sorted[pivot] = temp;

    sorted = sort(sorted, low, pivot);
    sorted = sort(sorted, pivot + 1, high);
    
    sorted
}