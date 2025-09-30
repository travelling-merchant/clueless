pub mod quotes {
    #[derive(PartialEq, Clone)]
    pub struct Entry {
        pub text: String,
        pub author: String,
        pub theme: String,
        pub probability: u8,
    }
    pub fn get_quotes() -> Vec<Entry> {
        let mut m: Vec<Entry> = Vec::new();
        let a = Entry {
            text: "If debugging is the process of removing software bugs, then programming must be the process of putting them in".to_string(),
            author: "Edsger W. Dijkstra".to_string(),
            theme: "Cyberspace".to_string(),
            probability: 5,
        };
        let b = Entry {
            text: "Object-oriented programming is an exceptionally bad idea which could only have originated in California.".to_string(),
            author: "Edsger W. Dijkstra".to_string(),
            theme: "Cyberspace".to_string(),
            probability: 5,
        };
        let c = Entry {
            text: "The use of COBOL cripples the mind; its teaching should, therefore, be regarded as a criminal offense".to_string(),
            author: "Edsger W. Dijkstra".to_string(),
            theme: "Cyberspace".to_string(),
            probability: 5,
        };
        let d = Entry {
            text: "Computer science is no more about computers than astronomy is about telescopes"
                .to_string(),
            author: "Edsger W. Dijkstra".to_string(),
            theme: "Cyberspace".to_string(),
            probability: 5,
        };
        let e = Entry {
            text: "It is practically impossible to teach good programming to students that have had a prior exposure to BASIC: as potential programmers they are mentally mutilated beyond hope of regeneration".to_string(),
            author: "Edsger W. Dijkstra".to_string(),
            theme: "Cyberspace".to_string(),
            probability: 5,
        };
        m.push(a);
        m.push(b);
        m.push(c);
        m.push(d);
        m.push(e);
        m
    }
}
