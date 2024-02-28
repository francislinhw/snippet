use chrono::{NaiveDate, Duration};

fn main() {
    let call_buy: f64 = 15.0;
    let put_buy: f64 = 10.0;
    let call_sell: f64 = 10.0;
    let put_sell: f64 = 6.0;
    let stock_buy: f64 = 2000.0;
    let stock_sell: f64 = 2500.0;
    let create_date: NaiveDate = NaiveDate::from_ymd(2023, 10, 15);
    let today_date: NaiveDate = NaiveDate::from_ymd(2023, 11, 20);
    let return_threshold: f64 = 0.08;
    let opion_trading_fee_open: f64 = 0.01;
    let opion_trading_fee_today: f64 = 0.01;

    let result: bool = exit_permit(call_buy, put_buy, call_sell, put_sell, stock_buy, stock_sell, create_date, today_date, return_threshold, opion_trading_fee_open, opion_trading_fee_today);
    println!("Result: {}", result);
}

fn exit_permit(
    call_buy: f64,
    put_buy: f64,
    call_sell: f64,
    put_sell: f64,
    stock_buy: f64,
    stock_sell: f64,
    create_date: NaiveDate,
    today_date: NaiveDate,
    return_threshold: f64,
    opion_trading_fee_open: f64,
    opion_trading_fee_today: f64) -> bool {
    let syntetic_return: f64 = call_sell - call_buy + put_sell - put_buy + stock_sell - stock_buy;
    let position_cost: f64 = put_buy + stock_buy - call_buy + opion_trading_fee_open + opion_trading_fee_today;
    let expected_return: f64 =  position_cost * (1.0 + return_threshold * ((today_date - create_date).num_days() as f64) / 365.0) - position_cost;

    if syntetic_return >= expected_return {
        println!("Syntetic return: {}", syntetic_return);
        println!("Expected return: {}", expected_return);
        return true;
    } 

    println!("Syntetic return: {}", syntetic_return);
    println!("Expected return: {}", expected_return);
    return false;
}