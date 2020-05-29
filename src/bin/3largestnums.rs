pub fn find_3_largest_nums(vec: &Vec<i32>) -> Result<(i32, i32, i32), String> {
    
    if vec.len() < 3 {
        return Err("Vec should have 3 or more elements".to_string())
    }

    let mut a: Option<&i32> = None;
    let mut b: Option<&i32> = None;
    let mut c: Option<&i32> = None;

    for n in vec.iter() {
        if n > a || a == None {
            let t = a;
            let t_ = b;
            a = n;
            b = t;
            c = t_;
        } else if n > b || b == None {
            let t = b;
            b = n;
            c = t;
        } else if n > c || c == None {
            c = n
        }
    }

    Ok((*a, *b, *c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_sorted_3_largest_nums() {
        let mut i = vec![8, 5, 2, 9, 5, 6, 3];
        let r = find_3_largest_nums(i.as_mut());
        assert!(r.is_ok());
        match r {
            Ok(largest_triple) => assert_eq!(largest_triple, (9, 8, 6)),
            Err(_) => {}
        }
    }
}