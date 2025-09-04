# Bevy Template

This project is intended to serve as a structural template for Bevy projects. It is by no means a strict template, and
should be modified to suit your needs. At the very least it should offer some insight into how to structure a Bevy
project — at least in the way I like it.

## Structure
```yaml
src: # Where src files go
  components: # Contains agnostic, commonly used components - like a health bar
    core: # Contains core component structs - singular, atomic
    bundles: # Contains bundles of atomic components that are used as collections, but still agnostic of entity.
  systems: # Contains global systems that are agnostic to entities - e.g. take damage, gravity, etc...
  entities: # Contains entity-specific components, systems and plugins - such as a player
    player: # Contains components and systems unique to the player, as well as plugin
  events: # Contains event definitions, e.g. a damage event linking an entity and an amount of damage
```

## Components
- Each component is defined within the `components/core`. It should atomic - otherwise it most likely warrants a bundle.
- Collections of components are created in `components/bundles`. 
- All components and bundles in `components/` should be completely entity agnostic.

## Entities
- Each entity declares a new module by creating an `entity.rs` file that declares the entity's components, systems and plugins.
- Each entity defines its own unique components and systems. 
- Each entity has a plugin that bundles its systems (spawning, etc...)
- This plugin is then added to the `entities/plugin.rs` file.

## Events
- Each event is defined in `events/`.

## Systems
- Each system is defined in `systems/`.
- Systems in this directory are agnostic to entities, i.e. they are global.