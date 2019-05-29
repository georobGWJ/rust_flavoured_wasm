# WARoS - The Web Assemply Robot System

An homage to Crobots, a C-driven API that allows creation and combat of virtual bots.

This is the Final project of [Programming Web Assembly with Rust](https://pragprog.com/book/khrust/programming-webassembly-with-rust)

---

Note that creating a workspace in the root folder (`waros`) will cause all builds to be put into a target directory in root and allow for tests across all subjects to be run from the root directory.

---

#### Entity-Component-System (ECS)
A way to separate out read concerns from write concerns when dealing with an array of structs. It heavily emphasizes Composition over Inheritance.

**Entitiy** - Entities are arbitrary things. In most ECS implementations, an entity is just a UID. For `waros`, the entity name is the WASM module name (which is the same as the player's name).

**Component** - A Component represents the state for a particular aspect of an entity. In our case these will be things like motion, damage, projectile, and scanner components.

**System** - Systems perform logic and take action globally against components within them. In the `waros` engine, there is a System responsible for each component. Most ECS implementations run Systems in background threads; the `waros` engine invoke them sequentially.