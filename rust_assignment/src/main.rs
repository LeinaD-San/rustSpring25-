// // fn main(){


// // let x = 5;
// //     let rem = x %3;

// //     if rem == 0{
// //         println!('Divisible');
// //     }
// //     else if rem == 1{
// //     //     println('Remainder = {}',rem);
// //     // }

/*
fn main() {
    let s = String::from("Hello"); // Allocates memory on the heap
    println!("message from heap: {}", s);

    let mut s = 1234.to_string(); // Note: rules regarding mutability still apply
    println!("message from heap: {}", s);

    // Strings are mutable
    s.push_str("4567");
    println!("My string number: {}", s);
    drop(s);
}
*/


/*
5 is going to be allocated in stack

fn main() {
    let x = 5;
    let y = x; // This creates a copy for primitive types
    println!("x is: {}, y is: {}", x, y);

    let s1 = String::from("Hello");
    let s2 = s1; // This moves ownership, s1 is no longer valid
    
    // println!("s1 is: {}, s2 is: {}", s1, s2); // This would cause a compile error
}
    */

    
    //s gets removed from the heap
/*
    fn main() {
        let s = String::from("hello");
        let s = takes_ownership(s);
        println!("{}",s)
        //takes_ownership(s);
        // s is no longer valid here
    
        //let x = 5;
        //makes_copy(x);
        // x is still valid here
    }
    
    fn takes_ownership(some_string: String) -> String{
        println!("{}", some_string);
        some_string
    } // some_string goes out of scope and is dropped
    
    
    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    } // some_integer goes out of scope, nothing special happen


     */

    /*
    word is initialized with 'UTRGV' to a string, where its only
    read only
    next, borrow_word is getting the address of the variable word
    using &, where in a way is saying that it is pointing to the
    data the word is holding, reading the data

     fn main() {
        let word = "UTRGV".to_string();
        let borrow_word = &word;
        //println!("{} == {}", word, borrow_word);
        let borrow_word1 = &word;
        //println!("{} == {}",word, borrow_word1)
        print_string(&word);
    }
    
    fn main(){
        let word = "Hello, my name is".to_string();
        let loan_word = &word;
        println!("{} == {}",word,loan_word);
    }

    fn print_string(w:& String){
        println!("{}",w);
    }
    

    fn borrow_to_mut_watchout() {
        let mut word = "UT".to_string(); 
        fn update(word: &mut String) {
            word.push_str("RGV");
        }
        update(&mut word);
        println!("{word}")
    }
    fn main(){
        borrow_to_mut_watchout();
    }
        
        fn borrow_to_mut_watchout() {
            let mut word = "UT".to_string(); 
            let ptr_mut = &mut word;
            println!("{word}");
            let ptr_mut1 = &mut word;
            println!("{word}");


            println!("{ptr_mut},{ptr_mut1}")
            /*line 119 creates error because it can be used only
            once since it is being borrowed from mutable*/

            

        }
        fn main(){
            borrow_to_mut_watchout();
        }

        
        fn concat_strings(s1: &String, s2: &String) -> String {
            let result = s1.to_owned()+s2;
            return result;
        }
        
        fn main() {
            let s1 = String::from("Hello, ");
            let s2 = String::from("World!");
            let result = concat_strings(&s1, &s2);
            println!("{}", result); // Should print: "Hello, World!"
        }
            */

            fn clone_and_modify(s: &String) -> String {
                let modified.clone(s);
            }
            
            fn main() {
                let s = String::from("Hello, ");
                let modified = clone_and_modify(&s);
                println!("Original: {}", s); // Should print: "Original: Hello, "
                println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
            }
    