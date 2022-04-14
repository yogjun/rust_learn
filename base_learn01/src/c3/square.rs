fn main() {
    // println!("Hello, world!");
    println!("mysquare,{}",apply(1,2,square));
    

}


fn apply(v1: i32,v2:i32,f:fn(i32,i32)->i32) -> i32 {
    f(v1,v2)
}

fn square(a: i32,b: i32) ->i32{
    a*b
}