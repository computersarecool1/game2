fn main() {
    let added = 1;
   let c = |a: u32| a+added;

   println!("{}",c(5)) 
}
fn addermaker(a:u32) -> impl Fn(u32) -> {

}

fn addone(a: u32) -> u32 {
    a + 1
}


