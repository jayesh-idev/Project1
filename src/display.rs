pub fn display(notes:&mut Vec<String>)
{
    if notes.len() == 0
    {
        println!("List is Empty");
        return;
    }
    for a in 0..notes.len()
    {
        println!("{}: {}",a,notes[a]);
    }
}