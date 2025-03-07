use tracing::{debug, info};

pub fn drop_release() {
    info!("drop_release");
    let a = Droppable { name: "a" };
    {
        let _b = Droppable { name: "b" };
        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            info!("exiting block B")
        }
        info!("exiting block A")
    }
    drop(a);
    info!("exiting main");

    let base64 = PackageBuilder::new("base64").version("0.1.3").build();
    debug!("base64: {:?}", base64);
    let log = PackageBuilder::new("log")
        .version("0.4.8")
        .language(Language::Rust)
        .build();
    debug!("log: {:?}", log);
    let serde = PackageBuilder::new("serde")
        .version("1.0.116")
        .authors(vec![
            "Dmitry Chestnykh".to_string(),
            "Serde Developers".to_string(),
        ])
        .dependency(base64.as_dependency())
        .dependency(log.as_dependency())
        .build();
    debug!("serde: {:?}", serde);

    run_pets();
}

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        debug!("Dropping {}", self.name);
    }
}

#[derive(Debug)]
enum Language {
    Rust,
    Java,
    Perl,
}

#[derive(Debug, Clone)]
struct Dependency {
    name: String,
    version_expresion: String,
}

#[derive(Debug, Default)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
    dependencies: Vec<Dependency>,
    language: Option<Language>,
}

impl Package {
    fn as_dependency(&self) -> Dependency {
        Dependency {
            name: self.name.clone(),
            version_expresion: self.version.clone(),
        }
    }
}

struct PackageBuilder(Package);

impl PackageBuilder {
    fn new(name: impl Into<String>) -> Self {
        Self(Package {
            name: name.into(),
            ..Default::default()
        })
    }

    fn version(mut self, version: impl Into<String>) -> Self {
        self.0.version = version.into();
        self
    }

    fn authors(mut self, authors: Vec<String>) -> Self {
        self.0.authors = authors;
        self
    }

    fn dependency(mut self, dependency: Dependency) -> Self {
        self.0.dependencies.push(dependency);
        self
    }

    fn language(mut self, language: Language) -> Self {
        self.0.language = Some(language);
        self
    }

    fn build(self) -> Package {
        self.0
    }
}

struct Dog {
    name: String,
    age: u8,
}
struct Cat {
    lives: i8,
}
trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        format!("{} has {} lives", "Miau", self.lives)
    }
}

fn run_pets() {
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Dog {
            name: "Fido".to_string(),
            age: 10,
        }),
        Box::new(Cat { lives: 9 }),
    ];
    for pet in pets {
        info!("Hello, who are you? {}", pet.talk());
    }

    info!(
        "mem sizeof: dog={}, cat={}",
        std::mem::size_of::<Dog>(),
        std::mem::size_of::<Cat>()
    );
    info!(
        "mem sizeof ref: dog={}, cat={}",
        std::mem::size_of::<&Dog>(),
        std::mem::size_of::<&Cat>()
    );
    info!("mem sizeof dyn pet={}", std::mem::size_of::<&dyn Pet>());
    info!(
        "mem sizeof box dyn pet={}",
        std::mem::size_of::<Box<dyn Pet>>()
    );
}
