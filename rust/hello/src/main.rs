fn main() {
    println!("Hello, world!");
    println!("Hello, {} !", "hoge",);
    println!(
        // {:.n} 少数n桁まで表示
        "半径{:.1} 円周率{:.100} 面積{:.100}",
        3.22,
        std::f64::consts::PI,
        3.2f64.powi(2) * std::f64::consts::PI,
    );
}

https://discordapp.com/api/guilds/742699905461649499/widget.json
