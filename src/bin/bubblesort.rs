fn main() {
    let mut input = vec![8, 5, 2, 9, 5, 6, 3];
    match bubblesort(input.as_mut()) {
        Ok(sorted) => {
            assert_eq!(sorted.to_vec(), vec![2, 3, 5, 5, 6, 8, 9]);
            println!("{:?}", sorted)
        },
        Err(msg) => println!("{}", msg)
    }
}

pub fn bubblesort(vec: &mut Vec<i32>) -> Result<&mut Vec<i32>, String> {
    if vec.len() == 0 {
        return Ok(vec)
    }

    let mut r = vec.len() - 1;
    while r > 0 {
        for i in 0..r {
            if vec[i] > vec[i + 1] {
                let t: Vec<i32> = vec.splice(i..i+1, vec![vec[i + 1]]).collect();
                vec.splice(i+1..i+2, vec![t[0]]);
            }
        }
        r = r - 1
    }
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sorts_unordered_int_vec() {
        let mut i = vec![8, 5, 2, 9, 5, 6, 3];
        let r = bubblesort(i.as_mut());
        assert!(r.is_ok());
        match r {
            Ok(sorted) => assert_eq!(sorted.to_vec(), vec![2, 3, 5, 5, 6, 8, 9]),
            Err(_) => {}
        }

    }

    #[test]
    fn returns_single_element_vec() {
        let mut i = vec![5];
        let r = bubblesort(i.as_mut());
        assert!(r.is_ok());
        match r {
            Ok(sorted) => assert_eq!(sorted.to_vec(), vec![5]),
            Err(_) => {}
        }
        
    }

    #[test]
    fn returns_empty_vec() {
        let mut i = vec![];
        let r = bubblesort(i.as_mut());
        assert!(r.is_ok());
        match r {
            Ok(sorted) => assert_eq!(sorted.to_vec(), vec![]),
            Err(_) => {}
        }
        
    }
}