crate::solution!();

fn recursive_find_best_deserts(
    target_cost: i32,
    curr_desert_cost: i32,
    remaining_topping_costs: &[i32],
    best_desert_cost: &mut i32,
) {
    let current_delta_cost = (curr_desert_cost - target_cost).abs();
    let best_delta_cost = (*best_desert_cost - target_cost).abs();

    if current_delta_cost < best_delta_cost {
        *best_desert_cost = curr_desert_cost;
    } else if current_delta_cost == best_delta_cost {
        *best_desert_cost = i32::min(*best_desert_cost, curr_desert_cost);
    }

    if curr_desert_cost > target_cost {
        return;
    }

    match remaining_topping_costs.split_first() {
        Some((&topping_cost, remaining_topping_costs)) => {
            recursive_find_best_deserts(
                target_cost,
                curr_desert_cost,
                remaining_topping_costs,
                best_desert_cost,
            );
            recursive_find_best_deserts(
                target_cost,
                curr_desert_cost + topping_cost,
                remaining_topping_costs,
                best_desert_cost,
            );
            recursive_find_best_deserts(
                target_cost,
                curr_desert_cost + topping_cost * 2,
                remaining_topping_costs,
                best_desert_cost,
            );
        }
        None => {}
    }
}

impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let mut best_desert_cost = i32::MAX;
        base_costs.into_iter().for_each(|base_cost| {
            recursive_find_best_deserts(target, base_cost, &topping_costs, &mut best_desert_cost);
        });

        best_desert_cost
    }
}
