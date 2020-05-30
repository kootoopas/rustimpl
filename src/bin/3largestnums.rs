pub fn find_3_largest_nums(vec: Vec<i32>) -> Result<(i32, i32, i32), String> {
    
    if vec.len() < 3 {
        return Err("input vec should have 3 or more elements".to_string())
    }

    let mut a: i32 = std::i32::MIN;
    let mut b: i32 = std::i32::MIN;
    let mut c: i32 = std::i32::MIN;

    for n in vec {
        if n > a {
            let t = a;
            let t_ = b;
            a = n;
            b = t;
            c = t_;
        } else if n > b {
            let t = b;
            b = n;
            c = t;
        } else if n > c {
            c = n;
        }
    }

    Ok((a, b, c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_sorted_3_largest_nums() {
        let i = vec![8, 5, 2, 9, 5, 6, 3];
        let r = find_3_largest_nums(i);
        assert!(r.is_ok());
        match r {
            Ok(largest_triple) => assert_eq!(largest_triple, (9, 8, 6)),
            Err(_) => {}
        }
    }
    
    #[test]
    fn errs_when_input_vec_len_is_less_than_desired_output() {
        let i = vec![2, 1];
        let r = find_3_largest_nums(i);
        assert!(r.is_err());
        match r {
            Err(msg) => assert!(msg.len() > 0),
            Ok(_) => {}
        }
    }
}
