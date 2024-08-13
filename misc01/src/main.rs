#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
struct CopiableEntity {
    id: i32,
    age: i32,
}

fn main() {
    let entity1 = CopiableEntity {
        id: 1,
        age: 10
    };
    println!("{:?}", entity1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_equal() {
        let entity1 = CopiableEntity {
            id: 1,
            age: 10
        };
        let entity2 = entity1;
        assert_eq!(entity1.id, entity2.id);
        assert_eq!(entity1.age, entity2.age);
    }

    #[test]
    fn change_copy_and_leave_the_original() {
        let entity = CopiableEntity {
            id: 1,
            age: 10
        };
        let changed_entity = change_copied_entity(entity);

        assert_eq!(entity.id, 1);
        assert_eq!(entity.age, 10);
        assert_ne!(entity.age, changed_entity.age);
        assert_eq!(changed_entity.age, 20);
    }

    fn change_copied_entity(mut src: CopiableEntity) -> CopiableEntity {
        src.age = 20;

        src
    }
}
