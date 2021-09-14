//! This library is designed to simplify the communication between a server agent, and a
//! central api server.

mod server;
mod client;

//TODO tests
#[cfg(test)]
mod tests {
    use macros::IntoImpl;
    #[test]
    fn test_macro() {  
        #[derive(IntoImpl, Debug, Eq, PartialEq)]
        enum Test {
            String(String),
            i32(i32),
        }
    
        //Testing that the macro is working correctly
        let a: Test = String::from("Hello, world-1!").into();
        let b: Test = (25 as i32).into();

        assert_eq!(a, Test::String("Hello, world-1!".to_owned()));
        assert_eq!(b, Test::i32(25));
    }
    
    #[test]
    fn test_macro_extended() {
        mod further_structs {
            #[derive(Debug, Eq, PartialEq)]
            pub struct Hello { }
        }        

        #[derive(IntoImpl, Debug, Eq, PartialEq)]
        enum Test {
            Other(i32),
            FurtherTest(further_structs::Hello),
        }

        let a: Test = (5 as i32).into();
        let b: Test = further_structs::Hello{}.into();

        assert_eq!(a, Test::Other(5));
        assert_eq!(b, Test::FurtherTest(further_structs::Hello{}))
    }
}
