use crate::IRComponent;
use alloc::string::String;

#[allow(unused)]
pub trait Value: IRComponent {}

pub struct Values;

impl Values {
    pub(crate) fn integer(value: &str) -> Option<Integer>
    where
        Self: Sized,
    {
        for char in value.chars() {
            if !char.is_digit(10) {
                return None;
            }
        }
        Some(Integer {
            value: String::from(value),
        })
    }

    pub(crate) fn global(name: &str) -> GlobalIdentifier
    where
        Self: Sized,
    {
        GlobalIdentifier {
            name: String::from(name),
        }
    }

    fn local(name: &str) -> LocalIdentifier
    where
        Self: Sized,
    {
        LocalIdentifier {
            name: String::from(name),
        }
    }
}

pub struct GlobalIdentifier {
    name: String,
}

impl IRComponent for GlobalIdentifier {
    fn append_to_string(&self, string: &mut String) {
        string.push('@');
        string.push_str(&self.name);
    }
}
impl Value for GlobalIdentifier {}

pub struct LocalIdentifier {
    pub name: String,
}

impl IRComponent for LocalIdentifier {
    fn append_to_string(&self, string: &mut String) {
        string.push('%');
        string.push_str(&self.name);
    }
}
impl Value for LocalIdentifier {}

pub trait Constant: Value {}

pub struct Integer {
    value: String,
}

impl IRComponent for Integer {
    fn append_to_string(&self, string: &mut String) {
        string.push_str(&self.value);
    }
}

impl Constant for Integer {}

impl Value for Integer {}

#[cfg(test)]
mod tests {
    use crate::value::Values;
    use crate::IRComponent;

    #[test]
    pub fn test_local_idents() {
        let value = Values::local("foo");
        assert_eq!(value.emit(), "%foo");
    }
    #[test]
    pub fn test_global_idents() {
        let value = Values::global("bar");
        assert_eq!(value.emit(), "@bar");
    }
    #[test]
    pub fn test_int_constants() {
        let value = Values::integer("1256").unwrap();
        assert_eq!(value.emit(), "1256");
    }
}
