//Packages,Crates,Modules, Paths, Workspaces.
//Crate: a binary(executable) or library(sharing functionality).
//Package: one or more crates an contains Cargo.toml to build the crates.
//Package: contains at most one library but many binary crates.
//Package: must contain one crate either library or binary.
//Modules: control privacy of crate items.

//src/lib.rs creates a library crate.

#[cfg(test)]
pub mod tests {
    //-Reborrow Example----------------------------------------------------------/
    #[test]
    fn reborrow_option() {
        let string_opt: &mut Option<String> = &mut None;
        println!("{:?}", string_opt);

        for _n in 1..3 {
            if string_opt.is_none() {
                let real_string = String::from("blah");
                *string_opt = Some(real_string);
                println!("{:?}", string_opt);
            } else if string_opt.is_some() {
                let real_string: &str = string_opt.as_ref().unwrap();
                println!("{:?}", real_string);
            }
        }
    }
    //-Static Vector Example-----------------------------------------------------/
    #[test]
    pub fn static_test() {
        let mut vec = create_new();
        println!("vec:{:?}", vec);

        let address = vec[0].as_ptr();
        let len = vec[0].len();
        println!("data:{} address:{:?} len:{}", vec[0], address, len);

        let address = vec[1].as_ptr();
        let len = vec[1].len();
        println!("data:{} address:{:?} len:{}", vec[1], address, len);

        let st1 = String::from(vec[0]);
        vec[0] = &st1;
        let address = vec[0].as_ptr();
        let len = vec[0].len();
        println!("data:{} address:{:?} len:{}", vec[0], address, len);
    }

