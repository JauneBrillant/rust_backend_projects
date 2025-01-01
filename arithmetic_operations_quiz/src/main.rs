use rand::Rng;

fn main() {
    let mut correct_count = 0;
    while correct_count < 3 {
        let (problem_statement, ans) = create_problem();
        println!("{}", problem_statement);

        println!("??の値を入力して下さい:");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: i32 = input.trim().parse().expect("Invalid input");

        if input == ans {
            println!("正解!\n");
            correct_count += 1;
        } else {
            println!("不正解!\n")
        }
    }

    println!("クリア!");
}

fn create_problem() -> (String, i32) {
    let mut rng = rand::thread_rng();
    let (start, end) = (0, 100);
    let lhs = rng.gen_range(start..end);
    let rhs = rng.gen_range(start..end);
    let operater = if rng.gen_bool(0.5) { "+" } else { "-" };
    let ans = if operater == "+" {
        lhs + rhs
    } else {
        lhs - rhs
    };
    let problem_statement = format!("{} {} {} = ??", lhs, operater, rhs);
    (problem_statement, ans)
}
