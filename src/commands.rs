// Command

use std::fs;
use std::io::Write;

pub trait Command {
    fn handle(&self) -> i32;
}

// Add Command

pub struct AddCommand {
    args: Vec<String>,
}

impl AddCommand {
    pub fn new (args: Vec<String>) -> Self {
        AddCommand {
            args,
        }
    }


}

impl Command for AddCommand {
    fn handle(&self) -> i32 {
        let description_option = &self.args.get(2);
        
        if let Some(description) = description_option {

            let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open("Storage.txt")
            .expect("File not found");

            writeln!(file, "{description}")
            .expect("File not writeable");

            println!("Todo Added");

            return 0;
        } else {
            println!("Description not provided but is required");

            return 1;
        }

        
    }
}

// List Command

pub struct ListCommand {

    
}

impl ListCommand {
    pub fn new () -> Self {
        ListCommand {
            // ...
        }
    }


}

impl Command for ListCommand {
    fn handle(&self) -> i32 {
        let contents = fs::read_to_string("Storage.txt")
        .expect("File not found");

        println!("{contents}");
        0
    }
}


// -- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_command() {
        
        // Prepare
        let args = vec![
            "todo".to_string(),
            "add".to_string(),
            "My todo 4".to_string(),
        ];

        let command = AddCommand::new(args);

        // Act
        let exit_code = command.handle();


        // Assert
        assert_eq!(0, exit_code);


    }
    #[test]
    fn list_command() {
                // Prepare
                let command = ListCommand::new();
                // Act
                let exit_code = command.handle();
        
        
                // Assert
                assert_eq!(0, exit_code);

    }

}