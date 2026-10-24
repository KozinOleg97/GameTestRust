# UI asks the game via messages, not resources

The interface does not touch game state directly. A button carries `UiAction`, a single observer `on_ui_activate` translates it into a domain request — `MenuRequest` for the menu, `SimulationRequest` for simulation — and the request is handled by a system of the module that owns it. Request types are declared in the domain, so the interface depends on the game, while the game knows nothing about the interface.

Messages rather than direct calls were chosen also because the source of a request may be more than a button: a hotkey, a script, or a network request will one day ask to start the game or change speed, and the domain will not care who exactly asked.

## Consequences

- `UiAction` remains the interface's dictionary and consists of sub-enums by owner — `MenuAction`, `SimulationAction` — so screen dictionaries grow independently: a new panel does not touch other branches.
- The observer declares only message writers, not domain resources: a new action adds a branch, not a parameter in the signature.
- Messages are not suitable for interactions with a specific game entity: there the intent is addressed to the entity and the response comes back there. The rule — addressed interactions go as events or as an intent component on the entity itself; the decision on them is made together with the first such task (objects on hexes), not in advance.
- Hex selection by click did not get into the interface at all: it is an interaction with the world, so it lives in the domain — `hex::HoveredHex` and `hex::SelectedHex` — and the panel only reads the result. The same principle will be applied going forward: if an action has no button, it is not an interface action.