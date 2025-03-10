const MONEY_TYPE: [u32; 8] = [100, 50, 30, 20, 10, 5, 2, 1];
pub fn dp_rec_mc(amount: u32) -> u32 {
    // TODO: 这里写逻辑
    let mut last = amount;
    let mut money_count = 0;
    for money in MONEY_TYPE {
        if last >= money {
            money_count += last / money;
            last = last % money;
        }
        if last == 0 {
            break;
        }
    }
    money_count
}
