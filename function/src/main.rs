fn five() -> i32 {
   5
}

fn plus_one(x:i32) ->i32{
    
     x+1
}
fn main() {
   
    let x = five();
 
    println!("The value of x is: {}", x);

    let y = plus_one(6);

    println!("the plus_one  return value is {}",y);


    let flag = true;
  
    let  number = if flag {
         5 
    }else{
         0 
    };
   
    println!("the number value of is {}",number);

    let list = [1,102,201,98,70,83];
   
    for element  in list.iter(){
         
        println!("the value  is {}",element);
    }
    for number  in(1..4).rev(){
         println!("{}!",number);
    }
    println!("lifeoff");

    let s1 = String::from("hello");

    let (s2,len) =  calculate_length(s1);
    
    println!("the length of {}, is {}",len,s2);

    let s3    = String::from("rust");

    let len3  = calculate_length_two(&s3);
 
    println!("the s3 length is {} is {}",len3,s3);
}
fn calculate_length(s: String) ->(String,usize){
    let length = s.len();
    (s,length)
}
fn calculate_length_two(s:&String) -> usize{
    s.len()
}
