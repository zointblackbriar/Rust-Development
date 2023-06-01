mod sample {
    fn private_function() {
        println!("called `sample::private_function()`");
    }

    pub fn function_public() {
        println!("called `my_mod::public_function()`");
    }

    // Use the `pub` modifier to override default visibility.

}