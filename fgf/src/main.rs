fn main() {
 let x = vec![0,2,6,8,10,8];
let filt: Vec<_> = x.iter().filter(|w| **w == 8).collect(); //filt = [8]
let pos =  x.iter().position(|w| *w == 8); // Some(3), because x[3] = 8
println!("{:?}",filt);
println!("{:?}",pos);

}
