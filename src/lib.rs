pub mod test_mod {
    pub fn hello() -> String {
        log::info!("test");
        String::from("test")
    }
}
