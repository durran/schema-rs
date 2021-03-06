#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

extern crate schema;
use schema::Type;
use schema::Field;

describe! field_test {
    describe! new {
        before_each {
            let mut types = Vec::new();
            types.push(Type::new("Decimal128".to_string(), 5, 0.75, 26));
            let field = Field::new("testing".to_string(), 5, 0.5, true, types);
        }

        it "exposes a name property" {
            assert_eq!("testing", field.name);
        }

        it "exposes a count property" {
            assert_eq!(5, field.count);
        }

        it "exposes a probability property" {
            assert_eq!(0.5, field.probability);
        }

        it "exposes a has_duplicates property" {
            assert_eq!(true, field.has_duplicates);
        }
    }
}
