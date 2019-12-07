use async_std::{fs::File, io::BufReader, prelude::*, task};


fn main() {
    let file_path = "./data/module_masses.txt";
    let mut rocket_calc = RocketCalc::new(file_path);
    
    let total = task::block_on(async {
        rocket_calc.load_and_calc_fuel_for_modules().await
    }).unwrap();
    println!("total: {:?}", total);
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

    async fn load_and_calc_fuel_for_modules(&mut self) -> Result<(i32), String>{
        self.load_file().await.map_err(|err| err.to_string())?;
        self.calc_fuel_for_modules().await
    }

    async fn load_file(&mut self) -> Result<(), std::io::Error> {
        // let contents = fs::read_to_string(&self.file_path)?;
        // self.file_str = Some(contents);

        self.file_line_stream = Some(BufReader::new(File::open(&self.file_path).await?));

        // for line in f.lines() {
        //     println!("{}", line.unwrap());
        // }

        Ok(())
    }



    fn calc_fuel(mass: i32) -> i32 {
        ((mass as f64 / 3_f64).floor() as i32) - 2
    }

    async fn calc_fuel_for_modules(&mut self) -> Result<(i32), String>{
        let mut fuel_total = 0;
        match self.file_line_stream {
            Some(ref mut buf)=>{
                let mut lines = buf.by_ref().lines();
                while let Some(line) = lines.next().await {
                    let line_str = line.map_err(|err| err.to_string())?;
                    // println!("line: {}", line_str);
                    let fuel = line_str.parse::<i32>().map_err(|err| err.to_string())?;
                    fuel_total += RocketCalc::calc_fuel(fuel);
                }

                Ok(fuel_total)
            }
            None=>{
                Err("Buf not loaded!".to_string())
            }
        }

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

    #[test]
    fn calculates_fuel_from_mass_stream() {
        let file_path = "./data/module_masses.txt";
        let mut rocket_calc = RocketCalc::new(file_path);
        
        let total = task::block_on(async {
            rocket_calc.load_and_calc_fuel_for_modules().await
        }).unwrap();


        assert_eq!(total, 3267638);
    }
}




