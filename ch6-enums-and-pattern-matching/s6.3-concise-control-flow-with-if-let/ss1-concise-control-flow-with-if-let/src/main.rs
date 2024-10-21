fn main() {
    println!("Concise control flow with if let");

    {
        // This is a common pattern:
        // match an option, doing something only in the Some case
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("The maximum is configured to be {max}"),
            _ => (),
        }
    
    }

    {
        // This sets some value, to be an Option::Some
        let config_max = Some(3u8);

        // This does something if the value is something
        if let Some(max) = config_max {
            println!("The maximum is configured to be {max}");
        }

        // Example when the supplied value is nothing
        println!("Try nothing");
        let nothing: Option<i32> = None;
        
        if let Some(max) = nothing {
            println!("The maximum is configured to be {max}");
        }

    
    }
}
