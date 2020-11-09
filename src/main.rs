
pub mod cats{
    #[derive(Debug)]
    pub struct Cat {
        pub name:String,
        pub breed: TypeOfCat,
    }

    #[derive(Debug)]
    pub enum TypeOfCat {
        Polydactyl,
        Snowshoe,
        Calico,
        British,
        Shorthair,
        Siamese,
        NorwegianForestCat,
        JapaneseBobtail,
        Persian,
        ScottishFold,
        GrayTabby
    }
}


pub mod shapes {
    #[derive(Debug)]
    pub struct Rectangle {
        x1: u32,
        y1: u32,
        x2: u32,
        y2: u32,
        width: u32,
        height: u32
    }

    impl Rectangle {
        pub fn new(x1: u32, y1: u32, x2: u32, y2: u32) -> Rectangle {
            let width = x2 - x1;
            let height = y2 - y1;
            Rectangle { x1, y1, x2, y2, width, height }
        }
        pub fn from(from: &Rectangle) -> Rectangle {
            Rectangle::new(from.x1, from.y1, from.x2, from.y2)
        }
        pub fn area(&self) -> u32 {
            self.height * self.width
        }
        pub fn can_contain(&self, r: &Rectangle) -> bool {
            self.area() > r.area()
        }
    }
}



fn main() {

    //test_looping();
    //test_while();
    //test_iterator();
    //test_for_with_range();
    //test_enum();
    //test_match()
    test_vector()

}

fn test_strings() -> String{
    let mut s=String::new();
    s.push_str("Ciao");
    s.push_str(" Mondo");
    s
}

fn test_looping(){
    let mut count=0;
    let num=loop{
        count+=1;
        println!("{}",count);
        if count==5{
            break count;
        }
    };
    println!("out from loop, num is:{}",num);
}

fn test_while(){
    let vec=[10,15,10,15,3];
    let mut c=0;
    while c<5{
        println!("{}",vec[c]);
        c+=1;

    }
}

fn test_iterator(){
    let vec=[10,15,10,15,3];
    print!("[");
    for i in vec.iter(){
        print!("{},",i);
    }
    println!("]");
}

fn test_for_with_range(){
    for i in (1..5).rev(){
        println!("{}",i);
    }
}

fn test_enum(){
    #[derive(Debug)]
    enum Temperature {
        Hot(u32),
        Warm(u32),
        Mild(u32),
        Cold(u32)
    }
    let current_t= Temperature::Hot(40);
    println!("current temperature is :{:?}",current_t);
}

fn test_match(){

    use crate::cats::*;

    let list_of_cats=vec![
        Cat {name:"Cora".to_string(),breed: TypeOfCat::Siamese},
        Cat {name:"Zelda".to_string(),breed: TypeOfCat::Persian}
    ];

    for c in &list_of_cats{
        match c.breed {
            TypeOfCat::Siamese => println!("a Siamese found"),
            _ =>(),
        };
    }

}
fn test_vector(){
    use crate::cats::*;

    let mut list_of_cats = Vec::new();

    list_of_cats.push(Cat {name:"Cora".to_string(),breed: TypeOfCat::Siamese});
    list_of_cats.push(Cat {name:"Zelda".to_string(),breed: TypeOfCat::Persian});
    list_of_cats.push(Cat {name:"Pallina".to_string(),breed: TypeOfCat::Siamese});

    assert_eq!(3,list_of_cats.len());

    let mut n_siameses=0;
    for cat in list_of_cats{
        if let TypeOfCat::Siamese=cat.breed{
            n_siameses+=1;
        }
    }
    println!("number of siameses:{}",n_siameses);



}

fn test_playing_with_shapes(){
    let s=test_strings();
    let ciao=&s[0..5];
    println!("{}",ciao);

    let r=shapes::Rectangle::new(0,0,5,2);
    let r1=shapes::Rectangle::from(&r);
    let r2=shapes::Rectangle::new(1,1,2,2);

    println!("r:{:?}",r);
    println!("r1:{:?}",r1);
    println!("r2:{:?}",r2);

    println!("Area r:{}",r.area());
    println!("r can contain r2 : {}",r.can_contain(&r2));

}