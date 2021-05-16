use rand;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(addd_one(3), 4);
    }
}

pub fn addd_one(x: i32) -> i32 {
    x + 1
}
