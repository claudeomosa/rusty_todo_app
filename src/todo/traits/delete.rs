pub trait Delete {
    fn delete(&self, name: &str){
        println!("{} is being deleted", name)
    }
}