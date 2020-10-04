use std::io;
use std::io::BufRead;
fn main() {
    println!("\n\n*******/// GPA Calculator \\\\\\*******");
    println!("Developed by GODWIN, Written in Rust\n\n *Note: this is based on Uniben 200L ENG class and any course OMITTED is assumed *F*\n\n");
    let stdin = io::stdin();
    let mut accum = 0f32;
    let d = 20f32;
    println!("hello dear please what's your name?");
    let mut name = String::new();
    io::stdin().read_line(&mut name)
            .expect("Failed to read line");
    println!("you're welcome! {}\nPlease Type the uppercase course code and grade(eg. MEE211 A) then press enter\n type `done` to calculate GPA", name);
    let reader = stdin.lock();
    let (unitsa, unitsb, unitsc, unitsd, unitse, unitsf) = (15f32, 12f32, 9f32, 6f32, 3f32, 0f32);//for 3 units course
    let (unita, unitb, unitc, unitd, unite, unitf) = (10f32, 8f32, 6f32, 4f32, 2f32, 0f32);//for 2 units course
    for line in reader.lines() {
        let srt = line.unwrap();
        match srt.trim() {
            "MEE212 A"|"MEE211 A"|"EEE211 A"|"CVE211 A" => {
                accum += unitsa;
            }
            "MEE212 B"|"MEE211 B"|"EEE211 B"|"CVE211 B" => {
                accum += unitsb;
            }
            "MEE212 C"|"MEE211 C"|"EEE211 C"|"CVE211 C"  => {
                accum += unitsc;
            }
            "MEE212 D"|"MEE211 D"|"EEE211 D"|"CVE211 D"  => {
                accum += unitsd;
            }
            "MEE212 E"|"MEE211 E"|"EEE211 E"|"CVE211 E"  => {
                accum += unitse;
            }
            "MEE212 F"|"MEE211 F"|"EEE211 F"|"CVE211 F"  => {
                accum += unitsf;
            }
            "ENS211 A"|"EMA281 A"|"PRE211 A"|"ECP211 A" => {
                accum += unita;
            }
            "ENS211 B"|"EMA281 B"|"PRE211 B" |"ECP211 B" => {
                accum += unitb;
            }
            "ENS211 C"|"EMA281 C"|"PRE211 C"|"ECP211 C" => {
                accum += unitc;
            }
            "ENS211 D"|"EMA281 D"|"PRE211 D"|"ECP211 D" => {
                accum += unitd;
            }
            "ENS211 E"|"EMA281 E"|"PRE211 E"|"ECP211 E" => {
                accum += unite;
            }
            "ENS211 F"|"EMA281 F"|"PRE211 F"|"ECP211 F" => {
                accum += unitf;
            }
            "E"|"e" => {
                println!("success in your academics bye! {}", name);
                break;
            }
            "done"|"DONE" => {
                
                println!("\n{} your Calculated GPA is: {}\n\ntype `e` to exit",name, accum/d)
            }
            _ => println!("Input Invalid!"),
        }
    }
}
