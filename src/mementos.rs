pub trait Command: 'static {
    fn execute(&self);
    fn undo(&self);
}

pub struct MementoManager{
    commands: Vec<Box<dyn Command + 'static>>
}

impl MementoManager {
    pub fn new() -> Self {
        return MementoManager { commands: vec![] };
    }

    pub fn execute_command(self: &mut Self, c: impl Command + 'static) {
        c.execute();
        self.commands.push(Box::new(c));
    }
    
    pub fn undo_last_command(self: &mut Self) {
        let command = self.commands.pop();
        match command {
            Some(cmd) => {
                cmd.undo();
            }
            None => panic!()
        }
    }
}