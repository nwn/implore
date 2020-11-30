use implore::impl_op;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct X;

#[impl_op]
fn add_assign<'a>(_: &'a mut X, _: u32) {}

#[test]
fn self_lifetime() {
    let mut x = X;
    x += 12;
    assert_eq!(X, x);
}

#[impl_op]
fn deref(_: &X) -> &'static str {
    "X"
}

#[test]
fn static_lifetime() {
    assert_eq!("X", &*X);
}

enum Category<'c> {
    Subject,
    Adverb,
    Verb(core::marker::PhantomData<&'c()>),
    Adjective,
    Object,
}
struct Sentence<'s> {
    subject: &'s str,
    adverb: &'s str,
    verb: &'s str,
    adjective: &'s str,
    object: &'s str,
}
impl<'s> Sentence<'s> {
    fn new(sub: &'s str, adv: &'s str, verb: &'s str, adj: &'s str, obj: &'s str) -> Self {
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
fn index<'s, 'a, 'c, 'b>(sent: &'a Sentence<'s>, cat: &'b Category<'c>) -> &'a &'s str
where 's: 'c, 'b: 's,
    {
    match *cat {
        Category::Subject => &sent.subject,
        Category::Adverb => &sent.adverb,
        Category::Verb(_) => &sent.verb,
        Category::Adjective => &sent.adjective,
        Category::Object => &sent.object,
    }
}

#[impl_op]
fn index_mut<'s, 'a, 'c, 'b>(sent: &'a mut Sentence<'s>, cat: &'b Category<'c>) -> &'a mut &'s str
where 's: 'c, 'b: 's {
    match *cat {
        Category::Subject => &mut sent.subject,
        Category::Adverb => &mut sent.adverb,
        Category::Verb(_) => &mut sent.verb,
        Category::Adjective => &mut sent.adjective,
        Category::Object => &mut sent.object,
    }
}

#[test]
fn many_lifetimes() {
    let mut sentence = Sentence::new("I", "quickly", "ate", "many", "scones");
    assert_eq!("I", sentence[&Category::Subject]);
    assert_eq!("quickly", sentence[&Category::Adverb]);
    assert_eq!("ate", sentence[&Category::Verb(core::marker::PhantomData)]);
    assert_eq!("many", sentence[&Category::Adjective]);
    assert_eq!("scones", sentence[&Category::Object]);

    sentence[&Category::Subject] = "You";
    assert_eq!("You", sentence[&Category::Subject]);
}
