use quill::document::{Element, ElementType, Screenplay};

#[test]
fn test_element_is_empty() {
    let empty = Element::empty(ElementType::Action);
    assert!(empty.is_empty());

    let whitespace = Element::new(ElementType::Action, "   ".to_string());
    assert!(whitespace.is_empty());

    let content = Element::new(ElementType::Action, "Text".to_string());
    assert!(!content.is_empty());
}

#[test]
fn test_formatted_content() {
    let scene = Element::new(
        ElementType::SceneHeading,
        "int. coffee shop - day".to_string(),
    );
    assert_eq!(scene.formatted_content(), "INT. COFFEE SHOP - DAY");

    let action = Element::new(ElementType::Action, "John walks in.".to_string());
    assert_eq!(action.formatted_content(), "John walks in.");
}

#[test]
fn test_screenplay_add_element() {
    let mut screenplay = Screenplay::empty();
    let element = Element::new(ElementType::Action, "Test".to_string());

    screenplay.add_element(element);
    assert_eq!(screenplay.element_count(), 1);
    assert!(screenplay.modified);
}

#[test]
fn test_screenplay_insert_remove() {
    let mut screenplay = Screenplay::empty();
    let elem1 = Element::new(ElementType::Action, "First".to_string());
    let elem2 = Element::new(ElementType::Action, "Second".to_string());

    screenplay.add_element(elem1);
    screenplay.insert_element(0, elem2);

    assert_eq!(screenplay.element_count(), 2);
    assert_eq!(screenplay.get_element(0).unwrap().content, "Second");

    let removed = screenplay.remove_element(0);
    assert!(removed.is_some());
    assert_eq!(screenplay.element_count(), 1);
}

#[test]
fn test_json_serialization() {
    let mut screenplay = Screenplay::new("Test Script".to_string());
    screenplay.author = Some("Test Author".to_string());
    screenplay.add_element(Element::new(
        ElementType::SceneHeading,
        "INT. TEST - DAY".to_string(),
    ));

    let json = screenplay.to_json().unwrap();
    let deserialized = Screenplay::from_json(&json).unwrap();

    assert_eq!(deserialized.title, "Test Script");
    assert_eq!(deserialized.author, Some("Test Author".to_string()));
    assert_eq!(deserialized.element_count(), 1);
}