// ADVENT OF CODE SOLUTION; 
// DAY 1 PART 1
// DAY 1 PART 2
// https://github.com/mkralla11/rocket_equation
// twitter @mkrallapro

use async_std::{fs::File, io::BufReader, prelude::*, task};


fn main() {
    let file_path = "./data/module_masses.txt";
    let mut rocket_calc = RocketCalc::new(file_path);
    
    let fuel_total_without_fuel_mass = task::block_on(async {
        rocket_calc.load_and_calc_fuel_for_modules_basic().await
    }).unwrap();


    let fuel_total_with_fuel_mass = task::block_on(async {
        rocket_calc.load_and_calc_fuel_for_modules_and_fuel_mass().await
    }).unwrap();

    // DAY 1 PART 1
    println!("Fuel total WITHOUT fuel mass compensation: {:?}", fuel_total_without_fuel_mass);
    // DAY 1 PART 2
    println!("Fuel total WITH fuel mass compensation: {:?}", fuel_total_with_fuel_mass);
}


#[derive(Debug)]
struct RocketCalc {
    file_path: String,
    file_line_stream: Option<BufReader<File>>
}

impl RocketCalc {
    fn new<S: Into<String>>(file_path: S) -> RocketCalc {
        RocketCalc {
            file_path: file_path.into(),
            file_line_stream: None
        }
    }
    async fn load_file(&mut self) -> Result<(), std::io::Error> {
        self.file_line_stream = Some(BufReader::new(File::open(&self.file_path).await?));
        Ok(())
    }

    fn calc_fuel(mass: i32) -> i32 {
        ((mass as f64 / 3_f64).floor() as i32) - 2
    }

    async fn load_and_calc_fuel_for_modules_via<F>(&mut self, f: F) -> Result<(i32), String> where F: Fn(i32) -> i32 {
        self.load_file().await.map_err(|err| err.to_string())?;
        self.calc_fuel_for_modules_via(f).await
    }



    async fn calc_fuel_for_modules_via<F>(&mut self, f: F) -> Result<(i32), String> where F: Fn(i32) -> i32 {
        let mut fuel_total = 0;
        match self.file_line_stream {
            Some(ref mut buf)=>{
                let mut lines = buf.by_ref().lines();
                while let Some(line) = lines.next().await {
                    let line_str = line.map_err(|err| err.to_string())?;
                    // println!("line: {}", line_str);
                    let fuel = line_str.parse::<i32>().map_err(|err| err.to_string())?;
                    fuel_total += f(fuel);

                }

                Ok(fuel_total)
            }
            None=>{
                Err("Buf not loaded!".to_string())
            }
        }
    }



    async fn load_and_calc_fuel_for_modules_basic(&mut self) -> Result<(i32), String>{
        let calc_fuel_without_fuel_mass = |module_mass| {
            RocketCalc::calc_fuel(module_mass)
        };

        self.load_and_calc_fuel_for_modules_via(calc_fuel_without_fuel_mass).await
    }


    async fn load_and_calc_fuel_for_modules_and_fuel_mass(&mut self) -> Result<(i32), String>{
        let calc_fuel_without_fuel_mass = |module_mass| {
            let fuel = RocketCalc::calc_fuel(module_mass);
            let mut fuel_mass_total = 0;
            let mut current_fuel_left_for_mass_calc = fuel;
            while current_fuel_left_for_mass_calc > 0 {
                current_fuel_left_for_mass_calc = RocketCalc::calc_fuel(current_fuel_left_for_mass_calc);
                if current_fuel_left_for_mass_calc > 0 {
                    fuel_mass_total += current_fuel_left_for_mass_calc;
                }
            }
            fuel + fuel_mass_total
        };

        self.load_and_calc_fuel_for_modules_via(calc_fuel_without_fuel_mass).await
    }


}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_rocket_calc() {
        let file_path = "./data/module_masses.txt";
        let rocket_calc = RocketCalc::new(file_path);
        assert_eq!(rocket_calc.file_path, file_path);
    }
    #[test]
    fn loads_file() {
        let file_path = "./data/module_masses.txt";
        let mut rocket_calc = RocketCalc::new(file_path);
        let result = task::block_on(async {
            rocket_calc.load_file().await
        });
        assert!(result.is_ok());
    }

    #[test]
    fn calculates_single_fuel() {
        let fuel = RocketCalc::calc_fuel(80891);
        // println!("total: {}", fuel);
        assert_eq!(fuel, 26961);
    }

    // DAY 1 PART 1
    #[test]
    fn load_and_calc_fuel_for_modules_without_fuel_mass() {
        let file_path = "./data/module_masses.txt";
        let mut rocket_calc = RocketCalc::new(file_path);
        
        let total = task::block_on(async {
            rocket_calc.load_and_calc_fuel_for_modules_basic().await
        }).unwrap();


        assert_eq!(total, 3267638);
    }

    // DAY 1 PART 2
    #[test]
    fn load_and_calc_fuel_for_modules_with_fuel_mass() {
        let file_path = "./data/module_masses.txt";
        let mut rocket_calc = RocketCalc::new(file_path);
        
        let total = task::block_on(async {
            rocket_calc.load_and_calc_fuel_for_modules_and_fuel_mass().await
        }).unwrap();

        assert_eq!(total, 4898585);
    }
}




