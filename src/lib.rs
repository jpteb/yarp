pub fn add(x: u32, y: u32) -> u32 {
    x + y
}

#[cfg(test)]
mod tests {
    use crate::add;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 3), 5);
    }
}
