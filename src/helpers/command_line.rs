


//style
use crossterm::{
    style::{Color,ResetColor,SetForegroundColor},
    ExecutableCommand,
};

use std::io::{stdin,stdout};


#[derive(PartialEq,Debug)]
pub enum PrintCommand{
    AICall,
    UnitTest,
    Issue,
}


impl PrintCommand{
    pub fn print_agent_message(&self,agent_pos:&str,agent_statement:&str){
        let mut stdout:std::io::Stdout = stdout();



        //Decide on the print color
        let statement_color:Color = match self{
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        //Print the agent position
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent:{}: ",agent_pos);

        //Make Selected color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}",agent_statement);

        //ResetColor
        stdout.execute(ResetColor).unwrap();

    }

}



#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn tests_prints_agent_msg(){
        PrintCommand::AICall.print_agent_message("Managing Agent","Testing testing,processing somethings ");
    }
}
//get user request
pub fn get_user_response(question:&str)->String{
    let mut stdout: std::io::Stdout = stdout();


    //Print the question in a specific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}",question);


    //Reset Color
    stdout.execute(ResetColor).unwrap();


    //Read user input
    let mut user_responses:String = String::new();
    stdin()
    .read_line(&mut user_responses)
    .expect("Failed to read response");

    //Trim white Space and return 
    return user_responses.trim().to_string();
} 