pub trait Create {
    fn create(&self, name: &str){
        println!("{} is being created", name)
    }
}