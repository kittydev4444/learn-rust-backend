fn main() {
    let result = crabby_tasks("gathering coins", 12);
    println!("{}", result);

    let result_2 = crabby_tasks("cooking", 30);
    println!("{}", result_2);

    let result_3 = crabby_tasks("fishing", 45);
    println!("{}", result_3);
}

fn crabby_tasks(task: &str, time: i32) -> String {
  return  format!("Crabby has successfully done {} in {} minutes.", task, time);
}