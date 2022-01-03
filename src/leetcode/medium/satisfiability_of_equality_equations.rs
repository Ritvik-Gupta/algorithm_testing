crate::leetcode::solution!();

struct Equation {
    left_var: char,
    are_equal: bool,
    right_var: char,
}

impl Equation {
    fn from(eqn_string: &String) -> Self {
        let mut eqn = eqn_string.chars();
        let (left_var, are_equal) = (eqn.next().unwrap(), eqn.next().unwrap() == '=');
        eqn.next();
        Self {
            left_var,
            are_equal,
            right_var: eqn.next().unwrap(),
        }
    }
}

impl Solution {
    pub fn equations_possible(equations: Vec<String>) -> bool {
        use std::collections::BTreeMap;

        let (mut variable_record, mut incremental_value) = (BTreeMap::new(), 0);

        for equation in equations
            .iter()
            .map(Equation::from)
            .filter(|eqn| eqn.are_equal)
        {
            if !variable_record.contains_key(&equation.left_var) {
                variable_record.insert(equation.left_var, incremental_value);
                incremental_value += 1;
            }

            if !variable_record.contains_key(&equation.right_var) {
                variable_record.insert(equation.right_var, variable_record[&equation.left_var]);
            }

            if variable_record[&equation.right_var] != variable_record[&equation.left_var] {
                return false;
            }
        }

        for equation in equations
            .iter()
            .map(Equation::from)
            .filter(|eqn| !eqn.are_equal)
        {
            if !variable_record.contains_key(&equation.left_var) {
                variable_record.insert(equation.left_var, incremental_value);
                incremental_value += 1;
            }

            if !variable_record.contains_key(&equation.right_var) {
                variable_record.insert(equation.right_var, incremental_value);
                incremental_value += 1;
            }

            if variable_record[&equation.right_var] == variable_record[&equation.left_var] {
                return false;
            }
        }

        true
    }
}
