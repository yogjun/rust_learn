use std::thread::spawn;
use std::sync::Arc;

fn main() {
    let str = "hello world";
    let arcstr1 = Arc::new(str);
    // let arcstr2 = arcstr1.clone();

    let handler = 
    std::thread::spawn(move||{
        println!("child {:?}",arcstr1);
    });

    println!("parent {:?}",str);
    handler.join().unwrap();
  }


//   fn main() {
//     let arr = vec![1];
  
//     std::thread::spawn(move|| {
//       println!("{:?}", arr);
//     });
//   }