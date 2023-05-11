pub trait Edit {
    fn set_to_done(&self, name: &str){
        println!("{} is being set to done", name);
    }
    fn set_to_pending(&self, name: &str) {
        println!("{} is being set to pending", name);
    }
}