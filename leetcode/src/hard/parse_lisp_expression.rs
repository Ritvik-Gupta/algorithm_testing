crate::solution!();

use std::{
    collections::{HashMap, HashSet},
    ops::{Index, IndexMut},
    str::Chars,
};

struct ScopedVariables(HashMap<String, Vec<i32>>);

impl ScopedVariables {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn find_constant_or_vairable_value(&self, token: &String) -> i32 {
        token
            .parse::<i32>()
            .unwrap_or_else(|_| *self.0[token].last().unwrap())
    }
}

impl Index<&String> for ScopedVariables {
    type Output = Vec<i32>;

    fn index(&self, index: &String) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<&String> for ScopedVariables {
    fn index_mut(&mut self, index: &String) -> &mut Self::Output {
        self.0.entry(index.clone()).or_insert_with(Vec::new)
    }
}

trait KeywordExpression {
    fn compute_for_scope(
        &self,
        expression: &mut Chars,
        scoped_variables: &mut ScopedVariables,
    ) -> i32;
}

struct LetExpression;
impl KeywordExpression for LetExpression {
    fn compute_for_scope(
        &self,
        expression: &mut Chars,
        scoped_variables: &mut ScopedVariables,
    ) -> i32 {
        let mut added_variables = HashSet::new();

        let mut token_count = 0;
        let mut last_variable_name = String::new();

        let mut collected_word = String::new();

        while let Some(ch) = expression.next() {
            if ch == ')' {
                break;
            }

            if ch.is_whitespace() {
                match token_count % 2 {
                    0 => last_variable_name = collected_word.clone(),
                    _ => {
                        let variable_value =
                            scoped_variables.find_constant_or_vairable_value(&collected_word);

                        let scope = &mut scoped_variables[&last_variable_name];

                        if added_variables.insert(last_variable_name.clone()) {
                            scope.push(variable_value);
                        } else {
                            *scope.last_mut().unwrap() = variable_value;
                        }
                        last_variable_name.clear();
                    }
                }

                collected_word.clear();
                token_count += 1;
            } else if ch == '(' {
                collected_word =
                    compute_scope_recursively(expression, scoped_variables).to_string();
            } else {
                collected_word.push(ch);
            }
        }

        let result = scoped_variables.find_constant_or_vairable_value(&collected_word);

        added_variables.into_iter().for_each(|variable| {
            scoped_variables[&variable].pop();
        });

        result
    }
}

enum Operator {
    ADD,
    MUL,
}

struct OperatorExpression(Operator);
impl KeywordExpression for OperatorExpression {
    fn compute_for_scope(
        &self,
        expression: &mut Chars,
        scoped_variables: &mut ScopedVariables,
    ) -> i32 {
        let mut result = match self.0 {
            Operator::ADD => 0,
            Operator::MUL => 1,
        };
        let mut collected_word = String::new();

        while let Some(ch) = expression.next() {
            if ch == ')' {
                break;
            }

            if ch.is_whitespace() {
                let value = scoped_variables.find_constant_or_vairable_value(&collected_word);
                result = match self.0 {
                    Operator::ADD => result + value,
                    Operator::MUL => result * value,
                };

                collected_word.clear();
            } else if ch == '(' {
                collected_word =
                    compute_scope_recursively(expression, scoped_variables).to_string();
            } else {
                collected_word.push(ch);
            }
        }

        let value = scoped_variables.find_constant_or_vairable_value(&collected_word);
        result = match self.0 {
            Operator::ADD => result + value,
            Operator::MUL => result * value,
        };

        result
    }
}

fn construct_keyword_expression(expression: &mut Chars) -> Box<dyn KeywordExpression> {
    let mut collected_word = String::with_capacity(4);
    while let Some(ch) = expression.next() {
        if ch.is_whitespace() {
            break;
        }
        collected_word.push(ch);
    }
    match collected_word.as_str() {
        "let" => Box::new(LetExpression),
        "add" => Box::new(OperatorExpression(Operator::ADD)),
        "mult" => Box::new(OperatorExpression(Operator::MUL)),
        _ => unreachable!(),
    }
}

fn compute_scope_recursively(
    expression: &mut Chars,
    scoped_variables: &mut ScopedVariables,
) -> i32 {
    construct_keyword_expression(expression).compute_for_scope(expression, scoped_variables)
}

impl Solution {
    pub fn evaluate(expression: String) -> i32 {
        let mut expression = expression.chars();
        expression.next();
        compute_scope_recursively(&mut expression, &mut ScopedVariables::new())
    }
}
