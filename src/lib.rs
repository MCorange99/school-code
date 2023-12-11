use std::{collections::HashMap, sync::RwLock};

use lazy_static::lazy_static;

lazy_static!(
    pub static ref RUNNER: RwLock<Runner> = {
        RwLock::new(Runner {
            tasks: HashMap::new()
        })
    };
);

pub type TaskFn = fn();
pub type TaskNum = usize;
pub struct Runner {
    tasks: HashMap<TaskNum, TaskFn>,
}

impl Runner {
    #[inline]
    pub fn add(&mut self, n: TaskNum, task: TaskFn) {
        self.tasks.insert(n, task);
    }

    pub fn run_one(&self, n: TaskNum){
        let Some(task) = self.tasks.get(&n) else {
            println!("ERROR: Unable to find task {n}");
            return;
        };

        println!("==== Running task {n} ====");
        task();
        println!("==== Task {n} exited ====")
    }

    pub fn run_all(&self) {
        for n in self.tasks.keys() {
            self.run_one(*n);
                
        }
    }
}

#[macro_export]
macro_rules! add_task {
    ($n:literal, $task:expr) => {
        match school_code::RUNNER.write() {
            Ok(mut r) => {
                r.add($n, $task);
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    };
    ($n:literal, $task:expr) => {
        match school_code::RUNNER.write() {
            Ok(mut r) => {
                r.add($n, $task);
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    };
}

#[macro_export]
macro_rules! run_task {
    ($n:literal) => {
        match school_code::RUNNER.read() {
            Ok(mut r) => {
                r.run_one($n);
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    };
    (all) => {
        match school_code::RUNNER.read() {
            Ok(mut r) => {
                r.run_all();
            }
            Err(e) => {
                eprintln!("{e}");
            }
        }
    };
}