
fn main() {
    println!("Hello, world!");

    struct Student {name : String, level: u8, remote: bool}
    
    //Tuple struct data types only
    struct Grades(char, char, char, char,f32);

    // Unit struct
    // struct Unit;

    let user_1 = Student {name: String::from("John luca"),remote: true, level: 8};

    let user_2 = Student {name: String::from("Dyson Tan"),level: 5, remote: false};

    let mark_1 = Grades('A','A','A','B',3.75);
    let mark_2 = Grades('C','A','A','B',3.25);

    println!("{}, level {}. Remote:{} Grade: {},{},{},{}. Average: {}",user_1.name,user_1.level,user_1.remote,mark_1.0,mark_1.1,mark_1.2,mark_1.3,mark_1.4);
    println!("{}, level {}. Remote:{} Grade: {},{},{},{}. Average: {}",user_2.name,user_2.level,user_2.remote,mark_2.0,mark_2.1,mark_2.2,mark_2.3,mark_2.4);
}