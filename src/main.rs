use std::collections::HashMap;
mod loginuser;
mod newuser;
mod display;
mod addtask;
mod delete;

use std::io;
struct Usernote 
{
    _usernamet:String,
    notes:Vec<String>
}
fn main()


{
   loop{
    let mut userpassword = HashMap::new();
    let mut userdata:HashMap<String,Usernote > = HashMap::new();
    userpassword.insert("Enter Username".to_string(),"Enter Password".to_string());
    let reply = loginuser::loginuser();
    let mut  user_name = String::new();
    
    
    let reply = reply.trim();
    if reply == "Sign up"
    {
       let user_data =  newuser::c_user();
       userpassword.insert(user_data[0].clone(),user_data[1].clone());
       let arr:Vec<String>  = Vec::new();
       userdata.insert(user_data[0].clone(),Usernote{_usernamet:user_data[0].clone(),notes:arr});
       user_name = user_data[0].clone();
    }
    else 
    {
        
             {
                     let mut right :bool =  false;
                     let mut username = String::new();
                     let mut pass = String::new();
                     println!("Plese Enter Your Username");
                     io::stdin().read_line(&mut username).expect(" ");
                     let username:String =  username.trim().parse().unwrap();
                     println!("Plese Enter Your Password for {}",username);
                     io::stdin().read_line(&mut pass).expect(" ");
                     let pass:String =  pass.trim().parse().unwrap();
                     if userpassword.contains_key( &username) == true
                      /* */      {
                               let passvalue = userpassword.get(&username).unwrap();
                                        if passvalue == &pass 
                                                 {
                                                       right = true;
                                                 }
                                        else {
                                                       right = false;
                                                 }                       }
                   if right == true { println!("welcome back {}",&username);} //UserName = username;break; }
                   let mut tri = String::new();
                   println!("Username and Password is incorrect");
                   println!("press R to retry or press anything to exit");
                   io::stdin().read_line(&mut tri).expect(" ");
                   let tri:String = tri.trim().parse().unwrap();
                   if tri.as_str() == "R" { continue;}
                   //else { panic!("You have exited the program"); }

       
           

    }
}
let data = userdata.get_mut(&user_name).unwrap();
if data.notes.len() == 0
    {
           println!(" Your list is Empty");
    }
    {
        display::display(&mut data.notes);
    }
    let mut condition:i32 =0; 
loop{
           println!("Enter , display , add ,delete, exit ,login");
           //println!("display");
          // println!("delete");
           //println!("add");
           //println!("exit");
           //loginprintln!("login");
           let mut query = String::new();
           io::stdin().read_line(&mut query).expect(" ");
           let query:String = query.trim().parse().unwrap();
           if query == "display"
              {
               display::display(&mut data.notes);
               }
           if query == "add"
              {
               addtask::addtask(&mut data.notes)
               }
           if query == "delete"
           {
               delete::delete(&mut data.notes);
           }
           if query == "exit"
           {
               break;
           }
           if query =="login"
           {
               condition= 1;
               break;
           }
           else {
               
               continue;

           }

       }
       if condition == 1 {continue;}
       else {break;}
   }
      
   }

          
    


    





    



       
       
   
