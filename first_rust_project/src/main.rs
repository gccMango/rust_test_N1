// enum WebEvent{
//     //struct without fields or data types
//     WELoad,
//     //tuple struct with data type but no named fields
//     WEKeys(String,char),
//     // classic struct with named fields and their data types
//     WEClick{x: i64,y:i64}

// }

// define a tuple struct
#[derive(Debug)]
struct KeyPress(String, char);

// define a classic struct
#[derive(Debug)]
struct MouseClick { x: i64,y: i64}

// redefine the enum variants to use the data from the new structs
// Update the page load variant to have the boolean type
#[derive(Debug)]
enum WebEvent { WELoad(bool), WEClick(MouseClick), WEKeys(KeyPress) }

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


    let click = MouseClick{ x: 100, y: 250};
    println!("Mouse click location : {},{}", click.x,click.y);

    let keys = KeyPress(String::from ("Ctrl+"),'N');
    println!("\nKeys pressed : {}{}",keys.0,keys.1);

    let we_load = WebEvent::WELoad(true);

    let we_click = WebEvent:: WEClick(click);

    let we_key = WebEvent::WEKeys(keys);

    println!("\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}", we_load, we_click, we_key); //we_click, we_key);
  
}