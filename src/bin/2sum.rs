fn main() {
    const ARRAY: [i8; 8] = [3i8, 5i8, -4i8, 8i8, 11i8, 1i8, -1i8, 6i8];
    let res = solution(&ARRAY, 10i8);
    match res {
        Ok(pair) => {
            assert!(pair == (11, -1));
            println!("({}, {})", pair.0, pair.1);
        },
        Err(e) => println!("{}", e)
    }
    
}

fn solution(array: &[i8], sum: i8) -> Result<(i8, i8), String> {
    for n in 0..=array.len() - 1 {
        for m in 0..=array.len() - 1 {
            if n == m {
                continue
            }
            if array[n] + array[m] == sum {
                return Ok((array[n], array[m]))
            }
        }
    }
    Err(format!("no 2 numbers sum to {:?}", sum))
}

