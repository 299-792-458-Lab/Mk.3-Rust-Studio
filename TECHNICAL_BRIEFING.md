Project: Mk.03-Rust-Studio  
World Name: TERA  
Document Type: Technical Briefing & Detailed Specification  
Version: 1.0  
Author: Engineering Division  
Date: 2025-10-13

---

## 1. Project Overview

Mk.03-Rust-Studio is an autonomous backend simulation engine built entirely in Rust, designed to emulate a living, evolving world named TERA. It operates without direct user control—instead, the system runs a persistent, event-driven world where hundreds of AI entities interact dynamically through exploration, trade, combat, and survival.

This project aims to demonstrate Rust’s strengths in memory safety, concurrency, and deterministic simulation within a self-regulating ecosystem architecture.

---

## 2. Core Concept

“A world that lives without players.”

TERA is not a game—it is a simulation of existence. Each entity (NPC) has an autonomous behavior cycle governed by probabilistic AI, operating under ECS-based world logic. All changes in the world are recorded as chronological logs, producing a real-time generative narrative of a self-sustaining civilization.

---

## 3. System Architecture

### High-Level Components

```
[Client / Observer]
        ↓
[Axum REST / Tonic gRPC API]
        ↓
[Simulation Core (ECS Engine)]
├── Entity System
│   ├─ NPC, Item, Resource, Region
│   └─ Components (Health, Position, Inventory, Trait)
├── Behavior System (FSM-based AI)
├── Economy & Combat Systems
├── World State Store (Arc<RwLock>)
└── Event Logger / Observer
```

---

## 4. Technical Stack

- Language: Rust (stable 1.80+)
- Async Runtime: Tokio
- ECS Framework: Bevy ECS or Legion
- Web Layer: Axum (REST) / Tonic (gRPC)
- Serialization: Serde, Bincode
- Logging / Metrics: Tracing, Prometheus Exporter
- Testing: Criterion, Proptest

---

## 5. Entity Model

Each entity in TERA is defined through the ECS model.

### Entity Examples

Merchant, Bandit, Farmer, Priest, Explorer, Hunter.

### Components

- Identity: unique id, name, faction
- Attributes: health, stamina, wealth, fame
- Position: (x, y) coordinate
- Inventory: items, resources, currency
- Personality: aggressive, cautious, social, curious
- Behavior: current state in FSM (Idle, Explore, Trade, Combat, Rest)

---

## 6. AI Behavior System

The AI follows a Finite State Machine with probabilistic transitions.

### States and Transitions

- Idle → Explore (time elapsed)
- Explore → Gather (resource discovery)
- Gather → Trade (inventory full)
- Trade → Hunt (low wealth)
- Hunt → Rest (low health)
- Rest → Idle (recovered)

Each NPC has a personality trait vector that modifies transition probabilities. Example: aggressive +10% chance to enter Hunt, cautious +15% chance to avoid Combat.

---

## 7. Simulation Loop

Simulation advances by discrete ticks. Each tick updates all ECS systems in sequence:

1. AI System (state transitions)
2. Movement and Combat System
3. Trade and Economy System
4. Event Generation
5. Logging and Metrics Export

Tick frequency: 1 tick per second (configurable). Concurrency is handled via Tokio tasks with shared state under `Arc<RwLock>`.

---

## 8. World Event Logging

Every tick generates structured world events which are aggregated as logs.

### Event Types

- `TradeEvent` (buyer, seller, item, amount)
- `CombatEvent` (attacker, defender, outcome)
- `DiscoveryEvent` (npc, resource)
- `DeathEvent` (npc, cause)
- `SocialEvent` (interaction, reputation change)

### Log Format Example

```
[Tick 2313] Merchant “Calix” traded herbs with Hunter “Rena”.
[Tick 2314] Bandit “Rena” attacked Merchant “Calix”.
[Tick 2315] Merchant “Calix” was killed in battle.
```

Logs are stored in both structured JSON and text form, exportable via REST and Prometheus.

---

## 9. API Specification

- `GET /world/state` – returns current world snapshot
- `GET /world/logs?from=n` – stream of world events
- `POST /admin/tick` – manually advance simulation
- `POST /npc/create` – spawn new entity
- `GET /npc/:id` – retrieve specific entity info

All API calls are stateless, exposing a read-only observer interface.

---

## 10. World: TERA

TERA is an autonomous continent-scale simulation stitched from five interlocking biomes: Forest, Plains, Desert, Village, and Market. Each biome is both a physical region and a social-economic membrane that regulates who travels, what resources flow, and how factions project influence.

