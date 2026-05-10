enum BmiError {
    InvalidInput,
    HealthIssue(f64),
}

struct BmiResult {
    value: f64,
    status: String,
}

fn clean_input(height: &str, weight: &str) -> Result<(f64, f64), ()> {
    let h = height.parse::<f64>().map_err(|_| ())?;
    let w = weight.parse::<f64>().map_err(|_| ())?;

    if h > 0.0 && w > 0.0 {
        Ok((h, w))
    } else {
        Err(())
    }
}

fn calculate_bmi(height: f64, weight: f64) -> Result<BmiResult, BmiError> {
    let bmi = weight / (height * height);

    if bmi >= 18.5 && bmi <= 24.0 {
        Ok(BmiResult {
            value: bmi,
            status: String::from("标准"),
        })
    } 

    else if bmi < 18.5 {
        Ok(BmiResult {
            value: bmi,
            status: String::from("偏瘦"),
        })
    } 

    else {
        Ok(BmiResult {
            value: bmi,
            status: String::from("超重"),
        })
    } 
}

fn main() {
    let input_h = "1.8";
    let input_w = "90.0";

    let final_result = match clean_input(input_h, input_w) {
        Ok((h, w)) => match calculate_bmi(h, w) {
            Ok(res) => format!("BMI: {:.1}, 状态: {}", res.value, res.status),
            Err(BmiError::HealthIssue(val)) => format!("BMI: {:.1}, 警告: 体型存在问题", val),
            _ => String::from("计算层未知错误"),
        },
        Err(_) => String::from("错误：输入必须是正数字"),
    };

    println!("{}", final_result);
}