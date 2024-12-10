use std::fs;

fn get_list_from_text(left_list: &mut Vec<i32>, right_list: &mut Vec<i32>) {
    let contents: String = fs::read_to_string("src/listsText.txt")
    .expect("Should have been able to readfile.");

    for line in contents.lines() {
        let numbers: Vec<&str> = line.split("   ").collect();

        if numbers.len() == 2 {
            if let Ok(left_num) = numbers[0].trim().parse::<i32>() {
                left_list.push(left_num);
            }

            if let Ok(right_num) = numbers[1].trim().parse::<i32>() {
                right_list.push(right_num);
            }
        }
    }
    left_list.sort();
    right_list.sort();
}

fn generate_diff(left_list: &mut Vec<i32>, right_list: &mut Vec<i32>) -> i32 {
    let mut diff_list: Vec<i32> = Vec::new();
    
    for idx in 0..left_list.len() {
        let diff = (left_list[idx] - right_list[idx]).abs();
        diff_list.push(diff);
    }
    
    diff_list.iter().sum()
}

fn score_similarity(left_list: &mut Vec<i32>, right_list: &mut Vec<i32>,) -> i32 {
    let mut sim_scores: Vec<i32> = Vec::new();

    for idx in 0..left_list.len() {
        let curr_num = left_list[idx];
        let sim_count = right_list.iter().filter(|&&n| n == curr_num).count();
        sim_scores.push(curr_num * sim_count as i32);
    }
    sim_scores.iter().sum()
}

fn main() {
    const DIFF_ANS: i32 = 3714264;
    const SIM_SCORE_ANS: i32 = 18805872;
    
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    get_list_from_text(&mut left_list, &mut right_list);
    let diff_sum: i32 = generate_diff(&mut left_list, &mut right_list);
    let sim_score: i32 =  score_similarity(&mut left_list, &mut right_list);
    
    // Check PT 1 works:
    if diff_sum == DIFF_ANS {
        println!("Pt1 == True");
    }
    else {
        println!("Pt1 == False");
    }

    // Check PT 2 works: 
    if sim_score == SIM_SCORE_ANS {
        println!("Pt2 == True");
    }
    else {
        println!("Pt2 == False");
    }
}
