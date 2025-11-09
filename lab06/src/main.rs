use std::{fs, io};

use rusqlite::Connection;
trait Command {
    fn get_name(&self) -> &str {
        "Name"
    }
    fn exec(&mut self, arg: &[&str]);
}

struct PingCommand {}
struct CountCommand {}
struct TimesCommand {
    count: i32,
}
struct FerrisCommand {}
struct StopCommand {}

impl Command for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }
    fn exec(&mut self, _arg: &[&str]) {
        println!("pong!");
    }
}
impl Command for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }
    fn exec(&mut self, arg: &[&str]) {
        let mut cnt = 0;
        for _ in arg {
            cnt += 1;
        }
        println!("{cnt} arguments");
    }
}
impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }
    fn exec(&mut self, _arg: &[&str]) {
        self.count += 1;
        println!("You called 'times' {} times", self.count)
    }
}
impl Command for FerrisCommand {
    fn get_name(&self) -> &str {
        "ferris"
    }
    fn exec(&mut self, _arg: &[&str]) {
        println!("     ,        ,");
        println!("    /(_,    ,_)\\");
        println!("    \\ _/    \\_ /");
        println!("    //        \\\\");
        println!("    \\\\ (@)(@) //");
        println!("     \\'=\"==\"='/");
        println!(" ,===/        \\===, ");
        println!("\",===\\        /===,\"");
        println!("\" ,==='------'===, \"");
        println!(" \"                \"");
    }
}
impl Command for StopCommand {
    fn get_name(&self) -> &str {
        "stop"
    }
    fn exec(&mut self, _arg: &[&str]) {
        std::process::exit(0);
    }
}

struct BookmarkCommand {}
struct Bookmark {
    name: String,
    url: String,
}
impl Command for BookmarkCommand {
    fn get_name(&self) -> &str {
        "bk"
    }
    fn exec(&mut self, arg: &[&str]) {
        let conn = match Connection::open("bookmarks.db") {
            Ok(c) => c,
            Err(err) => {
                println!("Couldn't connect to database: {err}");
                return;
            }
        };
        let create = r"
            create table if not exists bookmarks (
                name text    not null,
                url  text not null
                );
            ";
        match conn.execute(create, ()) {
            Ok(_) => (),
            Err(err) => {
                println!("Execution failed: {err}");
                return;
            }
        };

        let opt = match arg.first() {
            Some(o) => o,
            None => return,
        };

        match *opt {
            "add" => {
                let get = arg.get(1);
                let nume = match get {
                    Some(n) => n,
                    None => return,
                };
                let get = arg.get(2);
                let url = match get {
                    Some(u) => u,
                    None => return,
                };
                match conn.execute(
                    "INSERT INTO bookmarks (name, url) VALUES (?1, ?2);",
                    (nume, url),
                ) {
                    Ok(_) => (),
                    Err(err) => {
                        println!("Execution failed: {err}");
                    }
                };
            }
            "search" => {
                let n = arg.get(1);
                let search = match n {
                    Some(w) => w,
                    None => return,
                };
                let sql = format!("SELECT * FROM bookmarks WHERE name LIKE '%{search}%'");

                let mut stmt = match conn.prepare(&sql) {
                    Ok(s) => s,
                    Err(e) => {
                        println!("Couldn't prepare statement: {e}");
                        return;
                    }
                };

                let person_iter = match stmt.query_map([], |row| {
                    let name: String = match row.get("name") {
                        Ok(v) => v,
                        Err(e) => {
                            println!("Failed to get name: {e}");
                            return Ok(Bookmark {
                                name: "".to_string(),
                                url: "".to_string(),
                            }); // skip this row
                        }
                    };
                    let url: String = match row.get("url") {
                        Ok(v) => v,
                        Err(e) => {
                            println!("Failed to get url: {e}");
                            return Ok(Bookmark {
                                name: "".to_string(),
                                url: "".to_string(),
                            });
                        }
                    };
                    Ok(Bookmark { name, url })
                }) {
                    Ok(iter) => iter,
                    Err(e) => {
                        println!("Query failed: {e}");
                        return;
                    }
                };

                for item in person_iter {
                    match item {
                        Ok(b) => println!("name={}, url={}", b.name, b.url),
                        Err(e) => println!("Error reading row: {e}"),
                    }
                }
            }
            _ => (),
        }
    }
}

fn levensthein(s1: &str, s2: &str, n: usize, m: usize) -> usize {
    if n == 0 {
        return m;
    }

    if m == 0 {
        return n;
    }

    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    if s1_chars[n - 1] == s2_chars[m - 1] {
        return levensthein(s1, s2, n - 1, m - 1);
    }

    let mut l1 = levensthein(s1, s2, n, m - 1);
    let l2 = levensthein(s1, s2, n - 1, m);
    let l3 = levensthein(s1, s2, n - 1, m - 1);

    if l1 > l2 {
        l1 = l2;
    }
    if l1 > l3 {
        l1 = l3;
    }
    l1 + 1
}
fn read_file(path: &str) -> Result<String, io::Error> {
    let s = fs::read_to_string(path)?;
    Ok(s)
}
struct Terminal {
    comms: Vec<Box<dyn Command>>,
}
impl Terminal {
    fn new() -> Self {
        let comms: Vec<Box<dyn Command>> = vec![Box::new(StopCommand {})];
        Terminal { comms }
    }
    fn register(&mut self, arg: Box<dyn Command>) {
        self.comms.push(arg);
    }
    fn suggest(&self, s: &str) {
        let s_to_lower = s.to_lowercase();
        let s_ref = s_to_lower.as_str();
        if s_ref == "🦀" {
            println!("{s} is not a valid command. Did you want to write 'ferris'?");
            return;
        }
        let mut min = usize::MAX;
        let mut suggestion = "";
        for c in &self.comms {
            let d = levensthein(
                c.get_name(),
                s_ref,
                c.get_name().chars().count(),
                s_ref.chars().count(),
            );
            if d < min {
                min = d;
                suggestion = c.get_name();
            }
        }
        println!("{s} is not a valid command. Did you want to write '{suggestion}'?")
    }
    fn run(&mut self) {
        let path = String::from("file.txt");
        let mut file_cont = String::from("");
        match read_file(&path) {
            Ok(x) => file_cont = x,
            Err(e) => println!("{e:?}"),
        }
        let s = file_cont.as_str();
        for l in s.lines() {
            let line = l.trim();
            let mut it = line.split_whitespace();
            let com = it.next();
            if let Some(com) = com {
                let mut arg: Vec<&str> = Vec::new();
                for i in it {
                    arg.push(i);
                }
                let mut ok = false;
                for c in &mut self.comms {
                    if com == c.get_name() {
                        c.exec(&arg);
                        ok = true;
                    }
                }
                if !ok {
                    self.suggest(com);
                }
            }
        }
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(FerrisCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));
    terminal.register(Box::new(BookmarkCommand {}));

    terminal.run();
}
