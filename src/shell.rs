use std::io;

struct Command 
{
  command: String,
  output: fn(Option<String>) -> Option<String>
}

pub fn execute_command(command_option: Option<String>) -> String
{
  let command = match command_option
    {
      Some(x) => x,
      None => read_line()
    };

  let mut av_cmds = Vec::new();

  fn vim(x: Option<String>) -> Option<String>
    { Some("Did you mean emacs?".to_owned()) }

  av_cmds.push(Command { 
    command: "vim".to_owned(),
    output: vim
  });

  av_cmds.push(Command {
    command: "vi".to_owned(),
    output: vim
  });

  av_cmds.push(Command {
    command: "nvim".to_owned(),
    output: vim
  });

  fn emacs(x: Option<String>) -> Option<String>
    {Some("Did you mean vim?".to_owned())}

  av_cmds.push(Command {
    command: "emacs".to_owned(),
    output: emacs 
  });

  fn nano(x: Option<String>) -> Option<String>
    {Some("Did you mean ed?".to_owned())}

  av_cmds.push(Command {
    command: "nano".to_owned(),
    output: nano
  });

  fn ed(x: Option<String>) -> Option<String>
    {Some("ed? a real programmer uses a magnetized needle and a steady hand".to_owned())}

  av_cmds.push(Command {
    command: "ed".to_owned(),
    output: ed
  });


  fn butterfly(x: Option<String>) -> Option<String>
    {Some("There's an emacs extension for that".to_owned())}

  av_cmds.push(Command {
    command: "butterfly".to_owned(),
    output: butterfly
  });

  fn life(x: Option<String>) -> Option<String>
    {Some("42".to_owned())}

  av_cmds.push(Command {
    command: "life".to_owned(),
    output: life
  });

  fn echo(x: Option<String>) -> Option<String>
  {
    Some(match x
    {
      None => "No valid input entered".to_owned(),
      Some(x) => x
    })
  }

  av_cmds.push(Command {
    command: "echo".to_owned(),
    output: echo
  });

  let mut response: Option<String> = None;
  for av_command in &av_cmds // Prevents it from moving the value
  {
    if command == av_command.command
    {
      response = (av_command.output)(None);
    }
  }

  let coms = av_cmds.iter().map(|ref x| x.command.to_owned()).collect::<Vec<String>>();
  match response
  {
    Some(x) => x,
    None => format!("I'm sorry but {} isn't valid, available commands are {}\nYou're welcom ;)", command, get_avail_commands(&coms))
  }
}

pub fn read_line() -> String
{
  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Standard in Fail!");

  input
    .trim()
    .to_owned()
}

fn get_avail_commands(list: &Vec<String>) -> String
{
  let mut out = String::new();
  for item in list
  {
    out = out + item + ", " ;
  }
  out + "quit"
}
