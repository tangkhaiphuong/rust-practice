pub mod collection;
pub mod combine;
pub mod first;
pub mod permute;
pub mod second;
pub mod sort;
pub mod third;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fmt::{Display, Error, Formatter};

    struct Person {
        pub name: String,
        pub age: usize,
    }

    impl Person {
        fn new<T>(name: T, age: usize) -> Person
        where
            T: ToString,
        {
            Person {
                name: name.to_string(),
                age: age,
            }
        }
    }

    impl Display for Person {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            write!(f, "({}; {})", self.name, self.age)
        }
    }

    impl Clone for Person {
        fn clone(&self) -> Self {
            Person {
                name: self.name.clone(),
                age: self.age,
            }
        }
    }

    #[test]
    fn test_dictionary() {
        let mut x = HashMap::new();
        x.insert(1, 1);
        x.insert(2, 2);
        x.insert(3, 3);
        x.insert(4, 4);
        match x.get(&1) {
            None => println!("Not found"),
            Some(v) => println!("Go value: {}", v),
        }

        for pair in x.iter() {
            println!("{}:{}", pair.0, pair.1);
        }
    }
}

#[cfg(test)]
mod test_permute {
    use crate::permute::permute;

    #[test]
    fn test_permute() {
        let data = [
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
        ];

        let mut data_mut = data;

        let results = permute(&mut data_mut);

        for result in results {
            for item in result {
                print!("{}", item);
            }
            print!(" ");
        }
    }
}

#[cfg(test)]
mod test_combines {
    use crate::combine::combine;

    #[test]
    fn test_combine() {
        let source: &[&[i32]] = &[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]];

        let results = combine(source);

        for result in results {
            for item in result {
                print!("{}", item);
            }
            print!(" ");
        }
    }
}

#[cfg(test)]
mod test_sorts {
    use core::cell::RefCell;
    use std::fmt::{Display, Error, Formatter};
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    use crate::sort::sort;

    struct Person {
        pub name: String,
        pub age: usize,
    }

    impl Person {
        fn new<T>(name: T, age: usize) -> Person
        where
            T: ToString,
        {
            Person {
                name: name.to_string(),
                age: age,
            }
        }
    }

    impl Display for Person {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
            write!(f, "({}; {})", self.name, self.age)
        }
    }

    impl Clone for Person {
        fn clone(&self) -> Self {
            Person {
                name: self.name.clone(),
                age: self.age,
            }
        }
    }

    #[test]
    fn test_sort() {
        let p1 = Person::new("a", 1);
        let p2 = Person::new("c", 2);
        let p3 = Person::new("d", 4);
        let p4 = Person::new("b", 3);

        let data = [&p1, &p2, &p3, &p4];
        for item in data.iter() {
            print!("{0} ", item);
        }

        println!();

        let mut data = data;
        sort(&mut data, |left, right| left.name > right.name);

        for item in &data {
            print!("{0} ", item);
        }

        println!();
    }

    #[test]
    fn test_sort2() {
        let mut data = [
            Box::new(Person::new("a", 1)),
            Box::new(Person::new("c", 2)),
            Box::new(Person::new("d", 4)),
            Box::new(Person::new("b", 3)),
        ];
        sort(&mut data, |left, right| left.name > right.name);

        for item in &data {
            print!("{0} ", item);
        }

        println!();
    }

    #[test]
    fn move_semantic() {
        let mut vec0 = Vec::new();

        fn fill_vec(vec: &mut Vec<i32>) -> &mut Vec<i32> {
            vec.push(22);
            vec.push(44);
            vec.push(66);

            vec
        }

        let mut vec1 = fill_vec(&mut vec0);

        // Do not change the following line!
        println!("{} has length {} content `{:?}`", "vec0", vec1.len(), vec1);

        vec1.push(88);

        println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    }

    #[test]
    fn sending() {
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val);
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val);
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got: {}", received);
        }
    }

    #[test]
    fn threading_mutex() {
        let counter = Arc::new(Mutex::new(vec![1, 2, 3]));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = counter.clone();

            let handler = thread::spawn(move || {
                if let Ok(mut num) = counter.lock() {
                    num.push(1);
                }
            });

            handles.push(handler);
        }

        for handle in handles {
            handle.join();
        }
    }
}

#[cfg(test)]
mod tests2 {
    use std::borrow::BorrowMut;
    use std::cell::RefCell;

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

    struct MockMessenger {
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
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

#[cfg(test)]
mod tests_future {
    use std::cell::RefCell;
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};

    #[test]
    fn test() {
        let future = hello_world(); // Nothing is printed
                                    //block_on(future); // `future` is run and "hello, world!" is printed
    }

    fn hello_world() -> impl Future<Output = ()> {
        struct FutureImpl {
            flag: RefCell<bool>,
        };

        impl Future for FutureImpl {
            type Output = ();

            fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
                if *self.flag.borrow() {
                    println!("Hello world");
                    Poll::Ready(())
                } else {
                    let mut deref = self.flag.borrow_mut();
                    *deref = true;
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
        };

        FutureImpl {
            flag: RefCell::new(false),
        }
    }
}
