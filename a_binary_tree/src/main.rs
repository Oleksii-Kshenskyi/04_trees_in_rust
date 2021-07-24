use tree::BinaryTree;

mod tree;

// Binary tree should implement these operations:
// - Find element;
// - Insert element;
// - Remove element.
fn s(s: &str) -> String {
    s.to_string()
}

fn main() {
    let mut tree = BinaryTree::<String, String>::new();

    tree.insert(s("One"), s("1"));
    assert_eq!(Some(s("1")), tree.find(&s("One")));
    tree.insert(s("Two"), s("2"));
    assert_eq!(Some(s("2")), tree.find(&s("Two")));
    tree.insert(s("3"), s("Three"));
    assert_eq!(Some(s("3")), tree.find(&s("Three")));
    assert_eq!(None, tree.find(&s("KEKW")));

    assert_eq!(Ok(()), tree.delete(&s("Three")));
    assert_eq!(Err(()), tree.delete(&s("OMEGA")));
    assert_eq!(None, tree.find(&s("Three")));
    assert_eq!(Some(s("2")), tree.find(&s("Two")));
    assert_eq!(Some(s("1")), tree.find(&s("One")));

    assert_eq!(Ok(()), tree.delete(&s("Two")));
    assert_eq!(None, tree.find(&s("Three")));
    assert_eq!(None, tree.find(&s("Two")));
    assert_eq!(Some(s("1")), tree.find(&s("One")));

    assert_eq!(Ok(()), tree.delete(&s("One")));
    assert_eq!(None, tree.find(&s("Three")));
    assert_eq!(None, tree.find(&s("Two")));
    assert_eq!(None, tree.find(&s("One")));
    assert_eq!(true, tree.is_empty());
}
