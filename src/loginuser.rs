use std::io;
pub fn loginuser()->String
{
    let mut reply:String = String::new();
    io::stdin().read_line(&mut reply).expect("msg");
  loop{
    println!("Your TO-Do list");
    println!("Sign up or Sign in ? Enter");
     reply = String::new();
    io::stdin().read_line(&mut reply).expect("");
    let reply:String = reply.trim().parse().unwrap();
    let reply = match reply.as_str()
    {
      "Sign in"=>"Sign in",
      "Sign up"=>"Sign up",
      _ => "Wrong"
    };
    if reply == "Sign in" || reply =="Sign up"
    {
      break;
    }
    else {
      println!("Wrong input,Please Enter Sign in or Sign up");
        
    }

}
return reply;






}