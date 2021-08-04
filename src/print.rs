pub fn run() {
// print to console
println!("Hello from the print.rs file");

//Basic Formatting
println!("{} is from {}", "Sierra", "AZ");

//Positional Arguments
println!("{0} is from {1} and {0} likes to {2}",
"Sierra", "AZ", "code" );

// Named Arguments (named paramaters)
println!("{name} likes to play {activity}", 
name = "Sally", activity = "piano");

//placeholder traits
println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10 );

// Placeholder for debug trait tuple
println!("{:?}", (12, true, "hello"));



}