use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op]
fn index(_: &X, idx: bool) -> &'static str {
    if idx {
        "true"
    } else {
        "false"
    }
}

#[test]
fn immutable_index() {
    assert_eq!("true", &X[true]);
    assert_eq!("false", &X[false]);
}

enum Category {
    Subject,
    Adverb,
    Verb,
    Adjective,
    Object,
}
struct Sentence {
    subject: &'static str,
    adverb: &'static str,
    verb: &'static str,
    adjective: &'static str,
    object: &'static str,
}
impl Sentence {
    fn new(sub: &'static str, adv: &'static str, verb: &'static str, adj: &'static str, obj: &'static str) -> Self {
        Self {
            subject: sub,
            adverb: adv,
            verb: verb,
            adjective: adj,
            object: obj,
        }
    }
}

#[impl_op]
fn index(sent: &Sentence, cat: Category) -> &&'static str {
    match cat {
        Category::Subject => &sent.subject,
        Category::Adverb => &sent.adverb,
        Category::Verb => &sent.verb,
        Category::Adjective => &sent.adjective,
        Category::Object => &sent.object,
    }
}

#[impl_op]
fn index_mut<'a>(sent: &'a mut Sentence, cat: Category) -> &'a mut &'static str {
    match cat {
        Category::Subject => &mut sent.subject,
        Category::Adverb => &mut sent.adverb,
        Category::Verb => &mut sent.verb,
        Category::Adjective => &mut sent.adjective,
        Category::Object => &mut sent.object,
    }
}

#[test]
fn index() {
    let mut sentence = Sentence::new("I", "quickly", "ate", "many", "scones");
    assert_eq!("quickly", sentence[Category::Adverb]);
    sentence[Category::Subject] = "You";
    assert_eq!("You", sentence[Category::Subject]);
}
