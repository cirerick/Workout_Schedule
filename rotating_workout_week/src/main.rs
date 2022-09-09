use std::io;
use std::collections::VecDeque;

fn rotate (workouts : &mut VecDeque<String>) {
    let moved_workout : String = workouts.pop_back().unwrap();

    workouts.push_front(moved_workout);

}


fn print_week <T: std::fmt::Display>(days_string : &[String; 5], workouts : &VecDeque<T>) {
    //itrerate through the workout and Queue

    for _i in days_string.iter(){
        print!("{day_string:<15} ",day_string = _i);
    }
    println!("");
    for _i in workouts{
        print!("{workout:<15} ", workout = _i);
    } 
    println!("");

}

fn main() {
    println!("Let's start this work out.");

    /*
        A string array that stores weeks
        A string queue that stores the workouts
        rotation funtion
        print function
    */

    //Array containing days
    let days_string : [String; 5] = [
        String::from("Monday"), 
        String::from("Tuesday"), 
        String::from("Wednesday"), 
        String::from("Thursday"), 
        String::from("Friday")];
    
    //A form of a Queue hold the workouts
    let mut workouts = VecDeque::with_capacity(5);
        workouts.push_back(String::from("Shoulders"));
        workouts.push_back(String::from("Legs"));
        workouts.push_back(String::from("Chest"));
        workouts.push_back(String::from("Arms"));
        workouts.push_back(String::from("Back/Abs"));

    //rotation
    for _i in 1..10{
    println!("Week {}", _i);
    
    //print first
    print_week(&days_string, &workouts);
    rotate(&mut workouts);
    println!(" ");
    }

}
