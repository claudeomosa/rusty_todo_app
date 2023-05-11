pub trait Get{
    fn get(&self, name: &str){
        println!("{} is being fetched", name)
    }
}