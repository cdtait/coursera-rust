pub fn solution1(arr: &mut Vec<i32>) -> i32 {
    let n = arr.len() as i32;

    for i in 0..arr.len() {
        while arr[i] > 0 && arr[i] <= n {
            let pos = (arr[i] - 1) as usize;
            if arr[pos] != arr[i] {
                let temp = arr[pos];
                arr[pos] = arr[i];
                arr[i] = temp;
            } else {
                break;
            }
        }
    }

    for i in 0..arr.len() {
        if arr[i] != (i as i32) + 1 {
            return (i as i32) + 1;
        }
    }

    n + 1
}

