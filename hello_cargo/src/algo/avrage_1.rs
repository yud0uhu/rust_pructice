fn main(){
    let score:[i64; 5] = [70, 98, 92, 88, 64];
    let mut _total = 0;
    let mut _average = 0;
    for i in 0..5 {
        _total = _total + score[i];
    }
    _average = _total / 5;
    println!("合計点={}", _total);
    println!("平均点={}", _average);
}
