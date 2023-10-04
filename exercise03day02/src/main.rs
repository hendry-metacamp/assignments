struct Novel{
    title:String,
    author:String,
    genre:String,

}

struct NonFiction{
    title:String,
    author:String,
    topic:String,
}

trait Book{
    fn get_summary(&self)->();
}

impl Book for NonFiction{
    fn get_summary(&self){
        println!("Book is {} topic is by{}",self.title,self.topic);
    }
}


impl Book for Novel{
    fn get_summary(&self){
        println!("Book is {} written by{}",self.title,self.author);
    }
}
fn main() {
    let novel=Novel{title:"AAA".to_owned(),author:"HH".to_owned(),genre:"AAAC".to_owned()};
    novel.get_summary();

    let nonfiction=NonFiction{title:"AAA".to_owned(),author:"HH".to_owned(),topic:"AAAC".to_owned()};
    nonfiction.get_summary();
}
