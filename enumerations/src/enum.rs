enum Message{
 	Quit:,
	Move{x:i32,y:i32},
	Write(String),
	ChangeColor(i32,i32,i32),
}
struct QuitMessage;
struct MoveMessage{
	x:i32,
	y:i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32,i32,i32);

impl Message{
  	fn call(&self){
	  //method body would defind here
	}
}

let m  = Message::write(String::form("hello"));
m.call();


