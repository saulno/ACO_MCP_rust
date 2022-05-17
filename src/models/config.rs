pub enum Config {
    Help(),
    VertexAC(String, usize, usize, f32, f32, f32, f32),
    EdgeAC(String, usize, usize, f32, f32, f32, f32),
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } 

        match &args[1][..] {
            "help" => return Ok(Config::Help()),
            "vertex-ac" => {
                let res = match Config::parse_args(args) {
                    Ok(result) => result,
                    Err(e) => return Err(e)
                    
                };
                
                return Ok(Config::VertexAC(res.0, res.1, res.2, res.3, res.4, res.5, res.6));
            },
            "edge-ac" => {
                let res = match Config::parse_args(args) {
                    Ok(result) => result,
                    Err(e) => return Err(e)
                    
                };
                
                return Ok(Config::EdgeAC(res.0, res.1, res.2, res.3, res.4, res.5, res.6));
            },
            _ => return Err("Please execute a valid option (help, vertex-ac, edge-ac)")
        }
    }

    fn parse_args(args: &[String]) -> Result<(String, usize, usize, f32, f32, f32, f32), &str> {
        if args.len() < 9 {
            return  Err("Not enough arguments (input, cycles, ants, alpha, rho, tau_max, tau_min)");
        }

        let cycles: usize = match args[3].parse() {
            Ok(num) => num,
            Err(_) => return Err("Integer expected for second argument (cycles)")
        };
        let ants: usize = match args[4].parse() {
            Ok(num) => num,
            Err(_) => return Err("Integer expected for third argument (ants)")
        };
        let alpha: f32 = match args[5].parse() {
            Ok(num) => num,
            Err(_) => return Err("Float expected for fourth argument (alpha)")
        };
        let rho: f32 = match args[6].parse() {
            Ok(num) => num,
            Err(_) => return Err("Float expected for fifth argument (rho)")
        };
        let tau_max: f32 = match args[7].parse() {
            Ok(num) => num,
            Err(_) => return Err("Float expected for sixth argument (tau_max)")
        };
        let tau_min: f32 = match args[8].parse() {
            Ok(num) => num,
            Err(_) => return Err("Integer expected for seventh argument (tau_min)")
        };

        Ok((args[2].clone(), cycles, ants, alpha, rho, tau_max, tau_min))
    }
}