### 10.1 Biome Profiles

- **Forest (The Verdant Veil)**  
  Dense canopies and hidden shrines. Produces herbs, timber, and rare fauna. Bandit enclaves and the Explorers’ League use the terrain for ambushes and discovery.
- **Plains (Silverwind Expanse)**  
  Open grasslands that enable caravans, large-scale farming, and mounted patrols. Farmers and hunter clans compete over migratory herds.
- **Desert (Ashen Mirage)**  
  Arid dunes masking ancient ruins. Mineral wealth and relics fuel high-risk expeditions by Explorers and opportunistic bandits. Survival hinges on water caches and nomadic guides.
- **Village (Hearthbound Circuit)**  
  Networked settlements with workshops, granaries, and communal halls. Primary hub for Farmers, Priests, and social reputation systems.
- **Market (Golden Conflux)**  
  Cosmopolitan trade nexus with auction floors and guild halls. Currency exchange rates, faction treaties, and economic shocks radiate outward from here.

### 10.2 Faction Landscape

- **Merchant Guild** maintains trade routes, price stabilization, and caravan security. Operates credit ledgers in Market and Plains.
- **Bandit Clans** practice asymmetrical warfare, raiding convoys, and hoarding desert relics. Reputation driven by infamy.
- **Explorers’ League** catalogs biomes, charts anomalies, and sells discovery rights. Strong curiosity modifiers and desert/forest specialization.
- **Settlers’ Union** spans Village and Plains, coordinating crop cycles and civil projects. Reputation tied to communal prosperity.
- **Temple of Suns** (Priest order) mediates social conflicts, curates relic lore, and offers health services across regions.

Each faction controls influence vectors (trade tariff, military strength, or cultural sway) that feed into economy and social systems.

### 10.3 Economy

- Currency cycle: Market auctions → Merchant caravans → Village services → Desert expeditions → returns to Market. Scarcity shocks pivot around Desert relic yields and Plains harvest volumes.
- Dynamic pricing: Resource scarcity modifies item prices using regional supply indices and faction tariff policies.
- Randomized macro-events: drought (Plains & Village), invasion (Bandit surge), market crash (currency debasement), pilgrimage boon (Temple-led festivals), relic rush (Explorers’ discoveries).

### 10.4 Social Dynamics

- Reputation tiers: `Unknown`, `Noted`, `Renowned`, `Legendary`, `Infamous`.
- Fame/Infamy cross-influences faction interactions; e.g., an Infamous bandit raises alarm levels in Market, increasing guard patrol frequency.
- Social events include barter conclaves, peace councils, vendetta declarations, and pilgrimage processions. Outcomes shift faction alignments and local stability.

### 10.5 Temporal Cadence

- **Tick Epochs**: Daybreak (resource refresh), Highsun (trade volume peak), Nightfall (bandit aggression surge).
- **Seasonal Modifiers**: three-season cycle (Bloomrise, Highflame, Emberfall) adjusts biome productivity and event likelihood.

The simulation will encode these layers as structured metadata to drive systemic behaviors, ensuring world cohesion and emergent narrative density.

---

## 11. Development Roadmap

- **Phase 1 – Core ECS Simulation (1 month)**  
  Entity creation, FSM behavior, tick loop.
- **Phase 2 – World Economy & Logging System (1 month)**  
  Trade, resource generation, world log feed.
- **Phase 3 – API & Observer Interface (2 weeks)**  
  REST + gRPC endpoints, real-time log streaming.
- **Phase 4 – Metrics, Scaling, and Persistence (2 weeks)**  
  Prometheus metrics, database persistence, profiling.
- **Phase 5 – Optional Extensions**  
  Distributed multi-region simulation, AI learning adaptation (reinforcement tuning), external dashboard (CLI or web visualization).

---

## 12. Expected Deliverables

- Executable binary (`mk3_rust_studio`)
- Configuration YAML (`world.json` / `settings.toml`)
- REST/gRPC API server
- Log archive (chronological text + JSON)
- Technical documentation & architecture diagram
- Benchmark report (tick latency, entity throughput)

---

## 13. Objectives and Impact

Mk.03-Rust-Studio: TERA demonstrates autonomous system design, concurrency control, and emergent simulation using Rust. It bridges engineering precision with narrative generation, providing a dynamic proof of concept for self-regulating backend systems capable of producing a living data world.
