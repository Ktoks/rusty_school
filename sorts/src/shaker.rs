
pub fn sort(unsorted: Vec<i32>) -> Vec<i32> {

    let mut sorted: Vec<i32> = unsorted.clone();

    let mut changes: bool = true;

    while changes {
        changes = false;

        for i in 0..sorted.len() -1 {
            if sorted[i] > sorted[i + 1] {
                let temp: i32 = sorted[i];
                sorted[i] = sorted[i + 1];
                sorted[i + 1] = temp;
                changes = true;
            }
        }
        if changes {
            changes = false;
            for i in (0..sorted.len() - 1).rev() {
                if sorted[i] > sorted[i + 1] {
                    let temp: i32 = sorted[i];
                    sorted[i] = sorted[i + 1];
                    sorted[i + 1] = temp;
                    changes = true;
                }
            }
    
        }
    }
    sorted
}