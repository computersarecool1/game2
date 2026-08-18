fn main() {
 

    let mut i = H {t1:1,t2:2} ;
     i.trait1();
    print!("{:?}",i);
      f(&mut i);
    print!("{:?}", i);

}

    trait A {
        fn trait1(&mut self);
    }


impl A for H {
        fn trait1(&mut self) {
            self.t1 *= 2;
         
        }
    }

#[derive(Debug)]
struct H {
    t1: i32,
    t2: i32,
}

fn f(mut a: &mut impl A) {
      a.trait1();
}