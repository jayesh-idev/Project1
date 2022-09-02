use std::io;
pub fn addtask(task:&mut Vec<String>)
{
    loop 
    {
        println!("Please Enter Your task");
        let mut data = String::new();
        io::stdin().read_line(&mut data).expect(" ");
        let data:String = data.trim().parse().expect("");
        task.push(data);
        println!("Adding success");
        println!("Enter 'add' to add more or anything else to exit");
        let mut query = String::new();
        io::stdin().read_line(&mut query).expect(" ");
        let query:String = query.trim().parse().expect("");
        if query.as_str() == "add"{ continue;}
        else { break; }



    }
  
}