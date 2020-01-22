#![allow(dead_code)]
#![allow(unused_imports)]

#[macro_use]
extern crate impl_ops;

use std::{ops, thread};
use std::borrow::Borrow;
use std::cell::{Cell, RefCell, RefMut};
use std::collections::{HashMap, LinkedList};
use std::fmt::{Display, Formatter};
use std::future::Future;
use std::ops::{Add, Deref};
use std::rc::Rc;
use std::slice::SliceIndex;
use std::sync::{Arc, Mutex, RwLock};

// use collection::LinkedList;
//
// mod collection;

async fn get() -> isize {
    print!("Hello");
    2
}

fn get2() -> Box<dyn Future<Output=isize>> {
    let x = async {
        print!("Hello");
        2
    };
    Box::new(x)
}

fn main() {
}
