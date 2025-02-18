pub fn is_five(x: &i32) -> bool {
    //if statement to check if the value is 5
    if *x == 5 {
        return true;
    }
    else {
        return false;
    }
}


fn swap(a: &mut i32, b: &mut i32) {
   //function that will swap the values of a and b
    let temp = *a;
    *a = *b;
    *b = temp;
}



fn add_and_multiply(a: &i32, b: &mut i32) {
    //function that will add the value of a to b and then multiply the result by 2
    *b = (*b + *a) * 2;
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_is_five() {
        
        assert!(is_five(&5), "Expected true when the value is 5");
        assert!(!is_five(&6), "Expected false when the value is not 5");
    }


    #[test]
    fn test_swap() {
        let mut x = 5;
        let mut y = 10;
        swap(&mut x, &mut y);
        assert_eq!(x, 10);
        assert_eq!(y, 5);
    }


    #[test]
    fn test_add_and_multiply() {
        let a = 10;
        let mut b = 20;
        add_and_multiply(&a, &mut b);
        assert_eq!(b, 60); // (20 + 10) * 2
    }
}

