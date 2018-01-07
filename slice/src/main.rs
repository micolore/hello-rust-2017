fn main() {

  let mut s = String::from("b    s");

  let len = first_word(&s);
  
  s.clear();

  println!("this is length is {}",len);
 
  let m = String::from("hello rustocean");
 
  let hello = &m[0..5];

  let rustocean  = &m[6..14]; 

  let  r = String::from("java");

  let r_len = r.len();

  let slice = &r[3..r_len];

  let slice = &r[3..];

  let  o = String::from("fuck");
 
  let o_r = second_word(&o);

  println!("the o value is {}",o_r);

}

fn first_word(s:&String) ->usize{
   let bytes = s.as_bytes();
  
   for(i,&item) in bytes.iter().enumerate(){
      if item  == b' '{
      return i;
     }
   }
   s.len()
}

fn second_word(s:&String) -> &str{
   
    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate(){
       
       if item  == b' '{
           return &s[0..i];
       }
   }
   &s
}

