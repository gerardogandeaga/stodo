#[allow(dead_code)]

pub mod gutter;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gutter() -> Result<(), String> {
        let x = gutter::Gutter::new();
        
        println!("{}", x);
        assert!(true, "it's true!");
        Ok(())
    }
}


// pub fn test() {
//     println!("├");
//     println!("│");
//     println!("└");
//     println!("─");
//     println!(" ");
// }
