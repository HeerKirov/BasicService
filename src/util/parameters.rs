use std::collections::{HashSet, HashMap};

pub struct Parameters {
    commands: Vec<String>,
    flags: HashSet<String>,
    params: HashMap<String, String>
}
impl Parameters {
    pub fn new(args: Vec<String>) -> Self {
        let mut commands = Vec::new();
        let mut flags = HashSet::new();
        let mut params = HashMap::new();

        let mut command = true;
        let mut param = Option::None;

        for arg in &args[1..] {
            if arg.starts_with("--") {
                command = false;
                if let None = param {
                    param = Some(arg[2..].to_string());
                }else{
                    panic!("value of {} cannot starts with --", param.unwrap());
                }
            }else if arg.starts_with("-") {
                flags.insert(arg[1..].to_string());
            }else if let Some(key) = param {
                params.insert(key, arg.to_string());
                param = None;
            }else if command {
                commands.push(arg.to_string());
            }else{
                panic!("{} is an alone param", arg);
            }
        }
        Self { commands, flags, params }
    }
    pub fn side<F>(&self, execution: F) -> &Self where F: Fn(&Parameters) {
        execution(self);
        self
    }
    pub fn execute<F>(&self, execution: F) where F: Fn(&str, &Parameters) {
        if self.commands.len() > 0 {
            let command = self.commands.get(0).unwrap();
            let parameters = Parameters {
                commands: self.commands[1..].to_vec(),
                flags: self.flags.clone(),
                params: self.params.clone()
            };
            execution(command, &parameters);
        }else{
            panic!("no next command");
        }
    }
    pub fn param(&self, key: &str, default: String) -> String {
        if let Some(ref value) = self.params.get(key) {
            value.to_string()
        }else{
            default
        }
    }
}