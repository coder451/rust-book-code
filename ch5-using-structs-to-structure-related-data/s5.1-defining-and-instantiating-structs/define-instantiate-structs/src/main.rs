/// Struct declaration at global scope
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
} // No ;

/// Factory for the struct
/// Uses a slice so literals can be used in the factory.
/// The tutorial is silent at this point about ownership.
fn build_user(email: &str, username: &str) -> User {
    User {
        active: true,
        username: String::from(username),
        email: String::from(email),
        sign_in_count: 1,
    }
}

// A string version - cannot overload the name
// This uses the field init shortcut. The factory parameters have the
// same names as the struct members, so there is no need to repeat them
// in the struct
fn build_user_s(email: String, username: String) -> User {
    User {
        active: true,
        username, // <- shortcut
        email, // <- shortcut
        sign_in_count: 1,
    }
}


fn main() {
    println!("Defining and instantiating structs");

    {
        let user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };
        println!("Immutable:");
        println!("active: {}", user1.active);
        println!("username: {}", user1.username);
        println!("email: {}", user1.email);
        println!("sign_in_count: {}", user1.sign_in_count);
    }

    {
        
        let mut user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        println!();
        println!("Mutable:");
        println!("active: {}", user1.active);
        println!("username: {}", user1.username);
        println!("email: {}", user1.email);
        println!("sign_in_count: {}", user1.sign_in_count);

        user1.email = String::from("anotheremail@example.com");
        println!("updated email: {}", user1.email);
    }

    {
        // Using the slice version
        let user = build_user("a@b", "ab");
        println!("{0}: {1}", user.username, user.email);
    }
    {
        // Using the string version
        let username = String::from("ab");
        let email = String::from("a@b");
        let user = build_user_s(email, username);
        println!("{0}: {1}", user.username, user.email);
    }

    {
        // From The Reference
        // "nominal struct type" = struct type with a name
        struct Point {x: i32, y: i32}
        let p = Point {x: 10, y: 11};
        // Here The Reference specifies the type on the left
        let px = p.x;
        println!("px is {px}");
        println!("py is {}", p.y);
    }

    {
        // Reference: tuple struct
        // Allows naming a tuple type: "nominal tuple type"
        struct Point(i32, i32);
        let p = Point(10, 11);

        // Accessing elements using match
        let px = match p { Point(x, _) => x };

        // Accessing element using position
        let py = p.1;
        println!("px is {px}, py is {py}");
    }

    {
        // Reference: Unit struct
        // Implicitly defines a constant of the type, 
        // const Cookie = Cookie {};
        struct Cookie;
        let _c = [Cookie, Cookie {}];
    }

    {
        // Struct update syntax
        // As _user2 moved the username from user1, user1.username is 
        // no longer valid.
        let user1 = build_user("a@b", "ab");
        let _user2 = User {
            email: String::from("c@d"),
            ..user1
        };
        
        //println!("{}", user1.username); // error
        println!("{}", user1.email);
    }

    {
        // Named tuple structs with same types internally but different 
        // types overall
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);

        println!("black {} {} {}", black.0, black.1, black.2);
        println!("origin {} {} {}", origin.0, origin.1, origin.2);
    }

}
