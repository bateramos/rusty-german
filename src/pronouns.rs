use crate::types::Pronoun;

pub fn get_personal_pronouns() -> [Pronoun; 4] {
    [Pronoun {
        name: "Nominativ",
        subjects: ["ich", "du", "er sie es", "wir", "ihr", "Sie"]
    },Pronoun {
        name: "Akkusativ",
        subjects: ["mich", "dich", "ihn sie es", "uns", "euch", "Sie"]
    },Pronoun {
        name: "Dativ",
        subjects: ["mir", "dir", "ihm ihr ihm", "uns", "euch", "Ihnen"]
    },Pronoun {
        name: "Genitiv",
        subjects: ["mein", "dein", "sein ihr sein", "unser", "euer", "Ihr"]
    }]
}
