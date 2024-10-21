
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn main() {
    println!("Catch-all Patterns and the _ Placeholder");
    let dice_roll = 9;

    // Two specific values, 3 and 7.
    // Any "other" value is captured in a variable, called other in this 
    //case. It can then be used in the corresponding action.
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // If we want to take an action for the unspecified values, but
    // do not care what the value is, we use the variable name _ to
    // avoid capturing it.
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // Don't use value
    }

    // If we don't want to take any action for the unspecified values:
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // Don't do anything
    }

}
