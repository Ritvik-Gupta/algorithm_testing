struct Employee {
    inform_time: i32,
    subordinates: Vec<Link<Employee>>,
}

impl Employee {
    fn new(inform_time: i32) -> Link<Self> {
        Rc::new(RefCell::new(Employee {
            inform_time,
            subordinates: Vec::new(),
        }))
    }

    fn add_subordinate(&mut self, employee: Link<Employee>) {
        self.subordinates.push(employee);
    }

    fn inform_subordinates(&self) -> i32 {
        self.inform_time
            + self
                .subordinates
                .iter()
                .map(|employee| employee.borrow().inform_subordinates())
                .max()
                .unwrap_or(0)
    }
}

crate::solution!();

use std::{cell::RefCell, rc::Rc};

type Link<T> = Rc<RefCell<T>>;

impl Solution {
    pub fn num_of_minutes(_: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut employee_record = HashMap::<usize, Link<Employee>>::new();
        inform_time
            .iter()
            .enumerate()
            .for_each(|(employee_id, &inform_time)| {
                employee_record.insert(employee_id, Employee::new(inform_time));
            });

        manager
            .iter()
            .enumerate()
            .for_each(|(employee_id, &manager_id)| {
                if let Ok(manager_id) = usize::try_from(manager_id) {
                    employee_record[&manager_id]
                        .borrow_mut()
                        .add_subordinate(employee_record[&employee_id].clone());
                }
            });

        let total_time = employee_record[&usize::try_from(head_id).unwrap()]
            .borrow()
            .inform_subordinates();
        employee_record.clear();

        total_time
    }
}
