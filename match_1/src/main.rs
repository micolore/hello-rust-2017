enum Coin {
	Penny,
	Nickel,
	Dime,
	Quarter,
}

fn value_in_cents(coin:Coin) ->i32{

       match coin{
       	    Coin::Penny =>{
	         println!("lucky penny!");
		 1
	    },
	    Coin::Nickel =>5,
	    Coin::Dime	=>10,
	    Coin::Quarter =>25,
       }
}
fn main(){
  let result =  Coin::Penny;
  //println!("the value is {}",result);
}
