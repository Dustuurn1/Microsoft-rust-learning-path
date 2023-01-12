use std::str;

fn main() {
   println!("{}", multplication(3i64, 9i64));
}

fn pi_types(){
    let pi= 3.14;
    let pi_string = "pi";
    println!("The short form of {} is {}.",pi_string,pi);
}

fn mutable_types(){
    let mut number = 10;
    println!("The number is {}.",number);
    number = 20;
    println!("Now the number is {}.",number);
}

fn shadow_variable(){
    let shadow = 7;
    let shadow = shadow * 2;
    let shadow = shadow * 2;
    println!("{}",shadow);
}

fn data_math(){
    //Addition, subtraction, and multplication
    println!("2+2={} and 8-3={} and 3*3={}",2u32+2, 8i32-3, 3*3);
    //Interger vs Floating point division
    println!("9/2={} (int) vs 9/2={} (float)",9u32/2,9f64/2.0); 
}

fn booleans(){
    let bigger = 1 > 2;
    println!("Is 1 > 2? {}",bigger);
}

fn chars(){
    let uppercase_s:&str = "ss";
    let lowercase_f = 'f';
    let smiley_face = '😃';
    println!("{}\t{}\t{}",uppercase_s, lowercase_f, smiley_face);
}

fn tuple(){
    let a_tuple = ('A',5,3.2);
    println!("{}",a_tuple.0);
    println!("{}",a_tuple.1);
    println!("{}",a_tuple.2);
}

fn structs(){
    //Classic struct
    struct Character {name:String, level:u16, race:String, xp:u128, alive:bool}
    //Tuple struct
    struct Grades(char, char, char, char, char, f64);
    //Unit struct
    struct Unit;
}

fn character_structs(){
    struct Character {name:String, level:u16, race:String, xp:u128, alive:bool}
    let races = ("Orc","Human","Dwarf","Elf");
    let char1 = Character {name: String::from("Dustuurn"),level:80,race: String::from(races.0),xp:1800,alive:true };

    println!("Name:{}\tLevel:{}\tRace:{}",char1.name,char1.level,char1.race);
}

fn multplication(x:i64,y:i64)-> i64{
    x*y     //or return x*y;
}
