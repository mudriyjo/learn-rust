pub async fn double(num: u32) -> u32 {
    num * 2
}

#[tokio::main]
async fn main() {
    println!("Hello, world! And double 2 : {}", double(2).await);
}

#[cfg(test)]
mod test {
    use crate::double;

    #[test]
    fn test_double_with_runtime() {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        assert_eq!(rt.block_on(double(2)), 4);
    }

    #[tokio::test]
    async fn easy_test() {
        assert_eq!(double(2).await, 4);
    }
}
