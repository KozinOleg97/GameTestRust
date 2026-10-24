# A panel declares whether it intercepts the pointer

Hover and hex selection are computed with our own ray rather than engine picking, so Bevy does not know that the world must not be touched under a UI panel. So that this rule does not scatter across every consumer, the property is set on the panel itself — `PointerOcclusion::{Opaque, Transparent}` — and a single resource `PointerBlocked` answers the question "is the cursor over an opaque panel?". Both hex selection and camera dragging query it.

Interception is recomputed every frame from the picking hit map, walking the ancestors of the hit entity. The state is deliberately not accumulated: that way a removed or hidden panel will not leave the pointer stuck. Walking ancestors is mandatory because a hit almost always lands on a child node — text, button, progress bar — while the property is set on the panel root.

The camera decides the gesture once, at the moment of press, and holds it thereafter: a drag started over a panel does not start at all, and one started in the world continues even if the cursor moves over a panel.

## Consequences

Adding a panel means choosing `Opaque` or `Transparent` at spawn; nothing else needs to be wired anywhere. There is no automatic connection with engine picking and there will not be until the world moves to mesh picking: if that happens, `PointerBlocked` will have to be reconciled with picking, otherwise the interface will start to double.

The 4-pixel click threshold is a consequence of another feature: the left button simultaneously pans the camera and selects a hex, so selection is set on release and only if the mouse has not moved.