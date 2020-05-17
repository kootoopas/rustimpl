fn main() {
    const ARRAY: [i8; 8] = [3i8, 5i8, -4i8, 8i8, 11i8, 1i8, -1i8, 6i8];
    let p: (i8, i8) = solution(&ARRAY, 10i8);
    println!("({}, {})", p.0, p.1);
    assert!(p == (11, -1));
    
}

fn solution(array: &[i8], sum: i8) -> (i8, i8) {
    for n in 0..=array.len() - 1 {
        for m in 0..=array.len() - 1 {
            if n == m {
                continue
            }
            if array[n] + array[m] == sum {
                return (array[n], array[m])
            }
        }
    }
    (array[0], array[1])
}

