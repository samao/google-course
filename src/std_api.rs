use tracing::info;

pub fn std_api_run() {
    println!("std_api_run");

    let key = Key {
        id: 1,
        metadata: None,
    };

    if key.eq(&Key {
        id: 1,
        metadata: Some("".to_owned()),
    }) {
        info!("key is equal")
    }

    let key2 = Key {
        id: 2,
        metadata: Some("".into()),
    };

    if key < key2 {
        info!("key {:?} is less than key2 {:?}", key, key2)
    }
}

#[derive(Debug)]
struct Key {
    id: u32,
    metadata: Option<String>,
}

impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}

impl PartialOrd for Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.id.partial_cmp(&other.id) {
            Some(res) => Some(res),
            None => None,
        }
    }
}

#[test]
fn test_key_eq() {
    let key = Key {
        id: 1,
        metadata: None,
    };
    assert_eq!(
        key,
        Key {
            id: 1,
            metadata: Some("".to_owned())
        }
    );
    assert_ne!(
        key,
        Key {
            id: 2,
            metadata: None
        }
    );
}
#[test]
fn test_key_partial_cmp() {
    let key = Key {
        id: 1,
        metadata: None,
    };
    assert_eq!(
        key.partial_cmp(&Key {
            id: 1,
            metadata: Some("".to_owned())
        }),
        Some(std::cmp::Ordering::Equal)
    );
    assert_eq!(
        key.partial_cmp(&Key {
            id: 2,
            metadata: None
        }),
        Some(std::cmp::Ordering::Less)
    );
    assert_eq!(
        key.partial_cmp(&Key {
            id: 0,
            metadata: None
        }),
        Some(std::cmp::Ordering::Greater)
    );
}
