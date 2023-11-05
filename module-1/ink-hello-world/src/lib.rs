use ink_lang as ink;

#[ink::contract]
mod hello_world {
    #[ink(storage)]
    pub struct HelloWorld {}

    impl HelloWorld {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {}
        }

        #[ink(message)]
        pub fn say_hello(&self) {
            ink_env::debug_println("Hello, World!");
        }
    }
}
