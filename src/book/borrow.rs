use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use tracing::info;

pub fn borrow_run() {
    info!("hello borrow world");
    let mut a = 10;

    {
        let c = &mut a;
        *c = 100;
    }
    info!("a is {}", a);

    let b = &mut a;
    *b += 10;

    info!("b is {}", b);

    let root = Node::new(1);
    root.borrow_mut().children.push(Node::new(2));
    let subtree = Node::new(3);
    subtree.borrow_mut().children.push(Node::new(14));
    subtree.borrow_mut().children.push(Node::new(15));
    root.borrow_mut().children.push(subtree);
    // info!("root is {:?}", root);
    info!("root sum is {}", dbg!(root).borrow().sum());

    let celld = Cell::new(10);
    info!("celld is {}", celld.get());
    celld.set(100);
    info!("celld after set is {}", celld.get());

    let bob = User::new("bob".to_owned(), 20, 130.0);
    info!("i am {} and my age is {}", bob.name, bob.age);
}

#[derive(Debug, Default)]
struct Node {
    value: i64,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: i64) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value,
            ..Default::default()
        }))
    }

    fn sum(&self) -> i64 {
        self.value + self.children.iter().map(|c| c.borrow().sum()).sum::<i64>()
    }
}

struct User {
    name: String,
    age: u8,
    height: f32,
    visit_count: usize,
    last_blood_measure: Option<(f32, f32)>,
}

struct Measurements {
    height: f32,
    blood_pressure: (f32, f32),
}

struct HeathReport<'a> {
    patient_name: &'a str,
    visit_count: usize,
    height_change: f32,
    blood_pressure_change: Option<(f32, f32)>,
}

impl User {
    fn new(name: String, age: u8, height: f32) -> Self {
        Self {
            name,
            age,
            height,
            visit_count: 0,
            last_blood_measure: None,
        }
    }

    fn visit_doctor(
        &mut self,
        Measurements {
            height,
            blood_pressure: (x, y),
        }: Measurements,
    ) -> HeathReport {
        let report = HeathReport {
            patient_name: &self.name,
            visit_count: self.visit_count + 1,
            height_change: height - self.height,
            blood_pressure_change: match self.last_blood_measure {
                Some((x1, y1)) => Some((x - x1, y - y1)),
                None => None,
            },
        };
        self.visit_count += 1;
        self.height = height;
        self.last_blood_measure = Some((x, y));
        report
    }
}

#[test]
fn test_visit_doctor() {
    let mut user = User::new("bob".to_owned(), 20, 130.0);
    assert_eq!(user.visit_count, 0);
    let report = user.visit_doctor(Measurements {
        height: 131.0,
        blood_pressure: (120.0, 80.0),
    });
    assert_eq!(report.patient_name, "bob");
    assert_eq!(report.visit_count, 1);
    assert_eq!(report.blood_pressure_change, None);

    let report = user.visit_doctor(Measurements {
        height: 142.0,
        blood_pressure: (100.0, 70.0),
    });
    assert_eq!(report.visit_count, 2);
    assert_eq!(report.blood_pressure_change, Some((-20.0, -10.0)));
}
