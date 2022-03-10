// basic likeな for が書けるマクロを定義
macro_rules! easy_for {
    (
        for $i:ident = $from:tt to $to:tt
        $block:block
    ) => {{
        for $i in $from..=$to {
            $block
        }
    }};
    (
        for $i:ident = $from:tt to $to:tt step $step:tt
        $block:block
    ) => {{
        let mut $i = $from;
        loop {
            if $i > $to { break }
            $block
            $i += $step
        }
    }};
}

fn main() {
    // マクロを利用して1から10の合計を求める --- (*4)
    let mut total = 0;
    easy_for! {
        for i = 1 to 10 {
            total += i;
        }
    }
    println!("{}", total);
    // マクロを利用して0から10まで3刻みで表示する --- (*5)
    easy_for! {
        for i = 0 to 10 step 3 {
            println!("i={}", i);
        }
    }
}
