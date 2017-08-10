mod shell;

use shell as Shell;

use std::io::{self, Write};

fn main() 
{
  loop
  {
    print!("$~: ");
    io::stdout().flush();

    let thing = Shell::read_line();
    if thing == "quit"
    {
      break;
    }
    let output = Shell::execute_command(Some(thing));
    println!("{}", output);
  }
}
