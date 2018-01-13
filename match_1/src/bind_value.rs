#[derive(debug)]
enum UsState{
 	alibama,
	alaska,
}

enum Coin{
	Penny,
	Nickel,
	Dime,
	Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u32{
  	match coin{
		Coin::Penny =>1,
		...
		Coin::Quarter(state){
		       println!("state quarter from{:?}!",state);
		       25
		}
		
	}

}

//匹配 Option<T>

fn plus_one(x:Option<i32>) -> Option<i32>{
       macth x{
            None =>None,
	    Some(i) =>Some(i+2),
       }


}
let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

//匹配 Some<T>

//匹配是穷尽的

//_ 通配符

let some_u8_value = 0u8;
match some_u8_value {
 	1 => println!("one"),
	2 => println!("two"),
	5 => println!("five"),
	7 => println!("seven"),
	_ => (),//匹配到所有的值，放到最后
}























