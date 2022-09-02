use std::io;
pub fn delete(notes:&mut Vec<String>)
{
  loop
  {

    println!("Enter the task number You want to Delete or enter 'exit' to exit from remove manager");
    let mut task = String::new();
    io::stdin().read_line(&mut task).expect("");
    let task:String = task.trim().parse().unwrap();
    if task == "exit" { break;}
    let task:usize = match task.parse()
    {
        Ok(t)=>t,
        err=>{println!("Number should be a integer : {:?}",err); continue;}
    };
    if task >= 0 as usize && task< notes.len()
    {
          for i in task..notes.len()-1
          {
            notes[i] = notes[i+1].clone();
          }
          notes.pop();
    }
    else  {
      println!("The task query is in Range");
      continue;
        
    }


  }
   

   
}