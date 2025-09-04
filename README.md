# Bevy Template

This project is intended to serve as a structural template for Bevy projects. It is by no means a strict template, and
should be modified to suit your needs. At the very least it should offer some insight into how to structure a Bevy
project — at least in the way I like it.

## Structure
```yaml
src: # Where src files go
  components: # Contains agnostic, commonly used components - like a health bar
  systems: # Contains global systems that are agnostic to entities - e.g. take damage, gravity, etc...
  entities: # Contains entity-specific components, systems and plugins - such as a player
  events: # Contains event definitions, e.g. a damage event linking an entity and an amount of damage
```

