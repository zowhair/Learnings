impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut n = x;
        let mut result = 0i32;
        
        while n.abs() > 0 {
            match result.checked_mul(10) {
                Some(v) => {
                    match v.checked_add(n%10) {
                        Some(v2) => {
                            result = (result * 10) + (v2 %10);
                            
                        }
                        
                        None => {
                            return 0 as i32
                        }
                    }
                }
                None => {
                    return 0 as i32
                }
            }
            n = n/10
        }
        
        result
    }
}
