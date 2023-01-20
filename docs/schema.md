## Schema

```rust

struct Object {
    fields: Vec<Field>
}

struct Field {
    name: String,
    kind: Type
}

enum Primitive {
    String,
    Number,
    Bool,
}

struct Enum {
    variants: Vec<EnumVariant>
}

struct EnumVariant {
    name: String,
}

enum Type {
    Primitive(Primitive),
    Object(Object),
    List(Type),
    Map(Type),
    Union(Vec<Type>),
    Enum(Enum),
    Optional(Type)
}

```
