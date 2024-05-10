fn main() {
    let skewers: [&str; 5] = [
        "--xo--x--ox--",
        "--xx--x--xx--",
        "--oo--o--oo--",
        "--xx--x--ox--",
        "--xx--x--ox--"
    ];

    println!("{}", analyze(skewers));
}

fn analyze(skewers: [&str; 5]) -> String {
    let skewers_iter = skewers.iter();
    let mut meet = 0;
    let mut vegg = 0;

    for skewer in skewers_iter {
        if skewer.contains("-x") {
            meet += 1;
            continue;
        }
        vegg += 1;
    }

    format!("[{vegg}, {meet}]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze() {
        let skewers_1: [&str; 5] = [
            "--xo--x--ox--",
            "--xx--x--xx--",
            "--oo--o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--"
        ];

        let skewers_2: [&str; 5] = [
            "--oooo-ooo--",
            "--xx--x--xx--",
            "--o---o--oo--",
            "--xx--x--ox--",
            "--xx--x--ox--"
        ];

        let skewers_3: [&str; 5] = [
            "--oooo-ooo--",
            "--xxxxxxxx--",
            "--o---",
            "-o-----o---x--",
            "--o---o-----",
        ];

        assert_ne!(analyze(skewers_1), "[1, 4]");
        assert_eq!(analyze(skewers_2), "[2, 3]");
        assert_eq!(analyze(skewers_3), "[3, 2]");
    }

}
