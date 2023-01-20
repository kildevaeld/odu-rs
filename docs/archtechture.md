# Architechture

### Orginazation

### User

### Group

### Tag

### Project

### Resource

A resource is an _instance_ of a resource type.

```ts
interface Resource {
  name: string;
  content: Value;
}
```

#### Resource Type

A resource type is a blueprint of how a resource looks like.
it contains a **name** and a description of the data structure descripted in data [description](schema.md)

```ts
interface ResourceType {
  name: string;
  description: string;
  blueprint: Type;
}
```
