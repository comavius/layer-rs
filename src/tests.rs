use super::*;
use crate::prelude::*;

#[test]
fn test_get() {
    let node: Node<u32, Node<i32, Node<bool, (), ()>, ()>, Node<f32, (), ()>> = Node {
        requirement: 10u32,
        prev_left: Node {
            requirement: -5i32,
            prev_left: Node {
                requirement: true,
                prev_left: (),
                prev_right: (),
            },
            prev_right: (),
        },
        prev_right: Node {
            requirement: 3.14f32,
            prev_left: (),
            prev_right: (),
        },
    };
    let layer = Layer::dangerously_new_from_node(node);
    assert_eq!(*layer.get::<u32, _>(), 10u32);
    assert_eq!(*layer.get::<i32, _>(), -5i32);
    assert_eq!(*layer.get::<bool, _>(), true);
    assert_eq!(*layer.get::<f32, _>(), 3.14f32);
}

#[test]
fn test_insert() {
    let layer = Layer::new(10u32)
        .insert(-5i32)
        .insert(3.14f32)
        .insert(true)
        .insert(Some(10u32))
        .insert(Some(-5i32))
        .insert(Some(3.14f32))
        .insert(Some(true));
    assert_eq!(*layer.get::<u32, _>(), 10u32);
    assert_eq!(*layer.get::<i32, _>(), -5i32);
    assert_eq!(*layer.get::<f32, _>(), 3.14f32);
    assert_eq!(*layer.get::<bool, _>(), true);
    assert_eq!(*layer.get::<Option<u32>, _>(), Some(10u32));
    assert_eq!(*layer.get::<Option<i32>, _>(), Some(-5i32));
    assert_eq!(*layer.get::<Option<f32>, _>(), Some(3.14f32));
    assert_eq!(*layer.get::<Option<bool>, _>(), Some(true));
}

#[test]
fn test_replace() {
    let mut layer = Layer::new(10u32).insert(-5i32).insert(3.14f32).insert(true);
    assert_eq!(*layer.get::<i32, _>(), -5i32);
    let old_value = layer.replace::<i32, _>(42i32);
    assert_eq!(old_value, -5i32);
    assert_eq!(*layer.get::<i32, _>(), 42i32);
}
