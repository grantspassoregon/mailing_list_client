#[derive(Clone)]
pub struct Files {
    pub queue: Vec<std::path::PathBuf>,
    pub contents: Vec<std::path::PathBuf>,
    pub err: Option<String>,
    pub selected: Option<std::path::PathBuf>,
}

impl Files {
    pub fn new() -> Self {
        let cwd = match std::env::current_dir() {
            Ok(path) => path,
            Err(_) => "./".into(),
        };
        let mut files = Self {
            queue: vec![cwd],
            contents: Vec::new(),
            err: None,
            selected: None,
        };
        files.load();
        files
    }

    pub fn load(&mut self) {
        let path = self.queue.last().unwrap();
        let paths = match std::fs::read_dir(path) {
            Ok(e) => e,
            Err(e) => {
                self.err = Some(e.to_string());
                self.queue.pop();
                return;
            }
        };
        let collected = paths.collect::<Vec<_>>();

        self.clear_err();
        self.contents.clear();

        for path in collected {
            self.contents
                .push(path.expect("Could not read file name.").path());
        }
    }

    pub fn clear_err(&mut self) {
        self.err = None;
    }

    pub fn current(&self) -> &std::path::PathBuf {
        self.queue.last().unwrap()
    }

    pub fn enter(&mut self, id: usize) {
        let path = &self.contents[id];
        self.selected = Some(path.clone());
        self.queue.push(path.clone());
        self.load();
    }

    pub fn forward(&mut self) {
        if self.queue.len() > 1 {
            self.queue.pop();
        }
        self.load();
    }

    pub fn home(&mut self) {
        let cwd = match std::env::current_dir() {
            Ok(path) => path,
            Err(_) => "./".into(),
        };
        self.queue.push(cwd);
        self.load();
    }

    pub fn selected(&self) -> String {
        match self.selected.clone() {
            Some(value) => value.to_str().unwrap().to_string(),
            None => "".to_string(),
        }
    }

    pub fn up(&mut self) {
        if let Some(path) = self.queue.last() {
            if let Some(parent) = path.parent() {
                self.queue.push(parent.to_path_buf());
            }
        }
        self.load();
    }
}