    pub fn create_new() -> Vec<&'static str> {
        Vec::from(["abc", "dev"])
    }
    //-Result/Option Matching Example--------------------------------------------/
    #[test]
    pub fn match_option_result_test() {
        let val: Result<_, &str> = Ok("abc");
        let _val = match val {
            Ok(t) => t,
            Err(_) => &"",
        };

        let val: Option<&str> = Some("abc");
        let _val: &str = match val {
            Some(t) => t,
            None => &"",
        };
    }
    //-String To Number Conversion Example---------------------------------------/
    #[test]
    pub fn nums() {
        let c = vec![1, 2].len() + 1;
        assert_eq!(c, 3);
        let _ev = u64::from_str_radix("120013", 16).unwrap();
        assert_eq!(3, _ev & 3);
        let hex = u64::from_str_radix("F", 16).unwrap();
        assert_eq!(15, hex);
        println!("{:?}d", hex);
        let key = "402000000 3803078f800d001 feffffdfffefffff fffffffffffffffe";
        let mut split = key.split_whitespace();
        let main_keys = split.next_back().unwrap();
        let key = u64::from_str_radix(main_keys, 16).unwrap();
        println!("keys:0x{}= {}d", main_keys, key);
    }
    //-Concurrency Example-------------------------------------------------------/
    //Sync and Send std::marker Traits
    //Send - ownership can be transferred between threads.
    //Rc<T> does not implement Send.
    //Sync - type may be referenced from multiple threads.
    // eg if &T is Send then T is sync. (&T is an immutable reference)
    //Arc<T> is the Rc<T> threadsafe equivalent. Atomic Reference Counting.
    //Mutex<T> is the RefCell<T> threadsafe equivalent for interior mutability.
    #[test]
    pub fn mutex_arc_example() {
        use std::sync::{Arc, Mutex};
        use std::thread;
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();

                *num += 1;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }

    //mutual exclusion Mutex<T> example
    #[test]
    pub fn simple_mutex() {
        use std::sync::Mutex;
        let m = Mutex::new(5);

        {
            let mut num = m.lock().unwrap();
            *num = 6;
        }
        println!("m = {:?}", m);
    }

    //Data Between Threads with channel
    #[test]
    pub fn channel_concurrency() {
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;

        //multiple producer single consumer
        //for multiple tx: tx.clone()
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });
        //Receive a single value.
        //let received = rx.recv().unwrap(); // try_recv is not blocking.
        //println!("Got: {}", received);

        //treat rx as an iterator
        for received in rx {
            println!("Got: {}", received);
        }
    }

    ///Spawn a thread
    #[test]
    pub fn concurrency() {
        use std::thread;
        use std::time::Duration;

        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        //closure may out live the current function which owns vec so use 'move'.
        let handle = thread::spawn(move || {
            for i in vec {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        //join to allow the spawned thread to complete.
        handle.join().unwrap();
    }
    //-Weak Ref Example------------------------------------------------------/
    //Weak<T> avoids a 'reference cycle' using weak references.
    // Weak.upgrade() checks a value exists returning Option<Rc<T>>.
    //Rc::clone() increases the strong_count.
    //Rc::downgrade() gives a a smart pointer of type Weak<T> and
    //  increases weak_count.
    //Rustonomicon - The Dark Arts of Unsafe Rust.
    #[test]
    pub fn weak_ref_ex() {
        weak::weak_ex();
    }

    mod weak {
        use std::cell::RefCell;
        use std::rc::{Rc, Weak};

        #[derive(Debug)]
        struct Node {
            _value: i32,
            parent: RefCell<Weak<Node>>,
            _children: RefCell<Vec<Rc<Node>>>,
        }

        pub fn weak_ex() {
            let leaf = Rc::new(Node {
                _value: 3,
                parent: RefCell::new(Weak::new()),
                _children: RefCell::new(vec![]),
            });
            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );

            {
                println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

                let branch = Rc::new(Node {
                    _value: 5,
                    parent: RefCell::new(Weak::new()),
                    _children: RefCell::new(vec![Rc::clone(&leaf)]),
                });

                *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
                println!(
                    "branch strong = {}, weak = {}",
                    Rc::strong_count(&branch),
                    Rc::weak_count(&branch),
                );
                println!(
                    "leaf strong = {}, weak = {}",
                    Rc::strong_count(&leaf),
                    Rc::weak_count(&leaf),
                );
            }

            println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }
    }
    //-Rc<T>, RefCell<T> Example---------------------------------------------/
    // Multiple owers of mutable data Rc<T> with RefCell<T>
    //Cell<T> does not give references but copies the value in and out.
    //Mutex<T> gives interior Mutability across threads.
    #[test]
    pub fn rc_refcell_run() {
        rcref::rc_refcell_example();
    }

    mod rcref {

        #[derive(Debug)]
        enum List {
            Cons(Rc<RefCell<i32>>, Rc<List>),
            Nil,
        }

        use crate::tests::rcref::List::{Cons, Nil};
        use std::cell::RefCell;
        use std::rc::Rc;

        pub fn rc_refcell_example() {
            let value = Rc::new(RefCell::new(5));

            let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

            let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
            let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

            *value.borrow_mut() += 10;

            println!("a after = {:?}", a);
            println!("b after = {:?}", b);
            println!("c after = {:?}", c);
        }
    }

    // RefCell<T> allows mutation of immutable references.
    // Gives single ownership over it's data.

    //Borrow Rules
    // - At any given time, you can have either (but not both of)
    //   one mutable reference or any number of immutable references.
    // - References must always be valid.

    //With references and Box<T>, the borrowing rules’ invariants are enforced
    //at compile time. With RefCell<T>, these invariants are enforced at runtime.
    //With references, if you break these rules, you’ll get a compiler error.
    //With RefCell<T>, if you break these rules, your program will panic and exit.

    // - Rc<T> enables multiple owners of the same data;
    // - Box<T> and RefCell<T> have single owners.

    // - Box<T> allows immutable or mutable borrows checked at compile time;
    // - Rc<T> allows only immutable borrows checked at compile time;
    // - RefCell<T> allows immutable or mutable borrows checked at runtime.

    // RefCell<T> allows mutable borrows checked at runtime, allowing mutation
    // of the value inside the RefCell<T> even when the RefCell<T> is immutable.

    // & - create an immutable reference.   &mut - create a mutable reference.
    // RefCell.borrow() for immutable reference.
    // RefCell.borrow_mut() for mutable reference.
    // Allows many immutable refs but only one mutable.
    #[test]
    pub fn refcell_example() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow_mut().len(), 1);
    }

    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    use std::cell::RefCell;

    struct MockMessenger {
        //Use RefCell to make inner value mutable.
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            //Access sent_messages Vec<String> with borrow_mut()
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }
    //-Dereference Operator Example----------------------------------------------/
    // Deref coercion is a convenience that Rust performs on args to functions and
    //   methods on types that implement the Deref Trait. eg &String to &str.
    // Allows code to work for either references or smart pointers.

    // Deref Coercion with mutability:
    // - From &T to &U when T: Deref<Target=U>
    // - From &mut T to &mut U when T: DerefMut<Target=U>
    // - From &mut T to &U when T: Deref<Target=U
    #[test]
    pub fn deref_operator() {
        let x = 5;
        let y = &x;
        let a = &x;
        let b = MyBox::new(x);
        let z = Box::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
        assert_eq!(5, *a);
        assert_eq!(5, *b);
        assert_eq!(5, *z);

        let m = MyBox::new(String::from("Rust"));
        deref_coersion(&m);
        //do not have to write this because of deref coercion.
        deref_coersion(&(*m)[..]);
    }

    pub fn deref_coersion(name: &str) {
        println!("hello, {}!", &name);
        println!("hello, {}!", name);
    }

    #[derive(Debug)]
    struct MyBox<T: std::fmt::Debug>(T);

    use std::ops::Deref;

    impl<T: std::fmt::Debug> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    //-Drop Example-----------------------------------------------------------/
    #[test]
    pub fn drop_example() {
        let x = 5;
        let y = MyBox::new(x);
        println!("x:{}, y:{:?}", x, y);
        //Note: can not call y.drop() manuall so use drop(y) from std::mem
        // to force early cleanup. drop is in the prelude.
    }

    impl<T: std::fmt::Debug> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    //Note the Drop trait is included in the prelude
    impl<T: std::fmt::Debug> Drop for MyBox<T> {
        fn drop(&mut self) {
            println!("Dropping MyBox: {:?}", self);
        }
    }
    //-Box Example------------------------------------------------------------/
    //Box<T> provide indirection and heap allocation,
    // eg manage recursive types with unknown compile time sizing.
    // Box<T> implementd the Deref trait, allowing values to be
    // treated as references, also implements Drop trait for
    // heap cleanup.
    //Box<T> is a smart pointer.
    #[test]
    pub fn int_on_the_heap() {
        let b = Box::new(5);
        println!("b={}", b);
    }
    //-Shared Ownership Example-----------------------------------------------/
    //Shared immutable ownership example use reference count Rc<T>.
    //The clone does not make a copy but increments reference count.
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use crate::tests::List::{Cons, Nil};
    use std::rc::Rc;

    #[test]
    pub fn shared_immutable_example() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let _b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let _c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }

        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
    //-Simple Ownership Example-----------------------------------------------/
    #[test]
    pub fn test_ownership() {
        let mut _vec: Vec<i32> = Vec::new();
        get_vec(&mut _vec);
        println!("numlist ={:?}", _vec);
        let mut count = 0;
        while count < 10 {
            println!("{}", count);
            count = count + 1;
        }
    }

    pub fn get_vec(ivec: &mut Vec<i32>) {
        ivec.push(1);
    }

    #[test]
    pub fn take_ownership_example() {
        let vals = ["abc", "dev"];
        take_ownership(vals);
    }

    // Function parameters are fixed size at compile time,
    // so pass an array ref or fixed length array.
    pub fn take_ownership(vec: [&str; 2]) {
        let tmp0 = vec[0];
        let tmp1 = vec[1];
        println!("{},{}", tmp0, tmp1);
    }
    //-Compound Type Example--------------------------------------------------/
    //Arrays and Tuples have fixed length.
    #[test]
    pub fn compound() {
        //Tuple
        let tuple: (i32, f64, u8) = (500, 6.4, 1);
        //destructure
        let (_x, y, _z) = tuple;
        println!("The value of y is : {}", y);

        let x: (i32, f64, u8) = (500, 6.4, 1);

        let _five_hundred = x.0;
        let _six_point_four = x.1;
        let _one = x.2;
        //Array
        let _a: [i32; 5] = [1, 2, 3, 4, 5];
        let _a = [3; 5]; // [3,3,3,3,3]
                         //Struct
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }

    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // - method first param is self.
        // - associated functions do not have self as the first param and
        // is called using :: syntax.
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    //-Shadow Example---------------------------------------------------------/
    /*
       Shadowing is different from mut because a new variable is created.
       ie reusing the same name.
    */
    pub fn shadow() {
        let _spaces = "   ";
        let _spaces = _spaces.len();
    }
    //-Trait Example----------------------------------------------------------/
    // A struct with annotation of lifetimes.
    #[test]
    pub fn trait_example() {
        let b: Borrowed = Default::default();
        println!("b is {:?}", b);
        println!("x: {}", b.x);
    }

    #[derive(Debug)]
    struct Borrowed<'a> {
        x: &'a i32,
    }

    // Annotate lifetimes to impl.
    impl<'a> Default for Borrowed<'a> {
        fn default() -> Self {
            Self { x: &10 }
        }
    }

    ////Defining a Trait
    pub trait Summary {
        //fn summarize(&self) -> String;

        //Default impl for summary
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    // function with a parameter with the Summary trait.
    // 'pub fn notify(item: &impl Summary) {' is the same as the
    // Trait Bound Syntax:
    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    //Multiple Trait Bounds:
    //pub fn notify(item: &(impl Summary + Display)) {
    //pub fn notify<T: Summary + Display>(item: &T) {

    //Trait Bounds with where:
    //fn some_function<T, U>(t: &T, u: &U) -> i32
    //    where T: Display + Clone,
    //          U: Clone + Debug
    //{

    //Trait return type:
    //fn returns_summarizable() -> impl Summary {

    //Trait for method implementation:
    //struct Pair<T> {...
    //impl<T: Display + PartialOrd> Pair<T> {...

    //Use Trait objects(eg Box<dyn Draw> instead of Trait bounds
    // to process different objects with a trait. A generic type
    // parameter can only be substituted by one concrete type at a time.

    // Impl a Trait on a type
    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    //-Struct Lifetime Example------------------------------------------------/
    #[test]
    pub fn lifetime_test() {
        let mut owner = Owner(18);
        owner.add_one();
        owner.print();
    }

    struct Owner(i32);

    impl Owner {
        // Annotate lifetimes as in a standalone function.
        fn add_one<'a>(&'a mut self) {
            self.0 += 1;
        }
        fn print<'a>(&'a self) {
            println!("`print`: {}", self.0);
        }
    }
    //-String Lifetime Example------------------------------------------------/
    //Just return the string not dead reference!
    //fn invalid_output<'a>() -> &'a String { &String::from("foo") }
    pub fn invalid_output() -> String {
        String::from("foo")
    }

    #[test]
    pub fn str_test() {
        let mut arg1 = vec!["abc", "def"];
        let arg2 = vec!["ghi", "jkl"];
        let arg3 = str_add(&arg1, &arg2);
        println!("{:?}", arg1);
        println!("{:?}", arg2);
        println!("{:?}", arg3);
        arg1 = str_add(&arg1, &arg2);
        println!("{:?}", arg1);
    }

    pub fn str_add<'a>(arg1: &Vec<&'a str>, arg2: &Vec<&'a str>) -> Vec<&'a str> {
        let mut vec = Vec::new();
        vec.extend_from_slice(arg1);
        vec.extend_from_slice(arg2);
        return vec;
    }

    #[test]
    pub fn string_test() {
        let mut a1: Vec<String> = vec![String::from("abc"), String::from("def")];
        let a2: Vec<String> = vec![String::from("ghi"), String::from("jkl")];
        a1 = string_add(&a1, &a2);
        println!("{:?}", a1);
    }
    pub fn string_add(arg1: &Vec<String>, arg2: &Vec<String>) -> Vec<String> {
        let mut vec = Vec::new();
        vec.extend_from_slice(arg1);
        vec.extend_from_slice(arg2);
        return vec;
    }
    //-Closure Example--------------------------------------------------------/
    #[test]
    pub fn closure_ex() {
        let simulated_user_specified_value = 30;
        let simulated_random_number = 3;
        generate_workout(simulated_user_specified_value, simulated_random_number);
    }

    //Note: Fn is a closure trait.
    //      fn is a function pointer.
    //      fn implements all closure traints: Fn, FnMut, and FnOnce
    struct Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        calculation: T,
        value: Option<u32>,
    }

    impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
    {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }

    fn generate_workout(intensity: u32, random_number: u32) {
        use std::thread;
        use std::time::Duration;

        let mut expensive_result = Cacher::new(|num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });

        if intensity < 25 {
            println!("Today, do {} pushups!", expensive_result.value(intensity));
            println!("Next, do {} situps!", expensive_result.value(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!(
                    "Today, run for {} minutes!",
                    expensive_result.value(intensity)
                );
            }
        }
    }
    //-Vector Generic Function Example----------------------------------------/
    #[test]
    pub fn compare_ex() {
        let number_list = vec![1, 2, 3, 4];
        let &result = largest(&number_list);
        println!("The largest number is {}", result);
        println!("{:?}", number_list);

        let char_list = vec!['a', 'b', 'c', 'A', '#', 'd'];
        let &result = largest(&char_list);
        println!("The largest number is {}", result);
        println!("{:?}", char_list);
    }

    pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > &largest {
                largest = item;
            }
        }
        &largest
    }
    //-File Open Example------------------------------------------------------/
    #[test]
    pub fn open_file_ex() {
        use std::fs::File;
        let f = File::open("hello.txt");
        println!("{:?}", f);
    }
    //-Panic Example----------------------------------------------------------/
    #[test]
    pub fn panic_ex() {
        let _v = vec![1, 2, 3];
        //v[99];
        panic!("crash and burn");
    }
    //-Hashmap Example--------------------------------------------------------/
    #[test]
    pub fn hash_ex() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);

        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }
    //-String Example---------------------------------------------------------/
    #[test]
    pub fn string_ex() {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        let s3 = s1 + &s2;
        println!("s3 is {}", s3);
        println!("s2 is {}", s2);
        let s4 = s2.chars().nth(1);
        println!(
            "s4 is {}",
            match s4 {
                Some('a') => 'a',
                None => '0',
                _ => 'x',
            }
        );
        println!(
            "s4 is {}",
            match s4 {
                Some('a') => 1,
                None => 0,
                _ => 100,
            }
        );
    }
    //-Vector Example---------------------------------------------------------/
    #[test]
    pub fn vec_ex() {
        let mut v: Vec<i32> = vec![1, 2, 3];
        let first = &v[0];
        println!("The first element is: {}", first);
        v.push(4);
        match v.get(5) {
            Some(val) => println!("Hello, world!{}", val),
            None => println!("none"),
        }
    }
    //-Match Example----------------------------------------------------------/
    #[test]
    pub fn match_ex() {
        let x = Some(6);
        //let x = None;
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(1) | Some(2) => println!("1or2"),
            Some(3..=5) => println!("3..5"),
            Some(y) => println!("Matched, y = {:?}", y),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {:?}", x, y);

        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        //Destructure and match
        //Binding y_id with @
        //let p = Point { x: 0, y: 7, z: 5};
        let p = Point { x: 1, y: 0, z: 5 };

        //The 'if' in the match is called a 'match guard'
        match p {
            Point { x, y: y_id @ 0, .. } => println!("On the x axis at {} because y{}", x, y_id),
            Point { x: 0, y, .. } => println!("On the y axis at {}", y),
            Point { x, y, z } if z < 0 => println!("neither axis: ({}, {}, {})", x, y, z),
            Point { x, y, z } => println!("On neither axis: ({}, {}, {})", x, y, z),
        }
    }
    //-Unsafe Rust------------------------------------------------------------/
    #[test]
    pub fn unsafe_example() {
        let mut num = 5;
        let _r1 = &num as *const i32; //unsafe pointer
        let r2 = &mut num as *mut i32; //unsafe pointer

        unsafe {
            *r2 = 6;
            println!("{}", *r2);
        }
    }
}
