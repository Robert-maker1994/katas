use super::Arrays;

impl Arrays {
/// Divides players into pairs such that the sum of skills in each pair is equal,
/// and calculates the total chemistry of the pairs.
///
/// # Arguments
///
/// * `skill` - A vector of integers representing the skill levels of the players.
///
/// # Returns
///
/// * `i64` - The total chemistry of the pairs if all pairs have the same sum of skills,
///           otherwise returns -1.
///
/// # Explanation
///
/// The function first sorts the skill levels in ascending order. It then calculates the target
/// skill sum by adding the smallest and largest skill levels. The function iterates through the
/// first half of the sorted skill levels, pairing each element with its corresponding element
/// from the second half. If any pair does not sum to the target skill, the function returns -1.
/// Otherwise, it calculates the total chemistry by multiplying the skill levels of each pair
/// and summing these products.
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let mut skill = skill;
        skill.sort();
        let n = skill.len();
        let target_skill = skill[0] + skill[n - 1];
        let mut total_chemistry = 0;

    
        for i in 0..n / 2 {
            let pair_sum = skill[i] + skill[n - 1 - i];
    
            if pair_sum != target_skill {
                return -1;
            }
    
            total_chemistry += (skill[i] as i64) * (skill[n - 1 - i] as i64);
        }
    
        total_chemistry
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let skill = vec![3,4];
        let res = 12;
        assert_eq!(Arrays::divide_players(skill), res);
    }

    #[test]
    fn test_2() {
        let skill = vec![3,2,5,1,3,4];
        let res = 22;
        assert_eq!(Arrays::divide_players(skill), res);

    }

    #[test]
    fn test_3() {
        let skill = vec![1,1,2,3];
        let res = -1;
        assert_eq!(Arrays::divide_players(skill), res);
    }
}
